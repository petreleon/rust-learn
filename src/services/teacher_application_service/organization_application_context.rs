struct OrganizationApplicationContext {
    users: BTreeMap<i32, TeacherApplicationUserSummary>,
    organizations: BTreeMap<i32, String>,
    courses: BTreeMap<i32, String>,
    audits: BTreeMap<i64, TeacherApplicationAuditSummary>,
}
