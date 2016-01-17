use git2;

use std::path::Path;
use super::{ Error, Result, Oid, Review, Reviews };

const REVIEWS: Option<&'static str> = Some("refs/notes/devtools/reviews");

pub struct Repository {
  git: git2::Repository,
}

impl Repository {
  pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository> {
    git2::Repository::open(path)
      .map(|git| Repository { git: git })
      .map_err(From::from)
  }

  pub fn reviews(&self) -> Result<Reviews> {
    self.git.notes(REVIEWS)
      .map(|reviews| Reviews::new(reviews))
      .map_err(From::from)
  }

  pub fn review(&self, id: Oid) -> Result<Review> {
    self.git.find_note(REVIEWS, id)
      .map_err(From::from)
      .and_then(|note|
        note.message()
          .ok_or(Error::NotFound)
          .and_then(|message| Review::from_str(message).map_err(From::from)))
  }
}

