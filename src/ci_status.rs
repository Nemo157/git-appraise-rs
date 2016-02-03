use serde_json;

use std::str::FromStr;
use git2::{ Oid, Time };
use super::{ Result };

#[derive(Clone, Copy, Deserialize, Debug)]
pub enum Status {
  Success,
  Failure,
}

#[derive(Deserialize, Debug)]
struct Data {
  timestamp: Option<String>,
  url: Option<String>,
  status: Option<String>,
  agent: Option<String>,
  #[serde(default)]
  v: u32,
}

#[derive(Debug)]
pub struct CIStatus {
  commit_id: Oid,
  data: Data,
}

impl CIStatus {
  pub fn from_str(commit_id: Oid, s: &str) -> Result<CIStatus> {
    serde_json::de::from_str(s)
      .map_err(|err| From::from((err, s.to_string())))
      .map(|data| CIStatus::from_data(commit_id, data))
  }

  fn from_data(commit_id: Oid, data: Data) -> CIStatus {
    CIStatus {
      commit_id: commit_id,
      data: data,
    }
  }

  pub fn commit_id(&self) -> Oid {
    self.commit_id
  }

  pub fn timestamp(&self) -> Option<Time> {
    self.data.timestamp.as_ref()
      .and_then(|timestamp|
        FromStr::from_str(timestamp)
          .ok()
          .map(|time| Time::new(time, 0)))
  }

  pub fn url(&self) -> Option<&str> {
    self.data.url.as_ref().map(|s| &**s)
  }

  pub fn status(&self) -> Option<Status> {
    self.data.status.as_ref().and_then(|s| match &**s {
      "success" => Some(Status::Success),
      "failure" => Some(Status::Failure),
      _ => None,
    })
  }

  pub fn agent(&self) -> Option<&str> {
    self.data.agent.as_ref().map(|s| &**s)
  }

  pub fn key(&self) -> Option<&str> {
    self.url().or(self.agent())
  }
}
