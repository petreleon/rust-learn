pub async fn get_my_status(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<KycStatusResponse, KycError> {
    let user = User::find_by_id(user_id, conn).await?;
    let submission = KycSubmission::latest_for_user(conn, user_id).await?;
    Ok(KycStatusResponse {
        next_action: next_action(user.kyc_verified, submission.as_ref()),
        submission,
        user_kyc_verified: user.kyc_verified,
    })
}

pub async fn submit_my_kyc(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    request: SubmitKycRequest,
) -> Result<KycStatusResponse, KycError> {
    let user = User::find_by_id(user_id, conn).await?;
    if user.kyc_verified {
        return Err(KycError::InvalidTransition(
            "KYC is already verified for this account".to_string(),
        ));
    }
    let latest = KycSubmission::latest_for_user(conn, user_id).await?;
    if matches!(
        latest.as_ref().map(|s| s.status.as_str()),
        Some(KYC_STATUS_SUBMITTED | KYC_STATUS_UNDER_REVIEW)
    ) {
        return Err(KycError::InvalidTransition(
            "A KYC submission is already waiting for review".to_string(),
        ));
    }

    let submission = KycSubmission::create(conn, build_submission(user_id, request)?).await?;
    Ok(KycStatusResponse {
        next_action: "wait_for_review".to_string(),
        submission: Some(submission),
        user_kyc_verified: false,
    })
}

pub async fn list_review_queue(
    conn: &mut AsyncPgConnection,
    reviewer_user_id: i32,
) -> Result<KycReviewQueueResponse, KycError> {
    ensure_review_permission(conn, reviewer_user_id).await?;
    Ok(KycReviewQueueResponse {
        submissions: KycSubmission::list_review_queue(conn).await?,
    })
}

pub async fn decide_submission(
    conn: &mut AsyncPgConnection,
    reviewer_user_id: i32,
    submission_id: i64,
    request: KycDecisionRequest,
) -> Result<KycSubmission, KycError> {
    ensure_review_permission(conn, reviewer_user_id).await?;
    let existing = KycSubmission::find_by_id(conn, submission_id).await?;
    if !matches!(
        existing.status.as_str(),
        KYC_STATUS_SUBMITTED | KYC_STATUS_UNDER_REVIEW
    ) {
        return Err(KycError::InvalidTransition(
            "Only submitted or under-review KYC records can be decided".to_string(),
        ));
    }
    let (status, reason) = normalize_decision(request)?;
    let verified = status == KYC_STATUS_VERIFIED;

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let updated =
                KycSubmission::decide(conn, submission_id, reviewer_user_id, &status, reason)
                    .await?;
            diesel::update(users::table.find(updated.user_id))
                .set(users::kyc_verified.eq(verified))
                .execute(conn)
                .await?;
            Ok(updated)
        })
    })
    .await
    .map_err(KycError::from)
}

async fn ensure_review_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<(), KycError> {
    let permission = Permissions::REVIEW_KYC_SUBMISSIONS.to_string();
    if user_permission_platform_request(conn, user_id, &permission).await? {
        Ok(())
    } else {
        Err(KycError::PermissionDenied(permission))
    }
}
