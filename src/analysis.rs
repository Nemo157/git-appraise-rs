use serde_json;

use std::str::FromStr;
use git2::{ Time };
use super::{ Result, Oid };

#[derive(Deserialize, Debug)]
struct Data {
  timestamp: Option<String>,
  url: Option<String>,
  #[serde(default)]
  v: u32,
}

#[derive(Debug)]
pub struct Analysis {
  commit_id: Oid,
  data: Data,
}

impl Analysis {
  pub fn from_str(commit_id: Oid, s: &str) -> Result<Analysis> {
    serde_json::de::from_str(s)
      .map_err(|err| From::from((err, s.to_string())))
      .map(|data| Analysis::from_data(commit_id, data))
  }

  fn from_data(commit_id: Oid, data: Data) -> Analysis {
    Analysis {
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
}
