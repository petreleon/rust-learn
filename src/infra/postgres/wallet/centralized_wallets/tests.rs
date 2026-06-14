use super::*;

#[test]
fn parses_user_variants() {
    assert_eq!(OwnerType::parse("user").unwrap(), OwnerType::User);
    assert_eq!(OwnerType::parse("users").unwrap(), OwnerType::User);
    assert_eq!(OwnerType::parse("USER").unwrap(), OwnerType::User);
}

#[test]
fn parses_organization_variants() {
    assert_eq!(
        OwnerType::parse("organization").unwrap(),
        OwnerType::Organization
    );
    assert_eq!(OwnerType::parse("org").unwrap(), OwnerType::Organization);
    assert_eq!(
        OwnerType::parse("organizations").unwrap(),
        OwnerType::Organization
    );
    assert_eq!(OwnerType::parse("ORG").unwrap(), OwnerType::Organization);
}

#[test]
fn rejects_unknown_owner_type() {
    assert!(OwnerType::parse("").is_err());
    assert!(OwnerType::parse("admin").is_err());
    assert!(OwnerType::parse("wallet").is_err());
}
