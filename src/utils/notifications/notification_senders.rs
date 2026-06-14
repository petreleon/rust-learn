impl NotificationsState {
    /// Create a new NotificationsState from an existing DB pool.
    pub fn new(pool: DbPool) -> Self {
        NotificationsState { pool }
    }

    /// Send (add) a notification for a user.
    pub async fn send_notification(
        &self,
        user_id: i32,
        title: impl AsRef<str>,
        body: impl AsRef<str>,
    ) -> Result<i64> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        let new = NewNotification {
            user_id: Some(user_id),
            title: title.as_ref(),
            body: body.as_ref(),
        };
        let inserted_id = Notification::create(new, &mut conn).await?;
        Ok(inserted_id)
    }

    pub async fn send_event_notification(
        &self,
        user_id: i32,
        message: NotificationMessage,
    ) -> Result<i64> {
        self.send_notification(user_id, message.title, message.body)
            .await
    }

    pub async fn send_enrollment_notification(
        &self,
        user_id: i32,
        course_id: i32,
        course_title: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(user_id, enrollment_notification(course_id, course_title))
            .await
    }

    pub async fn send_content_published_notification(
        &self,
        user_id: i32,
        course_id: i32,
        content_id: i32,
        content_type: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            content_published_notification(course_id, content_id, content_type),
        )
        .await
    }

    pub async fn send_role_assignment_notification(
        &self,
        user_id: i32,
        scope: impl AsRef<str>,
        scope_id: Option<i32>,
        role_name: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            role_assignment_notification(scope, scope_id, role_name),
        )
        .await
    }

    pub async fn send_worker_failure_notification(
        &self,
        user_id: i32,
        job_id: i64,
        object: impl AsRef<str>,
        attempts: i32,
        error: impl AsRef<str>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            worker_failure_notification(job_id, object, attempts, error),
        )
        .await
    }

    pub async fn send_reward_event_notification(
        &self,
        user_id: i32,
        amount: impl AsRef<str>,
        event_type: impl AsRef<str>,
        transaction_id: Option<i64>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            reward_event_notification(amount, event_type, transaction_id),
        )
        .await
    }

    pub async fn send_reward_wallet_credit_notification(
        &self,
        user_id: i32,
        course_id: i32,
        course_title: impl AsRef<str>,
        amount: impl AsRef<str>,
        wallet_id: i32,
        transaction_id: i64,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            reward_wallet_credit_notification(
                course_id,
                course_title,
                amount,
                wallet_id,
                transaction_id,
            ),
        )
        .await
    }

    pub async fn send_teacher_application_notification(
        &self,
        user_id: i32,
        application_id: i64,
        event_type: impl AsRef<str>,
        status: impl AsRef<str>,
        requested_scope: impl AsRef<str>,
        reason: Option<impl AsRef<str>>,
    ) -> Result<i64> {
        self.send_event_notification(
            user_id,
            teacher_application_notification(
                application_id,
                event_type,
                status,
                requested_scope,
                reason,
            ),
        )
        .await
    }

    /// Get notifications for a user ordered by created_at desc.
    pub async fn get_notifications(&self, user_id: i32) -> Result<Vec<Notification>> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        let rows = Notification::find_by_user_id(user_id, NOTIFICATION_LIST_LIMIT, &mut conn).await?;
        Ok(rows)
    }
}
