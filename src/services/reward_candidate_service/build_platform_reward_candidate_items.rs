async fn build_platform_reward_candidate_items(
    conn: &mut AsyncPgConnection,
    candidates: &[RewardCandidate],
) -> Result<Vec<PlatformRewardCandidateItem>, RewardCandidateError> {
    if candidates.is_empty() {
        return Ok(vec![]);
    }

    let user_ids: Vec<i32> = candidates
        .iter()
        .flat_map(|c| {
            let mut ids = vec![c.student_user_id, c.submitter_user_id];
            if let Some(tid) = c.teacher_approver_user_id {
                ids.push(tid);
            }
            ids
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let course_ids: Vec<i32> = candidates
        .iter()
        .map(|c| c.course_id)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let users = users::table
        .filter(users::id.eq_any(&user_ids))
        .select((users::id, users::name, users::email))
        .load::<(i32, String, String)>(conn)
        .await
        .map_err(RewardCandidateError::from)?
        .into_iter()
        .map(|(id, name, email)| (id, PlatformRewardCandidateUserSummary { id, name, email }))
        .collect::<std::collections::HashMap<i32, PlatformRewardCandidateUserSummary>>();

    let courses = courses::table
        .filter(courses::id.eq_any(&course_ids))
        .select((courses::id, courses::title))
        .load::<(i32, String)>(conn)
        .await
        .map_err(RewardCandidateError::from)?
        .into_iter()
        .map(|(id, title)| (id, PlatformRewardCandidateCourseSummary { id, title }))
        .collect::<std::collections::HashMap<i32, PlatformRewardCandidateCourseSummary>>();

    let items = candidates
        .iter()
        .map(|c| PlatformRewardCandidateItem {
            id: c.id,
            student: users.get(&c.student_user_id).cloned().unwrap_or_else(|| {
                PlatformRewardCandidateUserSummary {
                    id: c.student_user_id,
                    name: format!("Student {}", c.student_user_id),
                    email: String::new(),
                }
            }),
            course: courses.get(&c.course_id).cloned().unwrap_or_else(|| {
                PlatformRewardCandidateCourseSummary {
                    id: c.course_id,
                    title: format!("Course {}", c.course_id),
                }
            }),
            event_type: c.event_type.clone(),
            status: c.status.clone(),
            teacher_approver: c
                .teacher_approver_user_id
                .and_then(|id| users.get(&id).cloned()),
            teacher_decision_reason: c.teacher_decision_reason.clone(),
            approved_amount: c.approved_amount.as_ref().map(|a| a.to_string()),
            submitter: users.get(&c.submitter_user_id).cloned().unwrap_or_else(|| {
                PlatformRewardCandidateUserSummary {
                    id: c.submitter_user_id,
                    name: format!("Submitter {}", c.submitter_user_id),
                    email: String::new(),
                }
            }),
            source_organization_id: c.source_organization_id,
            source_scope: c.source_scope.clone(),
            created_at: c.created_at,
            updated_at: c.updated_at,
        })
        .collect();

    Ok(items)
}

#[cfg(test)]
mod tests;
