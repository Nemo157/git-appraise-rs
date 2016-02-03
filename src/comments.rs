use std::vec::IntoIter;
use git2::{ Oid, Repository, Note };
use super::{ Comment };
use refs;

pub struct Comments {
  iter: Option<IntoIter<Comment>>,
}

impl Comments {
  pub fn for_commit(git: &Repository, id: Oid) -> Comments {
    Comments::new(id, git.find_note(refs::DISCUSS, id).ok())
  }

  fn new(id: Oid, note: Option<Note>) -> Comments {
    let iter = note
      .and_then(|note| note.message()
        .map(|msg| {
          msg.lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| Comment::from_str(id, line).map_err(|e| println!("{}", e)).ok())
            .collect::<Vec<Comment>>()
            .into_iter()
        }));
    Comments {
      iter: iter,
    }
  }
}

impl Iterator for Comments {
  type Item = Comment;

  fn next(&mut self) -> Option<Comment> {
    self.iter.as_mut().and_then(|iter| iter.next())
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.as_ref().map(|iter| iter.size_hint()).unwrap_or((0, Some(0)))
  }
}

