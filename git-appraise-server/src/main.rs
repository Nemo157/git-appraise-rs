#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate iron;
extern crate router;
extern crate logger;
extern crate pulldown_cmark;
extern crate git_appraise;
extern crate persistent;
extern crate typemap;
extern crate chrono;
extern crate maud_pulldown_cmark;

mod render;

use std::env;
use iron::prelude::*;
use iron::status;
use router::*;
use logger::*;
use iron::mime::Mime;
use git_appraise::{ Oid, Repository, Review };
use persistent::{ Read };
use typemap::Key;

fn get_reviews(repo: &Repository) -> Vec<Review> {
  repo.all_reviews().unwrap().collect()
}

fn get_review(repo: &Repository, id: Oid) -> Review {
  repo.review_for(id).unwrap()
}

pub fn result(buffer: String) -> IronResult<Response> {
  Ok(Response::with(("text/html".parse::<Mime>().unwrap(), status::Ok, buffer)))
}

fn reviews_handler(req: &mut iron::request::Request) -> IronResult<Response> {
  let path = req.get::<Read<RepositoryPath>>().unwrap();
  let repo = Repository::open(&*path).unwrap();
  let reviews = get_reviews(&repo);
  let buffer = render::reviews(reviews);
  result(buffer)
}

fn review_handler(req: &mut iron::request::Request) -> IronResult<Response> {
  let path = req.get::<Read<RepositoryPath>>().unwrap();
  let repo = Repository::open(&*path).unwrap();
  let id = Oid::from_str(req.extensions.get::<Router>().unwrap().find("query").unwrap()).unwrap();
  let review = get_review(&repo, id);
  let buffer = render::review(review);
  result(buffer)
}

#[derive(Copy, Clone)]
struct RepositoryPath;
impl Key for RepositoryPath { type Value = String; }

fn main() {
  let path = env::args().nth(1).unwrap();

  let mut router = Router::new();
  router.get("/", reviews_handler);
  router.get("/:query", review_handler);

  let (logger_before, logger_after) = Logger::new(None);

  let mut chain = Chain::new(router);

  chain.link(Read::<RepositoryPath>::both(path));
  chain.link_before(logger_before);
  chain.link_after(logger_after);

  Iron::new(chain).http("localhost:3000").unwrap();
}
