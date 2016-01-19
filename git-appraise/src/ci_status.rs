use serde_json;

use std::str::FromStr;
use git2::{ Time };

#[derive(Clone, Copy, Deserialize, Debug)]
pub enum Status {
  Success,
  Failure,
}

#[derive(Deserialize)]
pub struct CIStatus {
  timestamp: Option<String>,
  url: Option<String>,
  status: Option<String>,
  agent: Option<String>,
}

impl CIStatus {
  pub fn from_str(s: &str) -> Result<CIStatus, (serde_json::error::Error, String)> {
    serde_json::de::from_str(s).map_err(|err| (err, s.to_string()))
  }

  pub fn timestamp(&self) -> Option<Time> {
    self.timestamp.as_ref()
      .and_then(|timestamp|
        FromStr::from_str(timestamp)
          .ok()
          .map(|time| Time::new(time, 0)))
  }

  pub fn url(&self) -> Option<&str> {
    self.url.as_ref().map(|s| &**s)
  }

  pub fn status(&self) -> Option<Status> {
    self.status.as_ref().and_then(|s| match &**s {
      "success" => Some(Status::Success),
      "failure" => Some(Status::Failure),
      _ => None,
    })
  }

  pub fn agent(&self) -> Option<&str> {
    self.agent.as_ref().map(|s| &**s)
  }
}
