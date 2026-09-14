use crate::project_root_argument;

#[test]
fn defaults_to_site_project_root() {
    let args = vec!["bezot_project_studio_ui".to_string()];

    assert_eq!(project_root_argument(&args).unwrap(), "site");
}

#[test]
fn accepts_explicit_project_root() {
    let args = vec!["bezot_project_studio_ui".to_string(), "site".to_string()];

    assert_eq!(project_root_argument(&args).unwrap(), "site");
}
