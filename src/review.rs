use refs;

use git2::{ Oid, Repository, Note, Commit };
use super::{ Error, Result, Request, Requests, CIStatuses, Analyses, Comments, Event, Events };

pub struct Review<'r> {
  git: &'r Repository,
  id: Oid,
  request: Request,
  requests: Vec<Request>,
}

impl<'r> Review<'r> {
  pub fn for_commit(git: &'r Repository, id: Oid) -> Result<Review<'r>> {
    git.find_note(refs::REVIEWS, id)
      .map_err(From::from)
      .and_then(|note| Review::from_note(git, id, note))
  }

  pub fn from_note(git: &'r Repository, id: Oid, note: Note<'r>) -> Result<Review<'r>> {
    Request::all_from_note(id, note)
      .and_then(|mut requests|
        if requests.is_empty() {
          Err(Error::NotFound)
        } else {
          requests.sort_by(|a, b| a.timestamp().cmp(&b.timestamp()));
          Ok((requests.pop().unwrap(), requests))
        })
      .map(|(request, requests)| Review::from_requests(git, id, request, requests))
  }

  pub fn id(&self) -> Oid {
    self.id
  }

  pub fn commit(&self) -> Result<Commit> {
    self.git.find_commit(self.id).map_err(From::from)
  }

  pub fn request(&self) -> &Request {
    &self.request
  }

  pub fn all_requests(&self) -> Requests {
    Requests::new(&self.request, &self.requests)
  }

  pub fn all_ci_statuses(&self) -> CIStatuses {
    CIStatuses::all_for_commit(&self.git, self.id())
  }

  pub fn latest_ci_statuses(&self) -> CIStatuses {
    CIStatuses::latest_for_commit(&self.git, self.id())
  }

  pub fn comments(&self) -> Comments {
    Comments::for_commit(&self.git, self.id())
  }

  pub fn analyses(&self) -> Analyses {
    Analyses::for_commit(&self.git, self.id())
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

  fn from_requests(git: &'r Repository, id: Oid, request: Request, requests: Vec<Request>) -> Review<'r> {
    Review {
      git: git,
      id: id,
      request: request,
      requests: requests,
    }
  }
}
