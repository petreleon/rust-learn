#[actix_web::test]
async fn schedule_retry_sets_queued_state_attempts_error_and_future_availability() {
    let _guard = lock_worker_upload_job_tests().await;
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let job = insert_upload_job(&mut conn, &unique_object("retry")).await;

    diesel::update(upload_jobs::table.find(job.id()))
        .set(upload_jobs::status.eq("processing"))
        .execute(&mut conn)
        .await
        .expect("test should mark job processing");

    let retry_at = Utc::now() + Duration::minutes(5);
    upload_job_queue::schedule_retry(
        job.id(),
        1,
        "transient ffmpeg failure".to_string(),
        retry_at,
        &mut conn,
    )
    .await
    .expect("schedule_retry should succeed");

    let updated = fetch_upload_job(&mut conn, job.id()).await;
    assert_eq!(updated.status, "queued");
    assert_eq!(updated.attempts, 1);
    assert_eq!(
        updated.last_error.as_deref(),
        Some("transient ffmpeg failure")
    );
    let updated_at = updated.updated_at.expect("retry should set updated_at");
    assert!(updated_at > Utc::now());
    assert!(updated_at <= retry_at + Duration::seconds(1));

    upload_job_queue::mark_done(job.id(), &mut conn)
        .await
        .expect("test should clean up retry job");
}

#[actix_web::test]
async fn mark_failed_sets_terminal_failure_state() {
    let _guard = lock_worker_upload_job_tests().await;
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let job = insert_upload_job(&mut conn, &unique_object("failed")).await;

    upload_job_queue::mark_failed(
        job.id(),
        5,
        "permanent processing failure".to_string(),
        &mut conn,
    )
    .await
    .expect("mark_failed should succeed");

    let updated = fetch_upload_job(&mut conn, job.id()).await;
    assert_eq!(updated.status, "failed");
    assert_eq!(updated.attempts, 5);
    assert_eq!(
        updated.last_error.as_deref(),
        Some("permanent processing failure")
    );
    assert!(updated.updated_at.is_some());

    upload_job_queue::mark_done(job.id(), &mut conn)
        .await
        .expect("test should clean up failed job");
}

#[actix_web::test]
async fn mark_done_sets_terminal_success_state_without_changing_attempts() {
    let _guard = lock_worker_upload_job_tests().await;
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let job = insert_upload_job(&mut conn, &unique_object("done")).await;

    diesel::update(upload_jobs::table.find(job.id()))
        .set(upload_jobs::attempts.eq(2))
        .execute(&mut conn)
        .await
        .expect("test should seed attempts");

    upload_job_queue::mark_done(job.id(), &mut conn)
        .await
        .expect("mark_done should succeed");

    let updated = fetch_upload_job(&mut conn, job.id()).await;
    assert_eq!(updated.status, "done");
    assert_eq!(updated.attempts, 2);
    assert!(updated.updated_at.is_some());
}
