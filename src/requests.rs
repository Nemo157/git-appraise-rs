use std::slice::Iter;
use super::{ Request };

pub struct Requests<'a> {
  request: Option<&'a Request>,
  requests: Iter<'a, Request>,
}

impl<'a> Requests<'a> {
  pub fn new(request: &'a Request, requests: &'a Vec<Request>) -> Requests<'a> {
    Requests {
      request: Some(request),
      requests: requests.iter(),
    }
  }
}

impl<'a> Iterator for Requests<'a> {
  type Item = &'a Request;

  fn next(&mut self) -> Option<&'a Request> {
    match (self.requests.next(), self.request) {
      (None, None) => None,
      (None, request) => {
        self.request = None;
        request
      },
      (request, _) => request,
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    match (self.request, self.requests.size_hint()) {
      (None, size_hint) => size_hint,
      (Some(_), (min, max)) => (min + 1, max.map(|max| max + 1)),
    }
  }
}

