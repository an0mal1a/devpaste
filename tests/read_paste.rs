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
fn read_all_pastes_only_returns_public_unprotected_pastes() {
    let _db = common::TestDb::new();

    let (public_id, _) = utils::create_paste(paste("public", "visible", true, ""))
        .expect("public paste should be created");
    utils::create_paste(paste("unlisted", "hidden", false, ""))
        .expect("unlisted paste should be created");
    utils::create_paste(paste("protected", "hidden", true, "secret"))
        .expect("protected paste should be created");

    let pastes = utils::read_all_pastes().expect("public paste list should be readable");

    assert_eq!(pastes.len(), 1);
    assert_eq!(pastes[0].id, public_id);
    assert_eq!(pastes[0].title, "public");
    assert!(pastes[0].public);
    assert!(!pastes[0].is_protected);
}

#[test]
fn read_paste_returns_same_error_for_missing_and_unlisted_ids() {
    let _db = common::TestDb::new();

    let (unlisted_id, _) = utils::create_paste(paste("private", "hidden", false, ""))
        .expect("unlisted paste should be created");

    let missing_error = utils::read_paste(999_999).expect_err("missing paste should fail");
    let unlisted_error = utils::read_paste(unlisted_id).expect_err("unlisted paste should fail");

    assert_eq!(missing_error, "Paste not found");
    assert_eq!(unlisted_error, missing_error);
}

#[test]
fn read_paste_slug_returns_error_for_missing_slug() {
    let _db = common::TestDb::new();

    let err = utils::read_paste_slug("missing-slug".to_string(), None)
        .expect_err("missing slug should fail");

    assert!(err.contains("Query returned no rows"));
}
