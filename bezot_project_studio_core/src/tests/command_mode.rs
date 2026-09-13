use crate::CommandMode;

#[test]
fn parses_modes() {
    assert_eq!(CommandMode::parse(Some("dev")).unwrap(), CommandMode::Dev);
    assert_eq!(
        CommandMode::parse(Some("production")).unwrap(),
        CommandMode::Production
    );
    assert!(CommandMode::parse(Some("preview")).is_err());
    assert!(CommandMode::parse(None).is_err());
}

#[test]
fn production_uses_release_assembler() {
    assert!(!CommandMode::Dev.use_release_assembler());
    assert!(CommandMode::Production.use_release_assembler());
}
