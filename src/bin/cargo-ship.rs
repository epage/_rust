use cargo_ship::error;

fn main() {
    let res = run();
    exit(res)
}

fn run() -> Result<(), error::CliError> {
    Ok(())
}

fn exit(result: Result<(), error::CliError>) -> ! {
    let code = error::report(result);
    std::process::exit(code)
}
