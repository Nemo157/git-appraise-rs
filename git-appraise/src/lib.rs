#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(serde_macros)]

extern crate git2;
extern crate serde;
extern crate serde_json;

mod error;
mod result;
mod repository;
mod review;

pub use git2::Oid;
pub use error::Error;
pub use result::Result;

pub use repository::Repository;
pub use review::{ Review, Reviews };

  //   .notes(Some("refs/notes/devtools/reviews"))
  //   .unwrap()
  //   .map(|(_, id)| {
  //     let note = repo.find_note(Some("refs/notes/devtools/reviews"), id).unwrap();
  //     let review = serde_json::de::from_str(note.message().unwrap().lines().nth(0).unwrap()).unwrap();
  //     (id, review)
  //   })
  //   .collect();
