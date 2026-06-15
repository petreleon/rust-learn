use crate::application::teacher_applications::TeacherApplicationAuditEventOutput;
use crate::domain::teacher_applications::audit::TeacherApplicationAuditEventType;
use crate::models::teacher_application::TeacherApplicationAuditEvent;

pub(super) fn audit_event_output(
    event: TeacherApplicationAuditEvent,
) -> Result<TeacherApplicationAuditEventOutput, String> {
    let event_type = TeacherApplicationAuditEventType::parse(&event.event_type)
        .map_err(|error| error.to_string())?;
    Ok(TeacherApplicationAuditEventOutput {
        actor_user_id: event.actor_user_id,
        application_id: event.application_id,
        created_at: event.created_at,
        event_type,
        from_status: event.from_status,
        id: event.id,
        reason: event.reason,
        to_status: event.to_status,
    })
}
