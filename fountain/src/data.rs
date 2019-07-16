//! Datatypes for storing Fountain documents.

/// A Line represents a line of a screenplay, as defined in the [Fountain spec](https://fountain.io/syntax)
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Line {
    /// A Scene Heading is any line that has a blank line following it, and either begins with INT or EXT.
    /// A Scene Heading always has at least one blank line preceding it.
    /// https://fountain.io/syntax#section-slug
    Scene(String),
    /// Action, or scene description, is any paragraph that doesn't meet criteria for another
    /// element (e.g. Scene Heading, Character, Dialogue, etc.)
    /// https://fountain.io/syntax#section-action
    Action(String),
    /// Dialogue is any text following a Character or Parenthetical element.
    /// https://fountain.io/syntax#section-dialogue
    Dialogue(String),
    /// A speaker is simply a Fountain "Character" element,
    /// i.e. any line entirely in uppercase and ends in newline. I renamed it "Speaker" interally
    /// to avoid confusion with a CS character i.e. a byte.
    Speaker(String),
    /// Parentheticals are wrapped in parentheses () and end in newline.
    /// https://fountain.io/syntax#section-paren
    Parenthetical(String),
}

/// Defines a document's metadata. Metadata should appear at the start of a screenplay and look
/// like this:
/// ```
/// let metadata = "
/// Title:
///     Alien
/// Author:
///     Dan O'Bannon
/// ";
/// ```
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Metadata {
    /// Document author
    pub author: Option<String>,
    /// Document title
    pub title: Option<String>,
    /// Other items, stored as a vec of key-value pairs.
    pub other: Vec<(String, String)>,
}

/// A Document is the entire screenplay, both metadata and its actual contents (stored as Lines).
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Document {
    pub lines: Vec<Line>,
    pub metadata: Metadata,
}
