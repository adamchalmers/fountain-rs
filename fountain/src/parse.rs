use super::data::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{char, line_ending, not_line_ending, space1},
    combinator::{cut, map, opt},
    error::{context, ParseError},
    multi::{many0, separated_list},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

/// Matches strings that contain no lower-case English letters.
fn no_lower<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = "abcdefghijklmnopqrstuvwxyz\n\r";
    context("no_lower", take_while1(move |c| !chars.contains(c)))(i)
}

/// Parses an Action. Action, or scene description, is any paragraph that doesn't meet criteria for another
/// element (e.g. Scene Heading, Character, Dialogue, etc.)
/// https://fountain.io/syntax#section-action
fn action<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(context("action", some_line), |s: &str| {
        Line::Action(s.to_string())
    })(i)
}

/// Matches any sequence of non-line-ending characters, terminated by a line ending.
fn some_line<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    terminated(not_line_ending, line_ending)(i)
}

/// Parses a Dialogue. Dialogue is any text following a Character or Parenthetical element.
/// https://fountain.io/syntax#section-dialogue
fn dialogue<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(terminated(not_line_ending, line_ending), |s: &str| {
        Line::Dialogue(s.to_string())
    })(i)
}

/// Parses a Parenthetical. Parentheticals are wrapped in parentheses () and end in newline.
/// https://fountain.io/syntax#section-paren
fn parenthetical<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    let parser = terminated(in_parens, cut(line_ending));
    map(context("parenthetical", parser), |s: &str| {
        Line::Parenthetical(s.to_string())
    })(i)
}

/// Matches "(x)" and returns "x"
fn in_parens<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    delimited(char('('), is_not(")"), char(')'))(i)
}

/// Parses a Speaker. A speaker is simply a Fountain "Character" element,
/// i.e. any line entirely in uppercase and ends in newline. I renamed it "Speaker" interally
/// to avoid confusion with a CS character i.e. a byte.
/// https://fountain.io/syntax#section-character
fn speaker<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    let parser = terminated(no_lower, line_ending);
    map(context("speaker", parser), |s| Line::Speaker(s.to_string()))(i)
}

/// Parses a Scene Heading. A Scene Heading is any line that has a blank line following it, and either begins with INT or EXT.
/// A Scene Heading always has at least one blank line preceding it.
/// https://fountain.io/syntax#section-slug
fn scene<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    let parse_scene_type = alt((tag("INT"), tag("EXT")));
    let parser = tuple((parse_scene_type, tag(". "), not_line_ending, line_ending));
    map(context("scene", parser), |(scene_type, _, desc, _)| {
        Line::Scene(format!("{}. {}", scene_type, desc))
    })(i)
}

fn metadata_val<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = "\n\r:";
    let parser = take_while1(move |c| !chars.contains(c));
    context("metadata_val", parser)(i)
}

/// Match a single key-value metadata item, e.g.
/// Title:
///     THE RING
fn metadata_item<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (&str, &str), E> {
    let parser = tuple((metadata_val, tag(":"), line_ending, space1, some_line));
    map(context("MetadataItem", parser), |(key, _, _, _, val)| {
        (key, val)
    })(i)
}

/// Matches the document's Metadata
fn metadata<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Metadata, E> {
    map(context("Metadata", many0(metadata_item)), |items| {
        let mut m = Metadata::default();
        for (k, v) in items {
            match k {
                "Title" => m.title = Some(v.to_string()),
                "Author" => m.author = Some(v.to_string()),
                _ => m.other.push((k.to_string(), v.to_string())),
            }
        }
        m
    })(i)
}

/// Parses a string slice into a Fountain document.
pub fn document<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Document, E> {
    let parser = pair(opt(metadata), separated_list(line_ending, block));
    map(parser, |(metadata, blocks)| {
        let lines: Vec<_> = blocks.into_iter().flatten().collect();
        Document { lines, metadata }
    })(i)
}

/// A block is either:
/// - An action
/// - A scene header
/// - A speaker then dialogue
/// - A speaker then parenthetical then dialogue
fn block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<Line>, E> {
    let action = map(action, singleton);
    let scene = map(scene, singleton);
    context("block", alt((scene, spd_block, sd_block, action)))(i)
}

/// Creates a vector containing only the given element.
fn singleton<T>(t: T) -> Vec<T> {
    vec![t]
}

// Speaker then dialogue
fn sd_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<Line>, E> {
    let parser = context("sd block", pair(speaker, dialogue));
    map(parser, |lines| vec![lines.0, lines.1])(i)
}

// Speaker then parenthetical then dialogue
fn spd_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<Line>, E> {
    let parser = context("spd block", tuple((speaker, parenthetical, dialogue)));
    map(parser, |lines| vec![lines.0, lines.1, lines.2])(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::{ErrorKind, VerboseError};

    #[test]
    fn test_metadata() {
        let input_text = "\
Title:
    MUPPET TREASURE ISLAND
Author:
    Jerry Juhl
Pages:
    223
";
        let output = metadata::<VerboseError<&str>>(input_text);
        let expected = Metadata {
            title: Some("MUPPET TREASURE ISLAND".to_string()),
            author: Some("Jerry Juhl".to_string()),
            other: vec![("Pages".to_string(), "223".to_string())],
        };
        let expected = Ok(("", expected));
        assert_eq!(output, expected)
    }

    #[test]
    fn test_no_lower() {
        let input_text = "ADAM CHALMERS";
        let output = no_lower::<(&str, ErrorKind)>(input_text);
        let expected = Ok(("", "ADAM CHALMERS"));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_speaker() {
        let input_text = "MRS. THOMPSON\nWhat really caused the fall of Rome?\n";
        let output = speaker::<(&str, ErrorKind)>(input_text);
        let expected = Ok((
            "What really caused the fall of Rome?\n",
            Line::Speaker("MRS. THOMPSON".to_owned()),
        ));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_int_scene() {
        let input_text = "INT. Michael's house\n";
        let output = scene::<(&str, ErrorKind)>(input_text);
        let expected = Ok(("", Line::Scene("INT. Michael's house".to_owned())));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_ext_scene() {
        let input_text = "EXT. Michael's garden\n";
        let output = scene::<(&str, ErrorKind)>(input_text);
        let expected = Ok(("", Line::Scene("EXT. Michael's garden".to_owned())));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_action() {
        let input_text = "MICHAEL drops the plate.\n";
        let output = action::<VerboseError<&str>>(input_text);
        let expected = Ok(("", Line::Action("MICHAEL drops the plate.".to_owned())));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_some_line() {
        let input_text = "MICHAEL drops the glass\n";
        let output = some_line::<VerboseError<&str>>(input_text);
        let expected = Ok(("", "MICHAEL drops the glass"));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_in_parens() {
        let input_text = "(gasping)";
        let output = in_parens::<(&str, ErrorKind)>(input_text);
        assert_eq!(output, Ok(("", "gasping")));
    }

    #[test]
    fn test_sd_block() {
        let input_text = "LIBRARIAN\nIs anyone there?\n";
        let output = sd_block::<(&str, ErrorKind)>(input_text);
        let expected = vec![
            Line::Speaker("LIBRARIAN".to_string()),
            Line::Dialogue("Is anyone there?".to_string()),
        ];
        assert_eq!(output, Ok(("", expected)));
    }

    #[test]
    fn test_spd_block() {
        let input_text = "LIBRARIAN\n(scared)\nIs anyone there?\n";
        let output = spd_block::<(&str, ErrorKind)>(input_text);
        let expected = vec![
            Line::Speaker("LIBRARIAN".to_string()),
            Line::Parenthetical("scared".to_string()),
            Line::Dialogue("Is anyone there?".to_string()),
        ];
        assert_eq!(output, Ok(("", expected)));
    }

    #[test]
    fn test_parenthetical() {
        let input_text = "(gasping)\n";
        let output = parenthetical::<(&str, ErrorKind)>(input_text);
        dbg!(&output);
        let expected = Ok(("", Line::Parenthetical("gasping".to_owned())));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_document_tiny() {
        let input_text = "INT. Public library

Lights up on a table, totally empty except for a book.
";
        let output = document::<VerboseError<&str>>(input_text);
        assert!(output.is_ok());
        let (unparsed, output) = output.unwrap();
        dbg!(&output);
        dbg!(&unparsed);
        assert_eq!(output.lines.len(), 2);
    }

    #[test]
    fn test_document_small() {
        let input_text = "INT. Public library

Lights up on a table, totally empty except for a book.

LIBRARIAN
(scared)
Is anyone there?
";
        let output = document::<VerboseError<&str>>(input_text);
        assert!(output.is_ok());
        let (unparsed, output) = output.unwrap();
        dbg!(&output);
        dbg!(&unparsed);
        assert_eq!(output.lines.len(), 5);
    }

    #[test]
    fn test_alien() {
        let input_text = "\
INT. MESS

The entire crew is seated. Hungrily swallowing huge portions of artificial food. The cat eats from a dish on the table.

KANE
First thing I'm going to do when we get back is eat some decent food.
";
        let output = document::<VerboseError<&str>>(input_text);
        dbg!(&output);
        assert!(output.is_ok());
        let (unparsed, output) = output.unwrap();
        dbg!(&output);
        dbg!(&unparsed);
        assert_eq!(output.lines.len(), 4);
    }

    #[test]
    fn test_document() {
        let input_text = "\
Title:
    Stephen King Interview
INT. Set of some morning TV show.

PAULINE
(cheerily)
Welcome back to In Conversation, I'm your host Pauline Rogers and today we're talking to renowned horror writer Stephen King. Great to have you here, Stephen.

STEPHEN KING
Thanks for having me, Pauline.

PAULINE
My pleasure. Now, I'm sure you get asked this all the time, but, where do you get your ideas from?
";
        let output = document::<VerboseError<&str>>(input_text);
        let expected_lines = vec![
            Line::Scene("INT. Set of some morning TV show.".to_string()),
            Line::Speaker("PAULINE".to_string()),
            Line::Parenthetical("cheerily".to_string()),
            Line::Dialogue("Welcome back to In Conversation, I'm your host Pauline Rogers and today we're talking to renowned horror writer Stephen King. Great to have you here, Stephen.".to_string()),
            Line::Speaker("STEPHEN KING".to_string()),
            Line::Dialogue("Thanks for having me, Pauline.".to_string()),
            Line::Speaker("PAULINE".to_string()),
            Line::Dialogue("My pleasure. Now, I'm sure you get asked this all the time, but, where do you get your ideas from?".to_string()),
        ];
        let expected_metadata = Metadata {
            title: Some("Stephen King Interview".to_string()),
            ..Default::default()
        };
        let expected = Document {
            lines: expected_lines,
            metadata: Some(expected_metadata),
        };
        assert!(output.is_ok());
        let (unparsed, output) = output.unwrap();
        dbg!(&output);
        dbg!(&unparsed);
        assert_eq!(output, expected);
    }
}
