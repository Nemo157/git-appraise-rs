use refs;

use std::str::FromStr;
use git2::{ Repository, Note };
use super::{ Error, Result, Oid };
use request::{ Request, ByTimestamp };

pub struct Review {
  id: Oid,
  request: Request,
}

impl Review {
  pub fn for_commit(git: &Repository, id: Oid) -> Result<Review> {
    git.find_note(refs::REVIEWS, id)
      .map_err(From::from)
      .and_then(|note| Review::from_note(id, note))
  }

  pub fn from_note<'r>(id: Oid, note: Note<'r>) -> Result<Review> {
    note.message()
      .ok_or(Error::NotFound)
      .and_then(|message|
        message.lines()
          .filter_map(|line| Request::from_str(line).ok())
          .map(|req| ByTimestamp(req))
          .max()
          .map(|wrapper| wrapper.0)
          .ok_or(Error::NotFound)
          .map(|req| Review::from_request(id, req)))
  }

  pub fn id(&self) -> Oid {
    self.id
  }

  pub fn request(&self) -> &Request {
    &self.request
  }

  fn from_request(id: Oid, req: Request) -> Review {
    Review {
      id: id,
      request: req,
    }
  }
}
