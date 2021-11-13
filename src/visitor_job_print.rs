use crate::model::{JenkinsItem, JenkinsItemVisitor};

pub struct JobPrintVisitor {}

impl JenkinsItemVisitor for JobPrintVisitor /*<'_>*/ {
    fn visit_freestyle_project(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        Ok(match it {
            JenkinsItem::FreeStyleProject {
                name, status, url, ..
            } => println!("{} {} {}", name, status, url),
            _ => (),
        })
    }
    fn visit_workflow_job(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        Ok(match it {
            JenkinsItem::WorkflowJob {
                name, status, url, ..
            } => println!("{} {} {}", name, status, url),
            _ => (),
        })
    }
    fn visit_freestyle_build(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
    fn visit_hudson(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
    fn visit_workflow_mb_project(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
    fn visit_folder(&mut self, _it: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
}
