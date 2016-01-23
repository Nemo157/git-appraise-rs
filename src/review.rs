use refs;

use git2::{ Repository, Note };
use super::{ Error, Result, Oid, Request, Requests, CIStatuses, Analyses, Comments, Event, Events };

pub struct Review<'r> {
  git: &'r Repository,
  commit: Oid,
  request: Request,
  requests: Vec<Request>,
}

impl<'r> Review<'r> {
  pub fn for_commit(git: &'r Repository, commit: Oid) -> Result<Review<'r>> {
    git.find_note(refs::REVIEWS, commit)
      .map_err(From::from)
      .and_then(|note| Review::from_note(git, commit, note))
  }

  pub fn from_note(git: &'r Repository, commit: Oid, note: Note<'r>) -> Result<Review<'r>> {
    Request::all_from_note(commit, note)
      .and_then(|mut requests|
        if requests.is_empty() {
          Err(Error::NotFound)
        } else {
          requests.sort_by(|a, b| a.timestamp().cmp(&b.timestamp()));
          Ok((requests.pop().unwrap(), requests))
        })
      .map(|(request, requests)| Review::from_requests(git, commit, request, requests))
  }

  pub fn commit(&self) -> Oid {
    self.commit
  }

  pub fn request(&self) -> &Request {
    &self.request
  }

  pub fn all_requests(&self) -> Requests {
    Requests::new(&self.request, &self.requests)
  }

  pub fn all_ci_statuses(&self) -> CIStatuses {
    CIStatuses::all_for_commit(&self.git, self.commit())
  }

  pub fn latest_ci_statuses(&self) -> CIStatuses {
    CIStatuses::latest_for_commit(&self.git, self.commit())
  }

  pub fn comments(&self) -> Comments {
    Comments::for_commit(&self.git, self.commit())
  }

  pub fn analyses(&self) -> Analyses {
    Analyses::for_commit(&self.git, self.commit())
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

  fn from_requests(git: &'r Repository, commit: Oid, request: Request, requests: Vec<Request>) -> Review<'r> {
    Review {
      git: git,
      commit: commit,
      request: request,
      requests: requests,
    }
  }
}
