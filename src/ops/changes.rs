use crate::error::CliError;

#[derive(Clone, Debug, clap::Args)]
pub struct ChangesCli {}

impl ChangesCli {
    pub fn run(&self) -> Result<(), CliError> {
        Ok(())
    }
}
