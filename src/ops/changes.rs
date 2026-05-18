use crate::error::CargoResult;
use crate::error::CliError;
use crate::utils::index::CratesIoIndex;
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
        let mut index = CratesIoIndex::new();

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
            let Some(baseline_version) = baseline_version(pkg, &mut index)? else {
                shell::status("new", format!("`{pkg_name}`"))?;
                continue;
            };
            let baseline_version_str = baseline_version.to_string();

            let baseline_sha = commit_sha(pkg_name, &baseline_version_str, &mut index)?;

            shell::status(
                "Changes",
                format!("for `{pkg_name}` from {baseline_version}"),
            )?;
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

fn baseline_version(
    pkg: &cargo_metadata::Package,
    index: &mut CratesIoIndex,
) -> CargoResult<Option<semver::Version>> {
    let Some(versions) = index.krate_versions(None, pkg.name.as_str(), Default::default())? else {
        return Ok(None);
    };
    let baseline = versions
        .into_iter()
        .filter_map(|v| semver::Version::parse(&v.version).ok())
        .filter(|v| *v <= pkg.version)
        .max();
    Ok(baseline)
}

fn commit_sha(name: &str, version: &str, index: &mut CratesIoIndex) -> CargoResult<Option<String>> {
    let krate = index.download(None, name, version, Default::default())?;
    Ok(None)
}
