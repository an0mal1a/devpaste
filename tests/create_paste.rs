mod common;

use devpaste::{modules::CreatePaste, utils};

fn paste(title: &str, content: &str, public: bool, password: &str) -> CreatePaste {
    CreatePaste {
        title: title.to_string(),
        content: content.to_string(),
        public,
        password: password.to_string(),
    }
}

#[test]
fn create_public_paste_returns_id_without_slug() {
    let _db = common::TestDb::new();

    let (id, slug) = utils::create_paste(paste("public title", "public content", true, ""))
        .expect("public paste should be created");

    assert!(id > 0);
    assert!(slug.is_none());

    let created = utils::read_paste(id).expect("public paste should be readable by id");
    assert_eq!(created.title, "public title");
    assert_eq!(created.content, "public content");
    assert!(created.public);
    assert!(!created.is_protected);
}

#[test]
fn create_unlisted_paste_returns_slug_and_hides_id_route() {
    let _db = common::TestDb::new();

    let (id, slug) = utils::create_paste(paste("unlisted title", "unlisted content", false, ""))
        .expect("unlisted paste should be created");
    let slug = slug.expect("unlisted paste should get a slug");

    assert!(id > 0);
    assert_eq!(
        utils::read_paste(id).expect_err("unlisted paste should not be readable by id"),
        "Paste not found"
    );

    let created =
        utils::read_paste_slug(slug, None).expect("unlisted paste should be readable by slug");
    assert_eq!(created.title, "unlisted title");
    assert!(!created.public);
    assert!(!created.is_protected);
}

#[test]
fn create_password_protected_paste_forces_unlisted_access() {
    let _db = common::TestDb::new();

    let (id, slug) = utils::create_paste(paste(
        "protected title",
        "protected content",
        true,
        "secret",
    ))
    .expect("protected paste should be created");
    let slug = slug.expect("protected paste should get a slug");

    assert!(id > 0);
    assert!(utils::read_paste(id).is_err());
    assert!(utils::read_paste_slug(slug.clone(), None).is_err());
    assert!(utils::read_paste_slug(slug.clone(), Some("wrong".to_string())).is_err());

    let created = utils::read_paste_slug(slug, Some("secret".to_string()))
        .expect("protected paste should be readable with correct password");
    assert_eq!(created.title, "protected title");
    assert!(!created.public);
    assert!(created.is_protected);
}

#[test]
fn create_duplicate_unlisted_paste_returns_unique_slug_error() {
    let _db = common::TestDb::new();

    utils::create_paste(paste("same title", "same content", false, ""))
        .expect("first unlisted paste should be created");

    let err = utils::create_paste(paste("same title", "same content", false, ""))
        .expect_err("duplicate slug should fail because slug is unique");

    assert!(err.contains("UNIQUE"));
}
