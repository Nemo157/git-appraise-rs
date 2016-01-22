use serde_json;

use std::cmp::{ Ord, Ordering };
use std::str::FromStr;
use super::{ Oid, Time, Error, Result };

#[derive(Clone, Deserialize)]
pub struct Data {
  timestamp: Option<String>,
  #[serde(rename="reviewRef")]
  review_ref: Option<String>,
  #[serde(rename="targetRef")]
  target_ref: Option<String>,
  requester: Option<String>,
  reviewers: Option<Vec<String>>,
  description: Option<String>,
  #[serde(rename="baseCommit")]
  base_commit: Option<String>,
}

#[derive(Clone)]
pub struct Request {
  commit: Oid,
  data: Data,
}

impl Request {
  pub fn from_str(commit: Oid, s: &str) -> Result<Request> {
    serde_json::de::from_str(s)
      .map_err(|err| From::from((err, s.to_string())))
      .map(|data| Request::from_data(commit, data))
  }

  fn from_data(commit: Oid, data: Data) -> Request {
    Request {
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

  pub fn review_ref(&self) -> Option<&str> {
    self.data.review_ref.as_ref().map(|s| &**s)
  }

  pub fn target_ref(&self) -> Option<&str> {
    self.data.target_ref.as_ref().map(|s| &**s)
  }

  pub fn requester(&self) -> Option<&str> {
    self.data.requester.as_ref().map(|s| &**s)
  }

  pub fn reviewers(&self) -> Option<Vec<&str>> {
    self.data.reviewers.as_ref().map(|v| v.iter().map(|s| &**s).collect())
  }

  pub fn description(&self) -> Option<&str> {
    self.data.description.as_ref().map(|s| &**s)
  }

  pub fn base_commit(&self) -> Option<&str> {
    self.data.base_commit.as_ref().map(|s| &**s)
  }
}

pub struct ByTimestamp(pub Request);

impl Eq for ByTimestamp {
}

impl PartialEq for ByTimestamp {
  fn eq(&self, other: &Self) -> bool {
    self.0.timestamp().eq(&other.0.timestamp())
  }
}

impl PartialOrd for ByTimestamp {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ByTimestamp {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.timestamp().cmp(&other.0.timestamp())
  }
}
