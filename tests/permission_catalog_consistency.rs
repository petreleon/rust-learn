use rust_learn::config::constants::permissions::Permissions;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use strum::IntoEnumIterator;

const ROLE_NAMES: &[&str] = &[
    "ADMIN",
    "GUEST",
    "MODERATOR",
    "STUDENT",
    "SUPER_ADMIN",
    "TEACHER",
    "USER",
];

const RETIRED_PERMISSION_NAMES: &[&str] = &["MANAGE_MINIO_OBJECTS"];

fn enum_permissions() -> BTreeSet<String> {
    Permissions::iter()
        .map(|permission| permission.to_string())
        .collect()
}

fn documented_permissions() -> BTreeSet<String> {
    include_str!("../PERMISSIONS.md")
        .lines()
        .filter_map(|line| {
            let value = line.trim().strip_prefix('*')?.trim();
            is_permission_name(value).then(|| value.to_string())
        })
        .collect()
}

fn migration_permissions() -> BTreeSet<String> {
    let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let mut up_sql_paths = fs::read_dir(migrations_dir)
        .expect("migrations directory must be readable")
        .map(|entry| {
            entry
                .expect("migration directory entry")
                .path()
                .join("up.sql")
        })
        .filter(|path| path.exists())
        .collect::<Vec<_>>();

    up_sql_paths.sort();

    let mut permissions = BTreeSet::new();
    for path in up_sql_paths {
        let sql = fs::read_to_string(&path).expect("migration up.sql must be readable");
        for statement in sql.split(';') {
            if !statement.contains("role_permission_") {
                continue;
            }

            for literal in single_quoted_literals(statement) {
                if is_permission_name(&literal) {
                    permissions.insert(literal);
                }
            }
        }
    }

    permissions
}

fn single_quoted_literals(sql: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let mut chars = sql.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\'' {
            continue;
        }

        let mut literal = String::new();
        while let Some(value_ch) = chars.next() {
            if value_ch == '\'' {
                if chars.peek() == Some(&'\'') {
                    literal.push('\'');
                    chars.next();
                    continue;
                }
                break;
            }
            literal.push(value_ch);
        }

        literals.push(literal);
    }

    literals
}

fn is_permission_name(value: &str) -> bool {
    value.contains('_')
        && !ROLE_NAMES.contains(&value)
        && value
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_')
}

fn without_retired_permissions(permissions: &BTreeSet<String>) -> BTreeSet<String> {
    permissions
        .iter()
        .filter(|permission| !RETIRED_PERMISSION_NAMES.contains(&permission.as_str()))
        .cloned()
        .collect()
}

fn difference(left: &BTreeSet<String>, right: &BTreeSet<String>) -> BTreeSet<String> {
    left.difference(right).cloned().collect()
}

#[test]
fn permission_catalog_matches_migrations_and_documentation() {
    let enum_permissions = enum_permissions();
    let documented_permissions = documented_permissions();
    let migration_permissions = without_retired_permissions(&migration_permissions());

    let migration_missing_from_enum = difference(&migration_permissions, &enum_permissions);
    assert!(
        migration_missing_from_enum.is_empty(),
        "permissions referenced by role_permission migrations but missing from Permissions enum: {migration_missing_from_enum:?}"
    );

    let enum_missing_from_migrations = difference(&enum_permissions, &migration_permissions);
    assert!(
        enum_missing_from_migrations.is_empty(),
        "Permissions enum variants missing from role_permission migrations: {enum_missing_from_migrations:?}"
    );

    let docs_missing_from_enum = difference(&documented_permissions, &enum_permissions);
    assert!(
        docs_missing_from_enum.is_empty(),
        "permissions documented in PERMISSIONS.md but missing from Permissions enum: {docs_missing_from_enum:?}"
    );

    let enum_missing_from_docs = difference(&enum_permissions, &documented_permissions);
    assert!(
        enum_missing_from_docs.is_empty(),
        "Permissions enum variants missing from PERMISSIONS.md: {enum_missing_from_docs:?}"
    );
}
