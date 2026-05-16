mod help;

use cargo_test_support::{ArgLineCommandExt, Execs, Project, execs, process};

pub fn cargo_ship_exe() -> std::path::PathBuf {
    snapbox::cmd::cargo_bin!("cargo-ship").to_path_buf()
}

pub trait ProjectExt {
    /// Creates an `Execs` instance to run the `cargo-ship` binary
    fn cargo_ship(&self, cmd: &str) -> Execs;
}

impl ProjectExt for Project {
    fn cargo_ship(&self, cmd: &str) -> Execs {
        let cargo_ship = cargo_ship_exe();

        let mut p = process(&cargo_ship);
        p.cwd(self.root()).arg_line(cmd);

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
