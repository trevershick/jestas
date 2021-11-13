use crate::Result;
use crate::Client;
use crate::model::JenkinsItem;

fn api_json_url(url: &str) -> String {
    let mut u = url.to_string();
    u.push_str("/api/json");
    u
}

pub fn load_jenkins_item(url: &str, client: &Client) -> Result<JenkinsItem> {
    debug!("load_jenkins_item {}", url);
    let response = client
        .get(api_json_url(url))
        .query(&[("pretty", "true")])
        .send()?
        .json()?;
    Ok(response)
}
