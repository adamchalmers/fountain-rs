//! Datatypes for storing Fountain documents.
//! If you'd like these types to derive `Serialize` and `Deserialize` using `serde`, please set your
//! dependency on `fountain` to use the `use_serde` feature:
//! `fountain = { version = <target version>, features = ["use_serde"] }`
#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

/// A Line represents a line of a screenplay, as defined in the [Fountain spec](https://fountain.io/syntax)
/// This will impl Serialize and Deserialize if the feature "use_serde" is specified.
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
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
    /// The `is_dual` field indicates whether this is [Dual Dialogue](https://fountain.io/syntax#section-dual)
    /// i.e. the character speaking simultaneously with the previous character.
    Speaker { name: String, is_dual: bool },
    /// [Parentheticals](https://fountain.io/syntax#section-paren) are wrapped in parentheses ()
    /// and end in newline.
    Parenthetical(String),
    /// [Transitions](https://fountain.io/syntax#section-trans) end in TO. or start with >
    Transition(String),
}

impl Line {
    pub fn is_scene(&self) -> bool {
        if let Line::Scene(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_dialogue(&self) -> bool {
        if let Line::Dialogue(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_action(&self) -> bool {
        if let Line::Action(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_speaker(&self) -> bool {
        if let Line::Speaker { .. } = self {
            true
        } else {
            false
        }
    }
    pub fn is_parenthetical(&self) -> bool {
        if let Line::Parenthetical(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_transition(&self) -> bool {
        if let Line::Transition(_) = self {
            true
        } else {
            false
        }
    }
}

/// Defines a document's title page.
/// This will impl Serialize and Deserialize if the feature "use_serde" is specified.
///
/// TitlePage should appear at the start of a screenplay and look
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
#[derive(PartialEq, Eq, Clone, Debug, Default)]
#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
pub struct TitlePage {
    /// Document author
    pub author: Option<String>,
    /// Document title
    pub title: Option<String>,
    /// Other items, stored as a vec of key-value pairs.
    pub other: Vec<(String, String)>,
}

/// A Document is the entire screenplay, both title page and its actual contents (stored as Lines).
/// This will impl Serialize and Deserialize if the feature "use_serde" is specified.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
pub struct Document {
    pub lines: Vec<Line>,
    pub titlepage: TitlePage,
}
