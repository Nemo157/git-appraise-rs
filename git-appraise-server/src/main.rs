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

use maud::PreEscaped;
use std::env;
use iron::prelude::*;
use iron::status;
use router::*;
use logger::*;
use iron::mime::Mime;
use git_appraise::{ Oid, Repository, Review, CIStatus, Status };
use persistent::{ Read };
use typemap::Key;
use maud_pulldown_cmark::Markdown;

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

fn get_reviews(repo: &Repository) -> Vec<git_appraise::Result<(Oid, Review)>> {
  repo.reviews().unwrap().map(|id| repo.review(id).map(|rev| (id, rev))).collect()
}

fn get_ci_statuses_for(repo: &Repository, id: Oid) -> Vec<CIStatus> {
  repo.ci_statuses_for(id).unwrap().into_iter().filter_map(|s| s.ok()).collect()
}

fn get_review(repo: &Repository, id: Oid) -> Review {
  repo.review(id).unwrap()
}

fn render_reviews(reviews: Vec<Result<(Oid, Review), git_appraise::Error>>) -> String {
  let mut buffer = String::new();
  html!(buffer, {
    head {
      style type="text/css" {
        $PreEscaped(style())
      }
    }
    body {
      ol {
        #for rev in &reviews {
          #if let Some(&(ref id, ref review)) = rev.as_ref().ok() {
            li {
              a href={ "/" $id } $id
              " -> "
              $review.description().unwrap()
            }
          }
          #if let Some(ref err) = rev.as_ref().err() {
            li {
              $err
            }
          }
        }
      }
    }
  }).unwrap();
  buffer
}

fn render_review(review: Review, statuses: Vec<CIStatus>) -> String {
  let mut buffer = String::new();
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
          li { "Timestamp: " $(chrono::naive::datetime::NaiveDateTime::from_timestamp(timestamp.seconds(), 0)) }
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
        #if let Some(ref description) = review.description() {
          li {
            "Description: "
            $(Markdown::FromString(description))
          }
        }
        #if statuses.len() > 0 {
          li {
            "CI Statuses: "
            ol {
              #for status in &statuses {
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
  let statuses = get_ci_statuses_for(&repo, id);
  let buffer = render_review(review, statuses);
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
