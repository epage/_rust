use crate::error::CliError;
use crate::utils::shell;

#[derive(Clone, Debug, clap::Args)]
pub struct ChangesCli {
    #[command(flatten)]
    manifest: clap_cargo::Manifest,

    #[command(flatten)]
    workspace: clap_cargo::Workspace,
}

impl ChangesCli {
    pub fn run(&self) -> Result<(), CliError> {
        let mut metadata = self.manifest.metadata();
        let metadata = metadata.no_deps().exec()?;
        let (selected, excluded) = self.workspace.partition_packages(&metadata);
        let selected = selected
            .into_iter()
            .filter(|p| is_publishable(p))
            .collect::<Vec<_>>();

        if selected.is_empty() {
            shell::note("no compatible packages selected")?;
        }
        for pkg in selected {
            let pkg_name = pkg.name.as_str();

            shell::status("Changes", format!("for `{pkg_name}`"))?;
        }

        if !excluded.is_empty() {
            let excluded = excluded
                .into_iter()
                .filter(|p| is_publishable(p))
                .map(|p| format!("`{}`", p.name))
                .collect::<Vec<_>>();
            shell::note(format!("ignoring changes for {}", excluded.join(", ")))?;
        }

        Ok(())
    }
}

fn is_publishable(pkg: &cargo_metadata::Package) -> bool {
    match &pkg.publish {
        None => true,
        Some(registries) if registries.iter().any(|r| r == CRATES_IO_REGISTRY_NAME) => true,
        Some(_) => {
            log::trace!("package `{}` is not publishable", pkg.name);
            false
        }
    }
}

const CRATES_IO_REGISTRY_NAME: &str = "crates-io";
