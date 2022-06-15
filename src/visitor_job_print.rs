use crate::model::{JenkinsItem, JenkinsItemVisitor};

use crate::model::JobStatus;
use termion::color::*;

pub struct JobPrintVisitor {}


fn status_color(s: &JobStatus) -> &'static str {
    match s {
        JobStatus::Ok => Green.fg_str(),
        JobStatus::Busy => Blue.fg_str(),
        JobStatus::New => White.fg_str(),
        JobStatus::Off => LightBlack.fg_str(),
        JobStatus::Fail =>Red.fg_str(),
    }
}

impl JenkinsItemVisitor for JobPrintVisitor {
    fn visit_freestyle_project(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_freestyle_project");
        Ok(match it {
            JenkinsItem::FreeStyleProject {
                status, url, ..
            } => println!("{}{:5} {}{:20} {}{}", status_color(status), format!("{}",status), Fg(White), it.name(), Fg(LightBlack), url),
            _ => (),
        })
    }
    fn visit_workflow_job(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_workflow_job");
        Ok(match it {
            JenkinsItem::WorkflowJob {
                status, url, ..
            } => println!("{}{:5} {}{:20} {}{}", status_color(status), format!("{}", status), Fg(White), it.name(), Fg(LightBlack), url),
            _ => (),
        })
    }
    fn visit_freestyle_build(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_freestyle_build");
        Ok(())
    }
    fn visit_hudson(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_hudson");
        Ok(())
    }
    fn visit_workflow_mb_project(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_workflow_mb_project");
        Ok(())
    }
    fn visit_folder(&mut self, _it: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_folder");
        Ok(())
    }
}
