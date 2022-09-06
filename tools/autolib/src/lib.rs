use regex::Regex;

pub fn find_version_in_str(s: &str) -> anyhow::Result<Option<regex::Match>> {
    let regex_match = Regex::new(r"(\d+\.\d+\.\d+)")?
        .find_iter(s)
        .inspect(|version| {
            log::info!(
                "Found candidate for version in commit message: {}",
                version.as_str(),
            );
        })
        .find(|m| {
            let confirmed = semver::Version::parse(m.as_str()).is_ok();

            if confirmed {
                log::info!("Candidate confirmed.");
            } else {
                log::info!("Candidate not confirmed.");
            }

            confirmed
        });

    Ok(regex_match)
}
