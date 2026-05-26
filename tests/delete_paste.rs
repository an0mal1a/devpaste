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
fn delete_public_paste_without_password_removes_it() {
    let _db = common::TestDb::new();

    let (id, _) = utils::create_paste(paste("delete me", "temporary", true, ""))
        .expect("public paste should be created");

    let deleted_id = utils::remove_paste(id, None).expect("public paste should be deleted");

    assert_eq!(deleted_id, id);
    assert_eq!(
        utils::read_paste(id).expect_err("deleted paste should not be readable"),
        "Paste not found"
    );
}

#[test]
fn delete_missing_paste_returns_error() {
    let _db = common::TestDb::new();

    let err = utils::remove_paste(999_999, None).expect_err("missing paste should fail");

    assert_eq!(err, "Paste dont exist.");
}

#[test]
fn delete_protected_paste_requires_correct_password() {
    let _db = common::TestDb::new();

    let (id, slug) = utils::create_paste(paste("protected delete", "temporary", true, "secret"))
        .expect("protected paste should be created");
    let slug = slug.expect("protected paste should get slug");

    assert_eq!(
        utils::remove_paste(id, None).expect_err("missing password should fail"),
        "Password Incorrect"
    );
    assert_eq!(
        utils::remove_paste(id, Some("wrong".to_string()))
            .expect_err("wrong password should fail"),
        "Password Incorrect"
    );

    let deleted_id = utils::remove_paste(id, Some("secret".to_string()))
        .expect("correct password should delete paste");

    assert_eq!(deleted_id, id);
    assert!(
        utils::read_paste_slug(slug, Some("secret".to_string())).is_err(),
        "deleted protected paste should not be readable by slug"
    );
}
