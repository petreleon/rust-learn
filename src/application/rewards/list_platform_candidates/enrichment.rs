use std::collections::{HashMap, HashSet};

use crate::application::rewards::list_platform_candidates::store::PlatformRewardCandidateStore;
use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidateCourseSummary, PlatformRewardCandidateItem,
    PlatformRewardCandidateRecord, PlatformRewardCandidateUserSummary,
    PlatformRewardCandidatesError,
};

pub async fn enriched_candidates(
    store: &mut impl PlatformRewardCandidateStore,
    records: Vec<PlatformRewardCandidateRecord>,
) -> Result<Vec<PlatformRewardCandidateItem>, PlatformRewardCandidatesError> {
    if records.is_empty() {
        return Ok(Vec::new());
    }

    let users = store
        .load_user_summaries(candidate_user_ids(&records))
        .await?
        .into_iter()
        .map(|user| (user.id, user))
        .collect::<HashMap<_, _>>();
    let courses = store
        .load_course_summaries(candidate_course_ids(&records))
        .await?
        .into_iter()
        .map(|course| (course.id, course))
        .collect::<HashMap<_, _>>();

    Ok(records
        .into_iter()
        .map(|candidate| enriched_candidate(candidate, &users, &courses))
        .collect())
}

pub fn candidate_matches_search(candidate: &PlatformRewardCandidateItem, search: &str) -> bool {
    candidate.student.name.to_lowercase().contains(search)
        || candidate.student.email.to_lowercase().contains(search)
        || candidate.course.title.to_lowercase().contains(search)
        || format!("{}", candidate.id).contains(search)
}

fn enriched_candidate(
    candidate: PlatformRewardCandidateRecord,
    users: &HashMap<i32, PlatformRewardCandidateUserSummary>,
    courses: &HashMap<i32, PlatformRewardCandidateCourseSummary>,
) -> PlatformRewardCandidateItem {
    PlatformRewardCandidateItem {
        id: candidate.id,
        student: users
            .get(&candidate.student_user_id)
            .cloned()
            .unwrap_or_else(|| missing_user(candidate.student_user_id, "Student")),
        course: courses
            .get(&candidate.course_id)
            .cloned()
            .unwrap_or_else(|| missing_course(candidate.course_id)),
        event_type: candidate.event_type,
        status: candidate.status,
        teacher_approver: candidate
            .teacher_approver_user_id
            .and_then(|id| users.get(&id).cloned()),
        teacher_decision_reason: candidate.teacher_decision_reason,
        approved_amount: candidate.approved_amount,
        submitter: users
            .get(&candidate.submitter_user_id)
            .cloned()
            .unwrap_or_else(|| missing_user(candidate.submitter_user_id, "Submitter")),
        source_organization_id: candidate.source_organization_id,
        source_scope: candidate.source_scope,
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    }
}

fn candidate_user_ids(records: &[PlatformRewardCandidateRecord]) -> Vec<i32> {
    records
        .iter()
        .flat_map(|candidate| {
            [
                Some(candidate.student_user_id),
                Some(candidate.submitter_user_id),
                candidate.teacher_approver_user_id,
            ]
        })
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn candidate_course_ids(records: &[PlatformRewardCandidateRecord]) -> Vec<i32> {
    records
        .iter()
        .map(|candidate| candidate.course_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn missing_user(id: i32, label: &str) -> PlatformRewardCandidateUserSummary {
    PlatformRewardCandidateUserSummary {
        id,
        name: format!("{label} {id}"),
        email: String::new(),
    }
}

fn missing_course(id: i32) -> PlatformRewardCandidateCourseSummary {
    PlatformRewardCandidateCourseSummary {
        id,
        title: format!("Course {id}"),
    }
}
