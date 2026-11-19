use crate::git_publisher::sanitize_branch_slug;

#[test]
fn sanitizes_a_post_id_into_a_safe_branch_fragment() {
    assert_eq!(
        sanitize_branch_slug("Édito IA: Petits Modèles!"),
        "dito-ia-petits-mod-les"
    );
    assert_eq!(sanitize_branch_slug("simple-id-123"), "simple-id-123");
    assert_eq!(
        sanitize_branch_slug("--leading-and-trailing--"),
        "leading-and-trailing"
    );
}
