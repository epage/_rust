use cargo_ship::error;

fn main() {
    let res = run();
    exit(res)
}

fn run() -> Result<(), error::CliError> {
    env_logger::Builder::from_env("CARGO_LOG").init();

    Ok(())
}

fn exit(result: Result<(), error::CliError>) -> ! {
    let code = error::report(result);
    std::process::exit(code)
}
