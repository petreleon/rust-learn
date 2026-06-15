use crate::support::*;

pub(crate) async fn wait_for_running_worker(
    conn: &mut AsyncPgConnection,
    job_id: i64,
    timeout: StdDuration,
) -> UploadJob {
    let started = std::time::Instant::now();

    loop {
        let job = upload_jobs::table
            .find(job_id)
            .first::<UploadJob>(conn)
            .await
            .expect("upload job should remain queryable");

        match job.status.as_str() {
            "done" => return job,
            "failed" => panic!(
                "running worker failed upload job {} after {} attempts: {:?}",
                job.id(),
                job.attempts,
                job.last_error
            ),
            _ if started.elapsed() >= timeout => panic!(
                "timed out waiting for running worker to process job {}; last status: {}",
                job.id(),
                job.status
            ),
            _ => tokio::time::sleep(StdDuration::from_millis(500)).await,
        }
    }
}
