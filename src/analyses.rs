use std::vec::IntoIter;
use git2::{ Repository, Note };
use super::{ Oid, Analysis };
use refs;

pub struct Analyses {
  iter: Option<IntoIter<Analysis>>,
}

impl Analyses {
  pub fn for_commit(git: &Repository, id: Oid) -> Analyses {
    Analyses::new(id, git.find_note(refs::ANALYSES, id).ok())
  }

  fn new(id: Oid, note: Option<Note>) -> Analyses {
    let iter = note
      .and_then(|note| note.message()
        .map(|msg| {
          msg.lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| Analysis::from_str(id, line).map_err(|e| println!("{}", e)).ok())
            .collect::<Vec<Analysis>>()
            .into_iter()
        }));
    Analyses {
      iter: iter,
    }
  }
}

impl Iterator for Analyses {
  type Item = Analysis;

  fn next(&mut self) -> Option<Analysis> {
    self.iter.as_mut().and_then(|iter| iter.next())
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.as_ref().map(|iter| iter.size_hint()).unwrap_or((0, Some(0)))
  }
}

