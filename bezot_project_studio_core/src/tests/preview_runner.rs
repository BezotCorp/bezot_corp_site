use std::fs;

use crate::preview_runner::{build_preview_root, preview_route};
use crate::tests::support::temporary_project_root;

#[test]
fn computes_the_route_for_a_locale_and_slug() {
    assert_eq!(
        preview_route("fr-fr", "blog/mon-article"),
        "/fr-fr/blog/mon-article/"
    );
    assert_eq!(
        preview_route("en-us", "/blog/my-article/"),
        "/en-us/blog/my-article/"
    );
    assert_eq!(preview_route("fr-fr", ""), "/fr-fr/");
}

#[test]
fn copies_the_project_excluding_install_and_build_output() {
    // Must not start with "preview_": build_preview_root cleans up any
    // leftover directory whose name starts with its own temp-dir prefix,
    // and that prefix ends in "..._preview_" — a label starting with
    // "preview_" would make this source directory match it too.
    let project_root = temporary_project_root("source_for_preview_test");
    fs::create_dir_all(project_root.join("content")).unwrap();
    fs::write(project_root.join("content/index.json"), "{}").unwrap();
    fs::create_dir_all(project_root.join("node_modules/some-package")).unwrap();
    fs::write(project_root.join("node_modules/some-package/index.js"), "").unwrap();
    fs::create_dir_all(project_root.join("prebuild/dist")).unwrap();
    fs::write(project_root.join("prebuild/dist/index.html"), "").unwrap();

    let preview_root = build_preview_root(&project_root).unwrap();

    assert!(preview_root.join("content/index.json").exists());
    assert!(!preview_root.join("node_modules").exists());
    assert!(!preview_root.join("prebuild").exists());

    fs::remove_dir_all(project_root).unwrap();
    fs::remove_dir_all(preview_root).unwrap();
}
