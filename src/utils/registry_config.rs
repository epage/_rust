use serde::Deserialize;

/// The [`config.json`] file stored in the index.
///
/// The config file may look like:
///
/// ```json
/// {
///     "dl": "https://example.com/api/{crate}/{version}/download",
///     "api": "https://example.com/api",
///     "auth-required": false
/// }
/// ```
///
/// [`config.json`]: https://doc.rust-lang.org/nightly/cargo/reference/registry-index.html#index-configuration
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RegistryConfig {
    /// Download endpoint for all crates.
    ///
    /// The string is a template which will generate the download URL for the
    /// tarball of a specific version of a crate. The substrings `{crate}` and
    /// `{version}` will be replaced with the crate's name and version
    /// respectively.  The substring `{prefix}` will be replaced with the
    /// crate's prefix directory name, and the substring `{lowerprefix}` will
    /// be replaced with the crate's prefix directory name converted to
    /// lowercase. The substring `{sha256-checksum}` will be replaced with the
    /// crate's sha256 checksum.
    ///
    /// For backwards compatibility, if the string does not contain any
    /// markers (`{crate}`, `{version}`, `{prefix}`, or `{lowerprefix}`), it
    /// will be extended with `/{crate}/{version}/download` to
    /// support registries like crates.io which were created before the
    /// templating setup was created.
    ///
    /// For more on the template of the download URL, see [Index Configuration](
    /// https://doc.rust-lang.org/nightly/cargo/reference/registry-index.html#index-configuration).
    pub dl: String,

    /// API endpoint for the registry. This is what's actually hit to perform
    /// operations like yanks, owner modifications, publish new crates, etc.
    /// If this is None, the registry does not support API commands.
    pub api: Option<String>,

    /// Whether all operations require authentication. See [RFC 3139].
    ///
    /// [RFC 3139]: https://rust-lang.github.io/rfcs/3139-cargo-alternative-registry-auth.html
    #[serde(default)]
    pub auth_required: bool,
}

impl RegistryConfig {
    /// File name of [`RegistryConfig`].
    pub const NAME: &'static str = "config.json";
}

/// Make a path to a dependency, which aligns to
///
/// - [index from of Cargo's index on filesystem][1], and
/// - [index from Crates.io][2].
///
/// <div class="warning">
///
/// Note: For index files, `dep_name` must have had `to_lowercase` called on it.
///
/// </div>
///
/// [1]: https://docs.rs/cargo/latest/cargo/sources/registry/index.html#the-format-of-the-index
/// [2]: https://github.com/rust-lang/crates.io-index
pub fn make_dep_path(dep_name: &str, prefix_only: bool) -> String {
    let (slash, name) = if prefix_only {
        ("", "")
    } else {
        ("/", dep_name)
    };
    match dep_name.len() {
        1 => format!("1{}{}", slash, name),
        2 => format!("2{}{}", slash, name),
        3 => format!("3/{}{}{}", &dep_name[..1], slash, name),
        _ => format!("{}/{}{}{}", &dep_name[0..2], &dep_name[2..4], slash, name),
    }
}

pub fn crate_url(dl_template: &str, krate: &str, version: &str, sha256_checksum: &str) -> String {
    let url = if !dl_template.contains(CRATE_TEMPLATE)
        && !dl_template.contains(VERSION_TEMPLATE)
        && !dl_template.contains(PREFIX_TEMPLATE)
        && !dl_template.contains(LOWER_PREFIX_TEMPLATE)
        && !dl_template.contains(CHECKSUM_TEMPLATE)
    {
        // Original format before customizing the download URL was supported.
        format!("{dl_template}/{krate}/{version}/download")
    } else {
        let prefix = make_dep_path(krate, true);
        let lowerprefix = prefix.to_lowercase();
        dl_template
            .replace(CRATE_TEMPLATE, krate)
            .replace(VERSION_TEMPLATE, version)
            .replace(PREFIX_TEMPLATE, &prefix)
            .replace(LOWER_PREFIX_TEMPLATE, &lowerprefix)
            .replace(CHECKSUM_TEMPLATE, sha256_checksum)
    };

    url
}

const CRATE_TEMPLATE: &str = "{crate}";
const VERSION_TEMPLATE: &str = "{version}";
const PREFIX_TEMPLATE: &str = "{prefix}";
const LOWER_PREFIX_TEMPLATE: &str = "{lowerprefix}";
const CHECKSUM_TEMPLATE: &str = "{sha256-checksum}";

#[cfg(test)]
mod tests {
    use super::make_dep_path;

    #[test]
    fn prefix_only() {
        assert_eq!(make_dep_path("a", true), "1");
        assert_eq!(make_dep_path("ab", true), "2");
        assert_eq!(make_dep_path("abc", true), "3/a");
        assert_eq!(make_dep_path("Abc", true), "3/A");
        assert_eq!(make_dep_path("AbCd", true), "Ab/Cd");
        assert_eq!(make_dep_path("aBcDe", true), "aB/cD");
    }

    #[test]
    fn full() {
        assert_eq!(make_dep_path("a", false), "1/a");
        assert_eq!(make_dep_path("ab", false), "2/ab");
        assert_eq!(make_dep_path("abc", false), "3/a/abc");
        assert_eq!(make_dep_path("Abc", false), "3/A/Abc");
        assert_eq!(make_dep_path("AbCd", false), "Ab/Cd/AbCd");
        assert_eq!(make_dep_path("aBcDe", false), "aB/cD/aBcDe");
    }
}
