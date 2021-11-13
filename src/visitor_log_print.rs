use crate::model::{JenkinsItemVisitor, JenkinsItem};
use crate::jenkins::load_jenkins_item;

pub struct LogPrintVisitor<'a> {
    pub recurse: bool,
    pub client: &'a reqwest::blocking::Client,
    pub fuzzy_string: Option<String>,
    pub matcher: Box<dyn fuzzy_matcher::FuzzyMatcher>,
}

impl LogPrintVisitor<'_> {
    pub fn matches(&self, value: &str)->bool {
        if let Some(filter) =&self.fuzzy_string {
            if let None = self.matcher.fuzzy_match(&value, &filter) {
                return false;
            }
        }
        true
    }
}

impl JenkinsItemVisitor for LogPrintVisitor<'_> {
    fn visit_freestyle_project(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if let JenkinsItem::FreeStyleProject { name, status, url, .. } = it {
            if self.matches(name) {
                println!("{} {} {}", name, status,url);
                println!("------------------------------------------");
                let text = crate::jenkins::load_job_log(url, self.client)?;
                println!("{}", text);
            }
        }
        Ok(())
    }
    fn visit_workflow_job(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if let JenkinsItem::WorkflowJob { name, status, url, .. } = it {
            if self.matches(name) {
                println!("{} {} {}", name, status,url);
                println!("------------------------------------------");
                let text = crate::jenkins::load_job_log(url, self.client)?;
                println!("{}", text);
            }
        }
        Ok(())
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
    fn visit_folder(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if !self.recurse {
            return Ok(());
        }
        debug!("visit_folder");
        if let JenkinsItem::Folder { url, jobs, .. } = it {
            if let Some(j) = jobs {
                for x in j.into_iter() {
                    x.walk(self)?;
                }
                return Ok(());
            }
            debug!("Load folder");
            let item = load_jenkins_item(url, self.client)?;
            item.walk(self)?;
        }
        Ok(())
    }
}
