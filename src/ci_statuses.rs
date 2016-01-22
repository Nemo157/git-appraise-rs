use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::vec::IntoIter;
use git2::{ Repository, Note };
use super::{ Oid, CIStatus };
use refs;

pub struct CIStatuses {
  iter: IntoIter<CIStatus>,
}

fn find_all_for_commit(git: &Repository, id: Oid) -> Vec<CIStatus> {
  git.find_note(refs::CI_STATUSES, id).map(|note| from_note(id, note)).unwrap_or(Vec::new())
}

fn from_note<'r>(id: Oid, note: Note<'r>) -> Vec<CIStatus> {
  note.message()
    .map(|msg| from_msg(id, msg))
    .unwrap_or(Vec::new())
}

fn from_msg<'r>(id: Oid, msg: &'r str) -> Vec<CIStatus> {
  msg.lines()
    .filter(|line| !line.is_empty())
    .filter_map(|line| CIStatus::from_str(id, line).map_err(|e| println!("{}", e)).ok())
    .collect()
}

fn filter_to_latest(vec: Vec<CIStatus>) -> Vec<CIStatus> {
  let mut unkeyed: Vec<CIStatus> = Vec::new();
  let mut latest: HashMap<String, CIStatus> = HashMap::new();
  for status in vec {
    match status.key().map(|key| key.to_string()) {
      Some(key) => {
        match latest.entry(key) {
          Entry::Occupied(mut entry) => {
            if status.timestamp() > entry.get().timestamp() {
              entry.insert(status);
            }
          },
          Entry::Vacant(entry) => {
            entry.insert(status);
          }
        }
      },
      None => {
        unkeyed.push(status);
      }
    }
  }
  unkeyed.into_iter().chain(latest.into_iter().map(|(_, status)| status)).collect()
}

impl CIStatuses {
  pub fn all_for_commit(git: &Repository, id: Oid) -> CIStatuses {
    CIStatuses::new(find_all_for_commit(git, id))
  }

  pub fn latest_for_commit(git: &Repository, id: Oid) -> CIStatuses {
    CIStatuses::new(filter_to_latest(find_all_for_commit(git, id)))
  }

  fn new(vec: Vec<CIStatus>) -> CIStatuses {
    CIStatuses {
      iter: vec.into_iter(),
    }
  }
}

impl Iterator for CIStatuses {
  type Item = CIStatus;

  fn next(&mut self) -> Option<CIStatus> {
    self.iter.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

