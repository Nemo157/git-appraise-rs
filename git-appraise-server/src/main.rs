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

use maud::PreEscaped;
use pulldown_cmark::html;
use std::env;
use iron::prelude::*;
use iron::status;
use router::*;
use logger::*;
use iron::mime::Mime;
use git_appraise::{ Oid, Repository, Review };
use persistent::{ Read };
use typemap::Key;

struct Markdown<'a>(&'a str);

impl<'a> maud::Render for &'a Markdown<'a> {
  fn render(&self, w: &mut std::fmt::Write) -> std::fmt::Result {
    use std::fmt::Write;
    let mut s = String::with_capacity(self.0.len() * 3 / 2);
    let p = pulldown_cmark::Parser::new(&self.0);
    html::push_html(&mut s, p);
    write!(w, "{}", s)
  }
}

fn style() -> String {
  format!("
    html * {{
      color-profile: sRGB;
      rendering-intent: auto;
    }}

    body {{
      background-color: {base03};
      color: {base0};
    }}

    h1, h2, h3, h4, h5, h6 {{
      color: {base1};
      border-color: {base0};
    }}

    body {{
      font-family: \"Raleway\", \"HelveticaNeue\", \"Helvetica Neue\", Helvetica, Arial, sans-serif;
      margin: 20px auto;
      max-width: 650px;
      line-height: 1.4;
      font-size: 18px;
      padding: 0 10px;
    }}

    h1,h2,h3 {{
      font-family: \"Lucida Console\", Monaco, monospace;
      line-height: 1.2;
    }}

    a {{
      color: {blue};
      text-decoration: none;
      cursor: pointer;
    }}

    a:hover {{
      color: {lightblue}
    }}
  ",
    base03 = "#002b36",
    // base02 = "#073642",
    // base01 = "#586e75",
    // base00 = "#657b83",
    base0 = "#839496",
    base1 = "#93a1a1",
    // base2 = "#eee8d5",
    // base3 = "#fdf6e3",
    // yellow = "#b58900",
    // orange = "#cb4b16",
    // red = "#dc322f",
    // magenta = "#d33682",
    // violet = "#6c71c4",
    blue = "#268bd2",
    // cyan = "#2aa198",
    // green = "#859900",
    lightblue = "#3797db"
  )
}

fn get_reviews(repo: &Repository) -> Vec<(Oid, Review)> {
  repo.reviews().unwrap().map(|id| (id, repo.review(id).unwrap())).collect()
}

fn get_review(repo: &Repository, id: &str) -> Review {
  let id = Oid::from_str(id).unwrap();
  repo.review(id).unwrap()
}

fn render_reviews(reviews: Vec<(Oid, Review)>) -> String {
  let mut buffer = String::new();
  html!(buffer, {
    head {
      style type="text/css" {
        $PreEscaped(style())
      }
    }
    body {
      ol {
        #for &(ref id, ref review) in &reviews {
          li {
            a href={ "/" $id } $id
            " -> "
            $review.description().unwrap()
          }
        }
      }
    }
  }).unwrap();
  buffer
}

fn render_review(review: Review) -> String {
  let mut buffer = String::new();
  let description = review.description().map(|des| Markdown(des));
  html!(buffer, {
    head {
      style type="text/css" {
        $PreEscaped(style())
      }
    }
    body {
      ul {
        #if let Some(requester) = review.requester() {
          li { "Requester: " $requester }
        }
        #if let Some(timestamp) = review.timestamp() {
          li { "Timestamp: " $timestamp }
        }
        #if let (Some(review_ref), Some(target_ref)) = (review.review_ref(), review.target_ref()) {
          li { "Proposed merge: " $review_ref " -> " $target_ref }
        }
        #if let Some(reviewers) = review.reviewers() {
          li { "Reviewers:"
            ul {
              #for reviewer in reviewers {
                li $reviewer
              }
            }
          }
        }
        #if let Some(ref description) = description {
          li {
            "Description: "
            $description
          }
        }
      }
    }
  }).unwrap();
  buffer
}

pub fn result(buffer: String) -> IronResult<Response> {
  Ok(Response::with(("text/html".parse::<Mime>().unwrap(), status::Ok, buffer)))
}

fn reviews_handler(req: &mut iron::request::Request) -> IronResult<Response> {
  let path = req.get::<Read<RepositoryPath>>().unwrap();
  let repo = Repository::open(&*path).unwrap();
  let reviews = get_reviews(&repo);
  let buffer = render_reviews(reviews);
  result(buffer)
}

fn review_handler(req: &mut iron::request::Request) -> IronResult<Response> {
  let path = req.get::<Read<RepositoryPath>>().unwrap();
  let repo = Repository::open(&*path).unwrap();
  let review = get_review(&repo, req.extensions.get::<Router>().unwrap().find("query").unwrap());
  let buffer = render_review(review);
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
