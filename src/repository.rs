use git2::{ Oid, Repository };
use super::{ Result, Review, Reviews };

pub trait AppraisedRepository {
  fn all_reviews(&self) -> Result<Reviews>;
  fn review_for(&self, id: Oid) -> Result<Review>;
}

impl AppraisedRepository for Repository {
  fn all_reviews(&self) -> Result<Reviews> {
    Reviews::all(self)
  }

  fn review_for(&self, id: Oid) -> Result<Review> {
    Review::for_commit(self, id)
  }
}
