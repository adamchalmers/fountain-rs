//! Datatypes for storing Fountain documents.

use serde::{Deserialize, Serialize};

/// A Line represents a line of a screenplay, as defined in the [Fountain spec](https://fountain.io/syntax)
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum Line {
    /// A [Scene Heading](https://fountain.io/syntax#section-slug) is any line that has a blank line
    /// following it, and either begins with INT or EXT. A Scene Heading always has at least one
    /// blank line preceding it.
    Scene(String),
    /// [Action](https://fountain.io/syntax#section-action), or scene description, is any paragraph
    /// that doesn't meet criteria for another element (e.g. Scene Heading, Speaker, etc.)
    Action(String),
    /// [Dialogue](https://fountain.io/syntax#section-dialogue) is any text following a Speaker or
    /// Parenthetical element.
    Dialogue(String),
    /// A [Speaker](https://fountain.io/syntax#section-character) is any line entirely in uppercase.
    /// The Fountain spec defines this as a "Character" but this library calls it a Speaker to avoid
    /// confusion, as in computer science a character means something different.
    Speaker(String),
    /// [Parentheticals](https://fountain.io/syntax#section-paren) are wrapped in parentheses ()
    /// and end in newline.
    Parenthetical(String),
}

/// Defines a document's title page. TitlePage should appear at the start of a screenplay and look
/// like this:
/// ```
/// use fountain::parse_document;
/// use fountain::data::{TitlePage, Document};
/// let titlepage = "\
/// Title:
///     Alien
/// Author:
///     Dan O'Bannon
/// Revision:
///     8
/// ";
/// let expected_titlepage = TitlePage {
///     title: Some("Alien".to_owned()),
///     author: Some("Dan O'Bannon".to_owned()),
///     other: vec![("Revision".to_owned(), "8".to_owned())],
/// };
/// let doc = fountain::parse_document::<(&str, _)>(&titlepage);
/// let parsed_titlepage = doc.unwrap().1.titlepage;
/// assert_eq!(parsed_titlepage, expected_titlepage);
/// ```
#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TitlePage {
    /// Document author
    pub author: Option<String>,
    /// Document title
    pub title: Option<String>,
    /// Other items, stored as a vec of key-value pairs.
    pub other: Vec<(String, String)>,
}

/// A Document is the entire screenplay, both title page and its actual contents (stored as Lines).
#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Document {
    pub lines: Vec<Line>,
    pub titlepage: TitlePage,
}
