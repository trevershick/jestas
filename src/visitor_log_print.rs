use crate::model::{JenkinsItem, JenkinsItemVisitor};

pub struct LogPrintVisitor<'a> {
    pub client: &'a reqwest::blocking::Client,
}

impl JenkinsItemVisitor for LogPrintVisitor<'_> {
    fn visit_freestyle_project(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        Ok(match it {
            JenkinsItem::FreeStyleProject {
                name, status, url, ..
            } => {
                println!("{} {} {}", name, status, url);
                println!("------------------------------------------");
                let text = crate::jenkins::load_job_log(url, self.client)?;
                println!("{}", text)
            }
            _ => (),
        })
    }
    fn visit_workflow_job(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        Ok(match it {
            JenkinsItem::WorkflowJob {
                name, status, url, ..
            } => {
                println!("{} {} {}", name, status, url);
                println!("------------------------------------------");
                let text = crate::jenkins::load_job_log(url, self.client)?;
                println!("{}", text)
            }
            _ => (),
        })
    }
    fn visit_workflow_mb_project(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
    fn visit_freestyle_build(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
    fn visit_hudson(&mut self, _s: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
    fn visit_folder(&mut self, _it: &JenkinsItem) -> crate::Result<()> {
        Ok(())
    }
}
