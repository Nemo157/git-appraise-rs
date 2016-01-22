use std::vec::IntoIter;
use super::{ Event };

pub struct Events {
  iter: IntoIter<Box<Event>>
}

impl Events {
  pub fn new(events: Vec<Box<Event>>) -> Events {
    Events {
      iter: events.into_iter(),
    }
  }
}

impl Iterator for Events {
  type Item = Box<Event>;

  fn next(&mut self) -> Option<Box<Event>> {
    self.iter.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

