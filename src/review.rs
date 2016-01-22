use refs;

use std::str::FromStr;
use git2::{ Repository, Note };
use super::{ Error, Result, Oid, Request, CIStatuses, Analyses, Comments, Event, Events };
use request::{ ByTimestamp };

pub struct Review<'r> {
  git: &'r Repository,
  id: Oid,
  request: Request,
}

impl<'r> Review<'r> {
  pub fn for_commit(git: &'r Repository, id: Oid) -> Result<Review<'r>> {
    git.find_note(refs::REVIEWS, id)
      .map_err(From::from)
      .and_then(|note| Review::from_note(git, id, note))
  }

  pub fn from_note(git: &'r Repository, id: Oid, note: Note<'r>) -> Result<Review<'r>> {
    note.message()
      .ok_or(Error::NotFound)
      .and_then(|message|
        message.lines()
          .filter(|line| !line.is_empty())
          .filter_map(|line| Request::from_str(id, line).map_err(|e| println!("{}", e)).ok())
          .map(|req| ByTimestamp(req))
          .max()
          .map(|wrapper| wrapper.0)
          .ok_or(Error::NotFound)
          .map(|req| Review::from_request(git, id, req)))
  }

  pub fn id(&self) -> Oid {
    self.id
  }

  pub fn request(&self) -> &Request {
    &self.request
  }

  pub fn all_ci_statuses(&self) -> CIStatuses {
    CIStatuses::all_for_commit(&self.git, self.id)
  }

  pub fn latest_ci_statuses(&self) -> CIStatuses {
    CIStatuses::latest_for_commit(&self.git, self.id)
  }

  pub fn comments(&self) -> Comments {
    Comments::for_commit(&self.git, self.id)
  }

  pub fn analyses(&self) -> Analyses {
    Analyses::for_commit(&self.git, self.id)
  }

  pub fn events(&self) -> Events {
    let mut vec: Vec<Box<Event>> = vec![Box::new(self.request().clone()) as Box<Event>]
      .into_iter()
      .chain(self.all_ci_statuses().map(|status| Box::new(status) as Box<Event>))
      .chain(self.comments().map(|comment| Box::new(comment) as Box<Event>))
      .chain(self.analyses().map(|analysis| Box::new(analysis) as Box<Event>))
      .collect();
    vec.sort_by(|a, b| a.timestamp().cmp(&b.timestamp()));
    Events::new(vec)
  }

  fn from_request(git: &'r Repository, id: Oid, req: Request) -> Review<'r> {
    Review {
      git: git,
      id: id,
      request: req,
    }
  }
}
