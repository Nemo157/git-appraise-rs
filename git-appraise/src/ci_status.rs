use serde_json;

use std::cmp::{ Ord, Ordering };
use std::str::FromStr;
use git2::{ Time };
use super::{ Result, Oid };

#[derive(Clone, Copy, Deserialize, Debug)]
pub enum Status {
  Success,
  Failure,
}

#[derive(Deserialize)]
struct Data {
  timestamp: Option<String>,
  url: Option<String>,
  status: Option<String>,
  agent: Option<String>,
}

pub struct CIStatus {
  commit: Oid,
  data: Data,
}

impl CIStatus {
  pub fn from_str(commit: Oid, s: &str) -> Result<CIStatus> {
    serde_json::de::from_str(s)
      .map_err(|err| From::from((err, s.to_string())))
      .map(|data| CIStatus::from_data(commit, data))
  }

  fn from_data(commit: Oid, data: Data) -> CIStatus {
    CIStatus {
      commit: commit,
      data: data,
    }
  }

  pub fn commit(&self) -> Oid {
    self.commit
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

pub struct ByTimestamp(pub CIStatus);

impl Eq for ByTimestamp {
}

impl PartialEq for ByTimestamp {
  fn eq(&self, other: &Self) -> bool {
    self.0.timestamp().map(|t| t.seconds()).eq(&other.0.timestamp().map(|t| t.seconds()))
  }
}

impl PartialOrd for ByTimestamp {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ByTimestamp {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.timestamp().map(|t| t.seconds()).cmp(&other.0.timestamp().map(|t| t.seconds()))
  }
}
