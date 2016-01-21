#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(serde_macros)]

extern crate git2;
extern crate serde;
extern crate serde_json;

mod error;
mod result;
mod repository;
mod review;
mod reviews;
mod ci_status;
mod refs;
mod request;
mod ci_statuses;

pub use git2::Oid;
pub use error::Error;
pub use result::Result;

pub use repository::Repository;
pub use request::Request;
pub use review::Review;
pub use reviews::Reviews;
pub use ci_status::{ Status, CIStatus };
pub use ci_statuses::CIStatuses;
