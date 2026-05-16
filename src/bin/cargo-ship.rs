use cargo_ship::error;
use cargo_ship::ops;
use clap::Parser;

fn main() {
    let res = run();
    exit(res)
}

fn run() -> Result<(), error::CliError> {
    env_logger::Builder::from_env("CARGO_LOG").init();

    let CargoCli::Ship(ship) = CargoCli::parse();

    match &ship.ops {
        Ops::Changes(cli) => cli.run(),
    }
}

fn exit(result: Result<(), error::CliError>) -> ! {
    let code = error::report(result);
    std::process::exit(code)
}

#[derive(Debug, Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub enum CargoCli {
    #[command(name = "ship")]
    #[command(about, author, version)]
    Ship(ShipCli),
}

/// Automated release for Rust crates
#[derive(Debug, Clone, clap::Args)]
pub struct ShipCli {
    #[command(subcommand)]
    pub ops: Ops,
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum Ops {
    Changes(ops::changes::ChangesCli),
}
