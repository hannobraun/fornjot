use regex::Regex;

pub fn find_version_in_str(s: &str) -> anyhow::Result<Option<semver::Version>> {
    let version = Regex::new(r"(\d+\.\d+\.\d+)")?
        .find_iter(s)
        .inspect(|version| {
            log::info!(
                "Found candidate for version in commit message: {}",
                version.as_str(),
            );
        })
        .filter_map(|m| {
            let version = semver::Version::parse(m.as_str()).ok();

            if version.is_some() {
                log::info!("Candidate confirmed.");
            } else {
                log::info!("Candidate not confirmed.");
            }

            version
        })
        .next();

    Ok(version)
}
