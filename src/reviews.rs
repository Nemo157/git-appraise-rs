use refs;

use git2::{ Repository, Notes };
use super::{ Result, Review };

pub struct Reviews<'r> {
  git: &'r Repository,
  notes: Notes<'r>,
}

impl<'r> Reviews<'r> {
  pub fn all(git: &'r Repository) -> Result<Reviews<'r>> {
    git.notes(refs::REVIEWS)
      .map_err(From::from)
      .map(|notes| Reviews::new(git, notes))
  }

  fn new(git: &'r Repository, notes: Notes<'r>) -> Reviews<'r> {
    Reviews {
      git: git,
      notes: notes,
    }
  }
}

impl<'r> Iterator for Reviews<'r> {
  type Item = Result<Review<'r>>;

  fn next(&mut self) -> Option<Result<Review<'r>>> {
    let git = self.git;
    self.notes
      .next()
      .map(|note| note
        .map_err(From::from)
        .and_then(|(_, id)|
          git.find_note(refs::REVIEWS, id)
            .map_err(From::from)
            .and_then(|note| Review::from_note(git, id, note))))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let (_, max) = self.notes.size_hint();
    (0, max)
  }
}

