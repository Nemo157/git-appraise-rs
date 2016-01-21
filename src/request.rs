use serde_json;

use std::cmp::{ Ord, Ordering };
use std::str::FromStr;
use git2::{ Time };
use super::{ Error, Result };

#[derive(Deserialize)]
pub struct Request {
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

impl Request {
  pub fn from_str(s: &str) -> Result<Request> {
    serde_json::de::from_str(s).map_err(|err| From::from((err, s.to_string())))
  }

  pub fn timestamp(&self) -> Option<Time> {
    self.timestamp.as_ref()
      .and_then(|timestamp|
        FromStr::from_str(timestamp)
          .ok()
          .map(|time| Time::new(time, 0)))
  }

  pub fn review_ref(&self) -> Option<&String> {
    self.review_ref.as_ref()
  }

  pub fn target_ref(&self) -> Option<&String> {
    self.target_ref.as_ref()
  }

  pub fn requester(&self) -> Option<&String> {
    self.requester.as_ref()
  }

  pub fn reviewers(&self) -> Option<&Vec<String>> {
    self.reviewers.as_ref()
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn base_commit(&self) -> Option<&String> {
    self.base_commit.as_ref()
  }
}

pub struct ByTimestamp(pub Request);

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
