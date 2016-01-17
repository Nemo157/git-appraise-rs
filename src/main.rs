#![feature(plugin, custom_derive)]
#![plugin(maud_macros)]
#![plugin(serde_macros)]

extern crate maud;
extern crate iron;
extern crate router;
extern crate logger;
extern crate git2;
extern crate serde;
extern crate serde_json;
extern crate pulldown_cmark;

use maud::PreEscaped;
use pulldown_cmark::html;
use std::env;
use iron::prelude::*;
use iron::status;
use router::*;
use logger::*;
use iron::mime::Mime;
use git2::{Oid, Repository};

struct Markdown<'a>(&'a str);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Review {
  timestamp: Option<String>,
  reviewRef: Option<String>,
  targetRef: String,
  requester: Option<String>,
  reviewers: Option<Vec<String>>,
  description: Option<String>,
  v: Option<u8>,
  baseCommit: Option<String>,
}

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

fn main() {
  let path = env::args().nth(1).unwrap();
  let repo = Repository::open(path).unwrap();
  let name = repo.head().unwrap().name().unwrap().to_string();
  let reviews: Vec<(Oid, Review)> = repo
    .notes(Some("refs/notes/devtools/reviews"))
    .unwrap()
    .map(|(_, id)| {
      let note = repo.find_note(Some("refs/notes/devtools/reviews"), id).unwrap();
      let review = serde_json::de::from_str(note.message().unwrap().lines().nth(0).unwrap()).unwrap();
      (id, review)
    })
    .collect();

  let r2 = reviews.clone();

  let mut router = Router::new();
  router.get("/",
    move |_: &mut Request| -> IronResult<Response> {
      let content_type = "text/html".parse::<Mime>().unwrap();
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
                $(serde_json::ser::to_string_pretty(review).unwrap())
              }
            }
          }
        }
      }).unwrap();
      Ok(Response::with((content_type, status::Ok, buffer)))
    });

  router.get("/:query",
    move |req: &mut Request| -> IronResult<Response> {
      let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap();
      let content_type = "text/html".parse::<Mime>().unwrap();
      let &(ref id, ref review) = r2.iter().filter(|&&(ref id, _)| *id == Oid::from_str(query).unwrap()).nth(0).unwrap();
      let mut buffer = String::new();
      let description = review.description.as_ref().map(|des| Markdown(des));
      html!(buffer, {
        head {
          style type="text/css" {
            $PreEscaped(style())
          }
        }
        body {
          ul {
            #if let Some(ref requester) = review.requester {
              li { "Requester: " $requester }
            }
            #if let Some(ref timestamp) = review.timestamp {
              li { "Timestamp: " $timestamp }
            }
            #if let Some(ref review_ref) = review.reviewRef {
              li { "Proposed merge: " $review_ref " -> " $review.targetRef }
            }
            #if let Some(ref reviewers) = review.reviewers {
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
      Ok(Response::with((content_type, status::Ok, buffer)))
    });

  let (logger_before, logger_after) = Logger::new(None);

  let mut chain = Chain::new(router);

  // Link logger_before as your first before middleware.
  chain.link_before(logger_before);

  // Link logger_after as your *last* after middleware.
  chain.link_after(logger_after);

  Iron::new(chain).http("localhost:3000").unwrap();
}
