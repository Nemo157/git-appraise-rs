use git2;
use serde_json;

use std::str::FromStr;
use git2::{ Time };
use super::{ Oid };

#[derive(Deserialize)]
pub struct Review {
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

pub struct Reviews<'r> {
  notes: git2::Notes<'r>,
}

impl Review {
  pub fn from_str(s: &str) -> Result<Review, (serde_json::error::Error, String)> {
    serde_json::de::from_str(s).map_err(|err| (err, s.to_string()))
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

impl<'r> Reviews<'r> {
  pub fn new(notes: git2::Notes<'r>) -> Reviews<'r> {
    Reviews {
      notes: notes,
    }
  }
}

impl<'r> Iterator for Reviews<'r> {
  type Item = Oid;

  fn next(&mut self) -> Option<Oid> {
    self.notes.next().map(|(_, id)| id)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.notes.size_hint()
  }
}

