use crate::model::{JenkinsItemVisitor, JenkinsItem};
use crate::jenkins::load_jenkins_item;

pub struct RecursingVisitor<'a> {
    pub delegate: Box<dyn JenkinsItemVisitor + 'a>,
    pub recurse: bool,
    pub client: &'a reqwest::blocking::Client,
    pub fuzzy_string: Option<String>,
    pub matcher: Box<dyn fuzzy_matcher::FuzzyMatcher>,
}
impl RecursingVisitor<'_> {
    pub fn matches(&self, value: &str)->bool {
        if let Some(filter) =&self.fuzzy_string {
            if let None = self.matcher.fuzzy_match(&value, &filter) {
                return false;
            }
        }
        true
    }
}
impl JenkinsItemVisitor for RecursingVisitor<'_> {
    fn visit_freestyle_project(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if let JenkinsItem::FreeStyleProject { name, .. } = it {
            if self.matches(name) {
                return self.delegate.visit_freestyle_project(it);
            }
        }
        Ok(())
    }

    fn visit_workflow_job(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if let JenkinsItem::WorkflowJob { name, .. } = it {
            if self.matches(name) {
                return self.delegate.visit_freestyle_project(it);
            }
        }
        Ok(())
    }

    fn visit_freestyle_build(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if let JenkinsItem::FreeStyleBuild { name, .. } = it {
            if self.matches(name) {
                return self.delegate.visit_freestyle_build(it);
            }
        }
        Ok(())
    }

    fn visit_hudson(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        self.delegate.visit_hudson(it)
    }

    fn visit_workflow_mb_project(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        self.delegate.visit_workflow_mb_project(it)
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
        self.delegate.visit_folder(it)
    }
}
