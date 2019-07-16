//! `fountain` parses and renders [Fountain markdown](https://fountain.io), which allows you to
//! write screenplays in plain text and render them into beautiful screenplay-formatted documents.
//! This crate currently only implements a subset of the full Fountain spec, but aims to eventually
//! be fully compliant.
//!
//! ## Quick Example
//!
//! ```no_run
//! // Parses a plain text Fountain-markup document and outputs HTML.
//! use fountain;
//! use nom::error::ErrorKind;
//! const SCREENPLAY: &str = "
//! INT. Mess hall
//!
//! The entire crew is seated. Hungrily swallowing huge portions of artificial food. The cat eats from a dish on the table.
//!
//! KANE
//! First thing I'm going to do when we get back is eat some decent food.
//! ";
//!
//! // Parse the Fountain-structured plaintext into a fountain::data::Document
//! let parse_result = fountain::parse_document::<(&str, ErrorKind)>(&SCREENPLAY);
//! match parse_result {
//!     Err(e) => eprintln!("Error while parsing the screenplay: {:?}", e),
//!     Ok(("", parsed)) => {
//!         eprintln!("Successfully parsed the document");
//!         println!("{}", parsed.as_html());
//!     }
//!     Ok((unparsed, parsed)) => {
//!         eprintln!("Couldn't parse the entire document. Unparsed section:");
//!         eprintln!("{}", unparsed);
//!         println!("{}", parsed.as_html());
//!     }
//! }
//! ```

pub mod data;
mod html;
mod parse;
pub use parse::document as parse_document;
