#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;
use log::debug;

extern crate clap;
extern crate shellexpand;
extern crate fuzzy_matcher;

mod settings;
mod model;
mod visitor_job_print;
mod jenkins;

use crate::jenkins::load_jenkins_item;
use crate::visitor_job_print::JobPrintVisitor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Client = reqwest::blocking::Client;

fn list_jobs(url: &str, recurse: bool, filters: Option<Vec<String>>) -> Result<()> {
    debug!("list_jobs({}, {})", url, recurse);
    // gather and print each job and it's status
    // load the object, recurse
    // iterate over the jobs and print them
    let client = Client::new();
    let item = load_jenkins_item(url, &client).expect("item loaded");
    let mut visitor = JobPrintVisitor {
        recurse,
        client: &client,
        fuzzy_string: filters.map(|f| f.join("")),
        matcher: Box::new(fuzzy_matcher::skim::SkimMatcherV2::default()),
    };
    item.walk(&mut visitor)?;
    Ok(())
}

fn main() {
    //let url: &str = "http://localhost:8080";
    let settings = settings::Settings::new().unwrap();
    println!("{:?}", settings);

    if let Some(c) = settings.jobs {
        list_jobs(
            &settings.server.expect("server"),
            c.recursive.unwrap_or_default(),
            c.filters,
        ).expect("list jobs");
    }
}
