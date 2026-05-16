mod help;

use cargo_test_support::git;
use cargo_test_support::prelude::*;
use cargo_test_support::registry::RegistryBuilder;
use snapbox::str;

use crate::ProjectExt as _;
use crate::git_commit;
use crate::git_head_id;
use crate::git_switch;

#[cargo_test]
fn correct_base() {
    let registry = RegistryBuilder::new().http_api().http_index().build();

    let (project, repo) = git::new_repo("foo", |project| {
        project
            .file(
                "Cargo.toml",
                r#"
                [package]
                name = "foo"
                version = "1.0.0"
                edition = "2015"
                authors = []
            "#,
            )
            .file("src/lib.rs", "pub fn initial() -> f64 { 1.0 }")
    });
    let initial_commit_id = git_head_id(&repo);
    project
        .cargo_global("publish")
        .replace_crates_io(registry.index_url())
        .run();

    project.change_file("src/lib.rs", "pub fn initial() -> f64 { 1.12 }");
    git::add(&repo);
    git_commit(&repo, "2");
    project.change_file("src/lib.rs", "pub fn initial() -> f64 { 1.13 }");
    git::add(&repo);
    git_commit(&repo, "3");

    project.change_file(
        "Cargo.toml",
        r#"
                [package]
                name = "foo"
                version = "1.1.0"
                edition = "2015"
                authors = []
            "#,
    );
    git::add(&repo);
    git_commit(&repo, "1.1.0");
    git::tag(&repo, "1.1.0");
    project
        .cargo_global("publish")
        .replace_crates_io(registry.index_url())
        .run();

    git_switch(&repo, initial_commit_id);

    project.change_file("src/lib.rs", "pub fn initial() -> f64 { 1.04 }");
    git::add(&repo);
    git_commit(&repo, "4");

    project.change_file("src/lib.rs", "pub fn initial() -> f64 { 1.05 }");
    git::add(&repo);
    git_commit(&repo, "5");

    project
        .cargo_ship("ship")
        .arg_line("changes")
        .replace_crates_io(registry.index_url())
        .with_stdout_data(str![[r#"
"#]])
        .with_stderr_data(str![[r#"
     Changes for `foo`

"#]])
        .run();
}

#[cargo_test]
fn package_selection() {
    let registry = RegistryBuilder::new().http_api().http_index().build();

    let (project, repo) = git::new_repo("foo", |project| {
        project
            .file(
                "Cargo.toml",
                r#"
                [workspace]
                members = ["foo", "bar"]
            "#,
            )
            .file(
                "foo/Cargo.toml",
                r#"
                [package]
                name = "foo"
                version = "1.0.0"
                edition = "2015"
                authors = []
            "#,
            )
            .file("foo/src/lib.rs", "pub fn initial() -> f64 { 1.0 }")
            .file(
                "bar/Cargo.toml",
                r#"
                [package]
                name = "bar"
                version = "1.0.0"
                edition = "2015"
                authors = []
            "#,
            )
            .file("bar/src/lib.rs", "pub fn initial() -> f64 { 2.0 }")
    });
    project
        .cargo_global("publish")
        .arg("--workspace")
        .replace_crates_io(registry.index_url())
        .run();

    project.change_file("foo/src/lib.rs", "pub fn initial() -> f64 { 1.1 }");
    git::add(&repo);
    git_commit(&repo, "foo 1");

    project.change_file("bar/src/lib.rs", "pub fn initial() -> f64 { 2.1 }");
    git::add(&repo);
    git_commit(&repo, "bar 1");

    project
        .cargo_ship("ship")
        .arg_line("changes -p foo")
        .replace_crates_io(registry.index_url())
        .with_stdout_data(str![[r#"
"#]])
        .with_stderr_data(str![[r#"
     Changes for `foo`
[NOTE] ignoring changes for `bar`

"#]])
        .run();
}

#[cargo_test]
fn publish_crates_io() {
    let registry = RegistryBuilder::new().http_api().http_index().build();

    let (project, repo) = git::new_repo("foo", |project| {
        project
            .file(
                "Cargo.toml",
                r#"
                [package]
                name = "foo"
                version = "1.0.0"
                edition = "2015"
                authors = []
                publish = ["crates-io"]
            "#,
            )
            .file("src/lib.rs", "pub fn initial() -> f64 { 1.0 }")
    });
    project
        .cargo_global("publish")
        .replace_crates_io(registry.index_url())
        .run();

    project.change_file("foo/src/lib.rs", "pub fn initial() -> f64 { 1.1 }");
    git::add(&repo);
    git_commit(&repo, "foo 1");

    project.change_file("bar/src/lib.rs", "pub fn initial() -> f64 { 2.1 }");
    git::add(&repo);
    git_commit(&repo, "bar 1");

    project
        .cargo_ship("ship")
        .arg_line("changes --workspace")
        .replace_crates_io(registry.index_url())
        .with_stdout_data(str![[r#"
"#]])
        .with_stderr_data(str![[r#"
     Changes for `foo`

"#]])
        .run();
}

#[cargo_test]
fn publish_none() {
    let registry = RegistryBuilder::new().http_api().http_index().build();

    let (project, repo) = git::new_repo("foo", |project| {
        project
            .file(
                "Cargo.toml",
                r#"
                [package]
                name = "foo"
                version = "1.0.0"
                edition = "2015"
                authors = []
                publish = false
            "#,
            )
            .file("src/lib.rs", "pub fn initial() -> f64 { 1.0 }")
    });

    project.change_file("foo/src/lib.rs", "pub fn initial() -> f64 { 1.1 }");
    git::add(&repo);
    git_commit(&repo, "foo 1");

    project.change_file("bar/src/lib.rs", "pub fn initial() -> f64 { 2.1 }");
    git::add(&repo);
    git_commit(&repo, "bar 1");

    project
        .cargo_ship("ship")
        .arg_line("changes --workspace")
        .replace_crates_io(registry.index_url())
        .with_stdout_data(str![[r#"
"#]])
        .with_stderr_data(str![[r#"
     Changes for `foo`

"#]])
        .run();
}

#[cargo_test]
fn publish_alt() {
    let registry = RegistryBuilder::new().http_api().http_index().build();
    let _alt_registry = RegistryBuilder::new()
        .http_api()
        .http_index()
        .alternative()
        .build();

    let (project, repo) = git::new_repo("foo", |project| {
        project
            .file(
                "Cargo.toml",
                r#"
                [package]
                name = "foo"
                version = "1.0.0"
                edition = "2015"
                authors = []
                publish = ["alternative"]
            "#,
            )
            .file("src/lib.rs", "pub fn initial() -> f64 { 1.0 }")
    });
    project.cargo_global("publish").run();

    project.change_file("foo/src/lib.rs", "pub fn initial() -> f64 { 1.1 }");
    git::add(&repo);
    git_commit(&repo, "foo 1");

    project.change_file("bar/src/lib.rs", "pub fn initial() -> f64 { 2.1 }");
    git::add(&repo);
    git_commit(&repo, "bar 1");

    project
        .cargo_ship("ship")
        .arg_line("changes --workspace")
        .replace_crates_io(registry.index_url())
        .with_stdout_data(str![[r#"
"#]])
        .with_stderr_data(str![[r#"
     Changes for `foo`

"#]])
        .run();
}
