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

use maud::{ PreEscaped, Render };
use std::env;
use std::fmt;
use iron::prelude::*;
use iron::status;
use router::*;
use logger::*;
use iron::mime::Mime;
use git_appraise::{ Oid, Repository, Review, Status, Comment };
use persistent::{ Read };
use typemap::Key;
use maud_pulldown_cmark::markdown;

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

fn get_reviews(repo: &Repository) -> Vec<Review> {
  repo.all_reviews().unwrap().collect()
}

fn get_review(repo: &Repository, id: Oid) -> Review {
  repo.review_for(id).unwrap()
}

fn render_reviews(reviews: Vec<Review>) -> String {
  let mut buffer = String::new();
  html!(buffer, {
    head {
      style type="text/css" {
        $PreEscaped(style())
      }
    }
    body {
      ol {
        #for review in &reviews {
          li {
            a href={ "/" $review.id() } $review.id()
            " -> "
            $review.request().description().unwrap()
          }
        }
      }
    }
  }).unwrap();
  buffer
}

struct RequestRenderer<'a>(&'a git_appraise::Request);

impl<'a> Render for RequestRenderer<'a> {
  fn render(&self, mut w: &mut fmt::Write) -> fmt::Result {
    let request = self.0;
    html!(w, {
      div {
        #if let Some(requester) = request.requester() {
          div { "Requester: " $requester }
        }
        #if let Some(timestamp) = request.timestamp() {
          div { "Timestamp: " $(chrono::naive::datetime::NaiveDateTime::from_timestamp(timestamp.seconds(), 0)) }
        }
        #if let (Some(review_ref), Some(target_ref)) = (request.review_ref(), request.target_ref()) {
          div { "Proposed merge: " $review_ref " -> " $target_ref }
        }
        #if let Some(reviewers) = request.reviewers() {
          div { "Reviewers:"
            ul {
              #for reviewer in reviewers {
                li $reviewer
              }
            }
          }
        }
        #if let Some(ref description) = request.description() {
          div {
            "Description: "
            div { $(markdown::from_string(description)) }
          }
        }
      }
    })
  }
}

struct CommentRenderer<'a>(&'a git_appraise::Comment);

impl<'a> Render for CommentRenderer<'a> {
  fn render(&self, mut w: &mut fmt::Write) -> fmt::Result {
    let comment = self.0;
    html!(w, {
      div {
        #if let Some(author) = comment.author() {
          div { "Comment from " $author }
        }
        div { "Comment Status: " $comment.resolved().map(|r| if r { "lgtm" } else { "nmw" }).unwrap_or("fyi") }
        #if let Some(location) = comment.location() {
          div { "Referencing " $(format!("{:?}", location)) }
        }
        #if let Some(description) = comment.description() {
          div { $(markdown::from_string(description)) }
        }
      }
    })
  }
}

fn render_review(review: Review) -> String {
  let mut buffer = String::new();
  html!(buffer, {
    head {
      style type="text/css" {
        $PreEscaped(style())
      }
    }
    body {
      $(RequestRenderer(review.request()))
      div {
        "CI Statuses: "
        ul {
          #for status in review.ci_statuses() {
            li {
              #if let Some(url) = status.url() {
                a href={ $url } $status.agent().unwrap_or("<Unknown agent>")
              }
              #if status.url().is_none() {
                $status.agent().unwrap_or("<Unknown agent>")
              }
              ": "
              $status.status().map(|s| match s { Status::Success => "success", Status::Failure => "failure" }).unwrap_or("null")
            }
          }
        }
      }
      div {
        "Analyses: "
        ul {
          #for analysis in review.analyses() {
            #if let Some(url) = analysis.url() {
              li {
                a href={ $url } $url
              }
            }
          }
        }
      }
      div {
        "Comments: "
        ul {
          #for comment in review.comments() {
            li {
              $(CommentRenderer(&comment))
            }
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
  let id = Oid::from_str(req.extensions.get::<Router>().unwrap().find("query").unwrap()).unwrap();
  let review = get_review(&repo, id);
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
