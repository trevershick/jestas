use serde::de::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub enum JobStatus {
    Ok,
    Busy,
    New,
    Off,
    Fail,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            JobStatus::Ok => "ok",
            JobStatus::Busy => "busy",
            JobStatus::New =>"new",
            JobStatus::Off => "off",
            JobStatus::Fail => "fail",
        })
    }
}

fn deserialize_color<'de, D>(deserializer: D) -> std::result::Result<JobStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    match buf.as_str() {
        "blue" => return Ok(JobStatus::Ok),
        "notbuilt" => return Ok(JobStatus::New),
        "disabled" => return Ok(JobStatus::Off),
        &_ => (),
    };
    if buf.starts_with("aborted") {
        return Ok(JobStatus::Fail);
    }
    if buf.ends_with("_anime") {
        return Ok(JobStatus::Busy);
    }
    return Ok(JobStatus::Fail);
}

#[derive(Debug, Deserialize)]
#[serde(tag = "_class")]
pub enum JenkinsItem {
    #[serde(rename = "hudson.model.Hudson")]
    Hudson { url: String, jobs: Option<Vec<JenkinsItem>> },
    #[serde(rename = "hudson.model.FreeStyleProject")]
    FreeStyleProject {
        name: String,
        url: String,
        #[serde(rename = "color", deserialize_with = "deserialize_color")]
        status: JobStatus,
    },
    #[serde(rename = "hudson.model.FreeStyleBuild")]
    FreeStyleBuild {
        name: String,
        url: String,
        result: Option<String>,
    },
    #[serde(rename = "com.cloudbees.hudson.plugins.folder.Folder")]
    Folder {
        display_name: Option<String>,
        name: String,
        url: String,
        #[serde(default)]
        jobs: Option<Vec<JenkinsItem>>,
    },
    #[serde(rename = "org.jenkinsci.plugins.workflow.multibranch.WorkflowMultiBranchProject")]
    WorkflowMultiBranchProject {
        name: String,
        url: String,
        result: Option<String>,
    },
    #[serde(rename = "org.jenkinsci.plugins.workflow.job.WorkflowJob")]
    WorkflowJob {
        name: String,
        url: String,
        #[serde(rename = "color", deserialize_with = "deserialize_color")]
        status: JobStatus,
    },
    #[serde(other)]
    Other,
}

impl JenkinsItem {
    pub fn jobs_iter<'a>(&'a self) -> std::slice::Iter<'a, JenkinsItem> {
        if let JenkinsItem::Hudson { jobs, .. } = self {
            if let Some(j) = jobs {
                return j.iter();
            }
        }
        [].iter()
    }

    pub fn walk(&self, visitor: &mut dyn JenkinsItemVisitor) -> crate::Result<()> {
        debug!("walk {:?}", self);
        let result = match self {
            JenkinsItem::Folder { .. } => visitor.visit_folder(self),
            JenkinsItem::WorkflowJob { .. } => visitor.visit_workflow_job(self),
            JenkinsItem::FreeStyleProject { .. } => visitor.visit_freestyle_project(self),
            JenkinsItem::Hudson { .. } => visitor.visit_hudson(self),
            JenkinsItem::FreeStyleBuild { .. } => visitor.visit_freestyle_build(self),
            JenkinsItem::WorkflowMultiBranchProject { .. } => {
                visitor.visit_workflow_mb_project(self)
            }
            JenkinsItem::Other => Ok(()),
        };
        if let Err(e) = result {
            return Err(e);
        }

        for j in self.jobs_iter() {
            debug!("jobs_iter {:?}", j);
            if let Err(x) = j.walk(visitor) {
                return Result::Err(x);
            }
        }
        Ok(())
    }
}

pub trait JenkinsItemVisitor {
    fn visit_folder(&mut self, n: &JenkinsItem) -> crate::Result<()>;
    fn visit_freestyle_project(&mut self, s: &JenkinsItem) -> crate::Result<()>;
    fn visit_freestyle_build(&mut self, s: &JenkinsItem) -> crate::Result<()>;
    fn visit_hudson(&mut self, s: &JenkinsItem) -> crate::Result<()>;
    fn visit_workflow_mb_project(&mut self, s: &JenkinsItem) -> crate::Result<()>;
    fn visit_workflow_job(&mut self, s: &JenkinsItem) -> crate::Result<()>;
}

