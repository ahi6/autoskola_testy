use crate::BASE_URL;

pub(crate) fn resolve_asset_path(relative_url: &str) -> String {
    format!("{}/assets/output{}", BASE_URL, relative_url)
}
