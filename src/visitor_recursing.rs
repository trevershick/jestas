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
        if let JenkinsItem::FreeStyleProject { .. } = it {
            if self.matches(it.name()) {
                return self.delegate.visit_freestyle_project(it);
            }
        }
        Ok(())
    }

    fn visit_workflow_job(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if let JenkinsItem::WorkflowJob { .. } = it {
            if self.matches(it.name()) {
                return self.delegate.visit_workflow_job(it);
            }
        }
        Ok(())
    }

    fn visit_freestyle_build(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        if let JenkinsItem::FreeStyleBuild { .. } = it {
            if self.matches(it.name()) {
                return self.delegate.visit_freestyle_build(it);
            }
        }
        Ok(())
    }

    fn visit_hudson(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        self.delegate.visit_hudson(it)
    }

    fn visit_workflow_mb_project(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_workflow_mb_project");
        self.delegate.visit_workflow_mb_project(it)?;

        if !self.recurse {
            return Ok(());
        }
        if let JenkinsItem::WorkflowMultiBranchProject { url, jobs, .. } = it {
            if let Some(j) = jobs {
                for x in j.into_iter() {
                    x.walk(self)?;
                }
                return Ok(());
            }
            debug!("Load MultiBranchJob");
            let item = load_jenkins_item(url, self.client)?;
            item.walk(self)?;
        }
        Ok(())
    }

    fn visit_folder(&mut self, it: &JenkinsItem) -> crate::Result<()> {
        debug!("visit_folder");
        self.delegate.visit_folder(it)?;

        if !self.recurse {
            return Ok(());
        }
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
