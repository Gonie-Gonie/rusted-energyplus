//! EnergyPlus oracle release metadata.

/// Locked EnergyPlus release metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleRelease {
    /// EnergyPlus version.
    pub version: &'static str,
    /// Git tag.
    pub tag: &'static str,
    /// Source commit.
    pub commit: &'static str,
    /// Windows x86_64 archive file.
    pub windows_x86_64_zip: &'static str,
    /// Windows x86_64 archive SHA256.
    pub windows_x86_64_sha256: &'static str,
}

/// Returns the initial oracle release.
#[must_use]
pub const fn default_oracle_release() -> OracleRelease {
    OracleRelease {
        version: "26.1.0",
        tag: "v26.1.0",
        commit: "6f2e40d10250a105b49966baa24d843711e61048",
        windows_x86_64_zip: "EnergyPlus-26.1.0-6f2e40d102-Windows-x86_64.zip",
        windows_x86_64_sha256: "0bb6932d277eed62f996b625f37c533b8c35f9af0c53710d961d8442fc4e70b3",
    }
}

#[cfg(test)]
mod tests {
    use super::default_oracle_release;

    #[test]
    fn oracle_version_is_locked() {
        let release = default_oracle_release();

        assert_eq!(release.version, "26.1.0");
        assert_eq!(release.commit, "6f2e40d10250a105b49966baa24d843711e61048");
    }
}
