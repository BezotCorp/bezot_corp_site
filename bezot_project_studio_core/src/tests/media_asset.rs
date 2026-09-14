use std::fs;

use crate::media_asset::{list_media_assets, media_dir, upload_media_asset};
use crate::tests::support::temporary_project_root;

#[test]
fn lists_no_assets_when_the_media_directory_does_not_exist() {
    let project_root = temporary_project_root("media_asset_test");

    let assets = list_media_assets(&project_root).unwrap();

    assert!(assets.is_empty());
}

#[test]
fn uploads_an_image_and_sanitizes_its_name() {
    let project_root = temporary_project_root("media_asset_test");
    fs::create_dir_all(&project_root).unwrap();
    let source_path = project_root.join("My Photo!! (Final).PNG");
    fs::write(&source_path, b"fake-png-bytes").unwrap();

    let asset = upload_media_asset(&project_root, &source_path).unwrap();

    assert_eq!(asset.name, "my-photo-final.png");
    assert_eq!(asset.public_path, "/media/my-photo-final.png");
    assert_eq!(asset.size_bytes, "fake-png-bytes".len() as u64);
    assert!(media_dir(&project_root).join("my-photo-final.png").exists());

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn avoids_overwriting_a_colliding_name() {
    let project_root = temporary_project_root("media_asset_test");
    fs::create_dir_all(&project_root).unwrap();
    let first_source = project_root.join("cover.jpg");
    fs::write(&first_source, b"first").unwrap();
    let second_source = project_root.join("other/cover.jpg");
    fs::create_dir_all(second_source.parent().unwrap()).unwrap();
    fs::write(&second_source, b"second").unwrap();

    let first = upload_media_asset(&project_root, &first_source).unwrap();
    let second = upload_media_asset(&project_root, &second_source).unwrap();

    assert_eq!(first.name, "cover.jpg");
    assert_eq!(second.name, "cover-1.jpg");
    assert_eq!(
        fs::read(media_dir(&project_root).join("cover.jpg")).unwrap(),
        b"first"
    );
    assert_eq!(
        fs::read(media_dir(&project_root).join("cover-1.jpg")).unwrap(),
        b"second"
    );

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn rejects_a_non_image_extension() {
    let project_root = temporary_project_root("media_asset_test");
    fs::create_dir_all(&project_root).unwrap();
    let source_path = project_root.join("notes.txt");
    fs::write(&source_path, b"hello").unwrap();

    assert!(upload_media_asset(&project_root, &source_path).is_err());

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn lists_uploaded_assets_sorted_by_name() {
    let project_root = temporary_project_root("media_asset_test");
    fs::create_dir_all(&project_root).unwrap();
    let zebra_source = project_root.join("zebra.png");
    fs::write(&zebra_source, b"z").unwrap();
    let apple_source = project_root.join("apple.png");
    fs::write(&apple_source, b"a").unwrap();
    upload_media_asset(&project_root, &zebra_source).unwrap();
    upload_media_asset(&project_root, &apple_source).unwrap();

    let assets = list_media_assets(&project_root).unwrap();

    assert_eq!(
        assets
            .iter()
            .map(|asset| asset.name.clone())
            .collect::<Vec<_>>(),
        vec!["apple.png".to_string(), "zebra.png".to_string()]
    );

    fs::remove_dir_all(project_root).unwrap();
}
