use serde_json;

use std::str::FromStr;
use git2::{ Oid, Time };
use super::{ Result };

#[derive(Deserialize, Debug)]
struct Data {
  timestamp: Option<String>,
  author: Option<String>,
  parent: Option<String>,
  location: Option<Location>,
  description: Option<String>,
  resolved: Option<bool>,
  #[serde(default)]
  v: u32,
}

#[derive(Deserialize, Debug)]
pub struct Location {
  commit: Option<String>,
  path: Option<String>,
  range: Option<Range>,
}

#[derive(Deserialize, Debug)]
pub struct Range {
  #[serde(rename="startLine")]
  start_line: Option<u32>,
}

#[derive(Debug)]
pub struct Comment {
  commit_id: Oid,
  data: Data,
}

impl Comment {
  pub fn from_str(commit_id: Oid, s: &str) -> Result<Comment> {
    serde_json::de::from_str(s)
      .map_err(|err| From::from((err, s.to_string())))
      .map(|data| Comment::from_data(commit_id, data))
  }

  fn from_data(commit_id: Oid, data: Data) -> Comment {
    Comment {
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

  pub fn author(&self) -> Option<&str> {
    self.data.author.as_ref().map(|s| &**s)
  }

  pub fn location(&self) -> Option<&Location> {
    self.data.location.as_ref()
  }

  pub fn description(&self) -> Option<&str> {
    self.data.description.as_ref().map(|s| &**s)
  }

  pub fn resolved(&self) -> Option<bool> {
    self.data.resolved
  }
}
