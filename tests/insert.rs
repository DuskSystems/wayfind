#![expect(missing_docs, reason = "Tests")]

use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::RouterBuilder;
use wayfind::errors::{InsertError, TemplateError};

#[test]
fn insert_conflict_static() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/test", 1)?;

    let error = builder.insert("/test", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            new: "/test".to_owned(),
            existing: "/test".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"`/test` conflicts with `/test`");

    let router = builder.build();
    insta::assert_snapshot!(router, @"/test");

    Ok(())
}

#[test]
fn insert_conflict_dynamic() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<id>", 1)?;

    let error = builder.insert("/<id>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            new: "/<id>".to_owned(),
            existing: "/<id>".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"`/<id>` conflicts with `/<id>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn insert_conflict_dynamic_structural() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<id>", 1)?;

    let error = builder.insert("/<user>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            new: "/<user>".to_owned(),
            existing: "/<id>".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"`/<user>` conflicts with `/<id>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn insert_conflict_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/edit", 1)?;

    let error = builder.insert("/<*path>/edit", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            new: "/<*path>/edit".to_owned(),
            existing: "/<*path>/edit".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"`/<*path>/edit` conflicts with `/<*path>/edit`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /edit
    ");

    Ok(())
}

#[test]
fn insert_conflict_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/edit", 1)?;

    let error = builder.insert("/<*slug>/edit", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            new: "/<*slug>/edit".to_owned(),
            existing: "/<*path>/edit".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"`/<*slug>/edit` conflicts with `/<*path>/edit`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /edit
    ");

    Ok(())
}

#[test]
fn insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*catch_all>", 1)?;

    let error = builder.insert("/<*catch_all>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            new: "/<*catch_all>".to_owned(),
            existing: "/<*catch_all>".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"`/<*catch_all>` conflicts with `/<*catch_all>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_conflict_end_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*catch_all>", 1)?;

    let error = builder.insert("/<*files>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            new: "/<*files>".to_owned(),
            existing: "/<*catch_all>".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"`/<*files>` conflicts with `/<*catch_all>`");

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_duplicate_parameter() {
    let mut builder = RouterBuilder::new();
    let error = builder.insert("/<*id>/users/<id>", 3).unwrap_err();
    assert_eq!(
        error,
        InsertError::Template {
            template: "/<*id>/users/<id>".to_owned(),
            error: TemplateError::DuplicateParameter {
                name: "id".to_owned(),
            },
        }
    );

    insta::assert_snapshot!(error, @"invalid template `/<*id>/users/<id>`: duplicate parameter name `id`");

    let router = builder.build();
    insta::assert_snapshot!(router, @"");
}
