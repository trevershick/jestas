use crate::Result;
use crate::Client;
use crate::model::JenkinsItem;

fn api_json_url(url: &str) -> String {
    let mut u = url.to_string();
    u.push_str("/api/json");
    u
}
pub fn load_job_log(job_url: &str, client: &Client) -> Result<String> {
    debug!("load_job_log {}", job_url);
    let response = client
        .get(format!("{}/lastBuild/logText/progressiveText?start=0", job_url))
        .send()?
        .text()?;
    Ok(response)
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
