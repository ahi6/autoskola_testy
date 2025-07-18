// use crate::BASE_URL;

/// Just uses hotlinking for now.
pub(crate) fn resolve_asset_path(relative_url: &str) -> String {
    const BASE_URL: &str = "https://etesty2.mdcr.cz";
    format!("{}{}", BASE_URL, relative_url)
}
