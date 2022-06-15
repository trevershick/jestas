#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;
use log::debug;

extern crate termion;
extern crate clap;
extern crate env_logger;
extern crate fuzzy_matcher;
extern crate shellexpand;

mod settings;
mod model;
mod visitor_job_print;
mod visitor_log_print;
mod visitor_recursing;
mod jenkins;

use crate::jenkins::load_jenkins_item;
use crate::visitor_job_print::JobPrintVisitor;
use crate::visitor_log_print::LogPrintVisitor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Client = reqwest::blocking::Client;

fn list_jobs(url: &str, recurse: bool, filters: Option<Vec<String>>) -> Result<()> {
    debug!("list_jobs({}, {})", url, recurse);
    // gather and print each job and it's status
    // load the object, recurse
    // iterate over the jobs and print them
    let client = Client::new();
    let item = load_jenkins_item(url, &client).expect("item loaded");
    let mut visitor = visitor_recursing::RecursingVisitor {
        delegate: Box::new(JobPrintVisitor{}),
        recurse,
        client: &client,
        fuzzy_string: filters.map(|f| f.join("")),
        matcher: Box::new(fuzzy_matcher::skim::SkimMatcherV2::default()),
    };
    item.walk(&mut visitor)?;
    Ok(())
}

fn list_logs(url: &str, recurse: bool, filters: Option<Vec<String>>) -> Result<()> {
    debug!("list_logs({}, {})", url, recurse);
    // gather and print each job and it's status
    // load the object, recurse
    // iterate over the logs and print them
    let client = Client::new();
    let item = load_jenkins_item(url, &client).expect("item loaded");
    let mut visitor = visitor_recursing::RecursingVisitor {
        delegate: Box::new(LogPrintVisitor{ client: &client }),
        recurse,
        client: &client,
        fuzzy_string: filters.map(|f| f.join("")),
        matcher: Box::new(fuzzy_matcher::skim::SkimMatcherV2::default()),
    };
    item.walk(&mut visitor)?;
    Ok(())
}

fn main() {
    env_logger::init();
    let settings = settings::Settings::new().unwrap();
    debug!("{:?}", settings);

    let server = settings.server.expect("server");

    if let Some(c) = settings.jobs {
        list_jobs(
            &server,
            c.recursive.unwrap_or_default(),
            c.filters,
        ).expect("list jobs");
    }
    if let Some(c) = settings.logs {
        list_logs(
            &server,
            c.recursive.unwrap_or_default(),
            c.filters,
        ).expect("list logs");
    }
}
