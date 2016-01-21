use std::collections::HashMap;
use std::vec::IntoIter;
use git2::{ Repository, Note };
use super::{ Oid, CIStatus };
use ci_status::ByTimestamp;
use refs;

pub struct CIStatuses {
  iter: Option<IntoIter<CIStatus>>,
}

impl CIStatuses {
  pub fn for_commit(git: &Repository, id: Oid) -> CIStatuses {
    CIStatuses::new(id, git.find_note(refs::CI_STATUSES, id).ok())
  }

  fn new(id: Oid, note: Option<Note>) -> CIStatuses {
    let iter = note
      .and_then(|note| note.message()
        .map(|msg| {
          let mut groups: HashMap<Option<String>, Vec<CIStatus>> = HashMap::new();
          let statuses = msg.lines().filter_map(|line| CIStatus::from_str(id, line).ok());
          for status in statuses {
            let group = groups.entry(status.key().map(|key| key.to_string())).or_insert(Vec::new());
            group.push(status);
          }
          groups.into_iter().flat_map(|(key, group)| match key {
            None => group,
            Some(_) => vec![group
              .into_iter()
              .map(|status| ByTimestamp(status))
              .max()
              .map(|wrapper| wrapper.0)
              .unwrap()]
          }).collect::<Vec<CIStatus>>().into_iter()
        }));
    CIStatuses {
      iter: iter,
    }
  }
}

impl Iterator for CIStatuses {
  type Item = CIStatus;

  fn next(&mut self) -> Option<CIStatus> {
    self.iter.as_mut().and_then(|iter| iter.next())
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.as_ref().map(|iter| iter.size_hint()).unwrap_or((0, Some(0)))
  }
}

