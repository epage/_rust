mod changes;
mod help;

use cargo_test_support::{ArgLineCommandExt, Execs, Project, TestEnvCommandExt, execs, process, t};
use cargo_util::ProcessBuilder;

pub fn cargo_ship_exe() -> std::path::PathBuf {
    snapbox::cmd::cargo_bin!("cargo-ship").to_path_buf()
}

pub trait ProjectExt {
    /// Creates an `Execs` instance to run the `cargo-ship` binary
    fn cargo_ship(&self, cmd: &str) -> Execs;
    /// Creates an `Execs` instance to run the globally installed `cargo` command
    fn cargo_global(&self, cmd: &str) -> Execs;
}

impl ProjectExt for Project {
    fn cargo_ship(&self, cmd: &str) -> Execs {
        let cargo_ship = cargo_ship_exe();

        let mut p = process(&cargo_ship);
        p.cwd(self.root()).arg_line(cmd);

        execs().with_process_builder(p)
    }

    fn cargo_global(&self, cmd: &str) -> Execs {
        let cargo = std::env::var_os("CARGO").unwrap_or("cargo".into());

        let mut p = ProcessBuilder::new(cargo);
        p.test_env().cwd(self.root()).arg_line(cmd);

        execs().with_process_builder(p)
    }
}

pub trait CargoCommandExt {
    fn cargo_ui() -> Self;
}

impl CargoCommandExt for snapbox::cmd::Command {
    fn cargo_ui() -> Self {
        use cargo_test_support::TestEnvCommandExt;
        Self::new(cargo_ship_exe())
            .with_assert(cargo_test_support::compare::assert_ui())
            .env("CARGO_TERM_COLOR", "always")
            .env("CARGO_TERM_HYPERLINKS", "true")
            .test_env()
    }
}

pub fn git_commit(repo: &git2::Repository, message: &str) -> git2::Oid {
    let tree_id = t!(t!(repo.index()).write_tree());
    let sig = t!(repo.signature());
    let mut parents = Vec::new();
    if let Some(parent) = repo.head().ok().map(|h| h.target().unwrap()) {
        parents.push(t!(repo.find_commit(parent)))
    }
    let parents = parents.iter().collect::<Vec<_>>();
    t!(repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        message,
        &t!(repo.find_tree(tree_id)),
        &parents
    ))
}

pub fn git_head_id(repo: &git2::Repository) -> git2::Oid {
    repo.head()
        .unwrap_or_else(|e| panic!("Unexpected git2 error: {e}"))
        .resolve()
        .unwrap_or_else(|e| panic!("Unexpected git2 error: {e}"))
        .target()
        .unwrap()
}

pub fn git_switch(repo: &git2::Repository, commit_id: git2::Oid) {
    t!(repo.set_head_detached(commit_id));
    let mut builder = git2::build::CheckoutBuilder::new();
    builder.force();
    t!(repo.checkout_head(Some(&mut builder)));
}
