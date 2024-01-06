use reqwest::Url;

pub fn add_param_to_url(url: &mut Url, name: &str, value: &Option<String>) {
    if let Some(val) = value {
        url.query_pairs_mut().append_pair(name, val);
    }
}
