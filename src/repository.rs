use git2;

use std::path::Path;
use super::{ Result, Oid, Review, Reviews };

pub struct Repository {
  git: git2::Repository,
}

impl Repository {
  pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository> {
    git2::Repository::open(path)
      .map_err(From::from)
      .map(Repository::new)
  }

  pub fn all_reviews(&self) -> Result<Reviews> {
    Reviews::all(&self.git)
  }

  pub fn review_for(&self, id: Oid) -> Result<Review> {
    Review::for_commit(&self.git, id)
  }

  fn new(git: git2::Repository) -> Repository {
    Repository {
      git: git
    }
  }
// const CI_STATUSES: Option<&'static str> = Some("refs/notes/devtools/ci");
//   pub fn ci_statuses_for(&self, id: Oid) -> Result<Vec<Result<CIStatus>>> {
//     self.git.find_note(CI_STATUSES, id)
//       .map_err(From::from)
//       .and_then(|note| note.message()
//         .ok_or(Error::NotFound)
//         .map(|message| message.lines()
//           .map(|line| CIStatus::from_str(line))
//           .map(|res| res.map_err(From::from))
//           .collect()))
//   }
}

