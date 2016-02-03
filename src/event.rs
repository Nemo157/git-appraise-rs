use git2::{ Oid, Time };
use super::{ Request, Comment, Analysis, CIStatus };

pub enum EventKind {
  Request,
  Comment,
  Analysis,
  CIStatus,
}

pub trait Event {
  fn commit_id(&self) -> Oid;
  fn timestamp(&self) -> Option<Time>;

  fn kind(&self) -> EventKind;

  fn as_request(&self) -> Option<&Request>;
  fn as_comment(&self) -> Option<&Comment>;
  fn as_analysis(&self) -> Option<&Analysis>;
  fn as_ci_status(&self) -> Option<&CIStatus>;
}

impl Event for Request {
  fn commit_id(&self) -> Oid {
    self.commit_id()
  }
  fn timestamp(&self) -> Option<Time> {
    self.timestamp()
  }

  fn kind(&self) -> EventKind {
    EventKind::Request
  }

  fn as_request(&self) -> Option<&Request> {
    Some(self)
  }
  fn as_comment(&self) -> Option<&Comment> {
    None
  }
  fn as_analysis(&self) -> Option<&Analysis> {
    None
  }
  fn as_ci_status(&self) -> Option<&CIStatus> {
    None
  }
}

impl Event for Comment {
  fn commit_id(&self) -> Oid {
    self.commit_id()
  }
  fn timestamp(&self) -> Option<Time> {
    self.timestamp()
  }

  fn kind(&self) -> EventKind {
    EventKind::Comment
  }

  fn as_request(&self) -> Option<&Request> {
    None
  }
  fn as_comment(&self) -> Option<&Comment> {
    Some(self)
  }
  fn as_analysis(&self) -> Option<&Analysis> {
    None
  }
  fn as_ci_status(&self) -> Option<&CIStatus> {
    None
  }
}

impl Event for Analysis {
  fn commit_id(&self) -> Oid {
    self.commit_id()
  }
  fn timestamp(&self) -> Option<Time> {
    self.timestamp()
  }

  fn kind(&self) -> EventKind {
    EventKind::Analysis
  }

  fn as_request(&self) -> Option<&Request> {
    None
  }
  fn as_comment(&self) -> Option<&Comment> {
    None
  }
  fn as_analysis(&self) -> Option<&Analysis> {
    Some(self)
  }
  fn as_ci_status(&self) -> Option<&CIStatus> {
    None
  }
}

impl Event for CIStatus {
  fn commit_id(&self) -> Oid {
    self.commit_id()
  }
  fn timestamp(&self) -> Option<Time> {
    self.timestamp()
  }

  fn kind(&self) -> EventKind {
    EventKind::CIStatus
  }

  fn as_request(&self) -> Option<&Request> {
    None
  }
  fn as_comment(&self) -> Option<&Comment> {
    None
  }
  fn as_analysis(&self) -> Option<&Analysis> {
    None
  }
  fn as_ci_status(&self) -> Option<&CIStatus> {
    Some(self)
  }
}
