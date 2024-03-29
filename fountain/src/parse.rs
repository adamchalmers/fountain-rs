use super::data::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{char, line_ending, multispace1, not_line_ending},
    combinator::{cut, map, opt, verify},
    error::{context, ContextError, ParseError},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

/// Matches strings that contain no lower-case English letters.
fn no_lower<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let chars = "abcdefghijklmnopqrstuvwxyz\n\r";
    context("no_lower", take_while1(move |c| !chars.contains(c)))(i)
}

/// Parses an Action. Action, or scene description, is any paragraph that doesn't meet criteria for another
/// element (e.g. Scene Heading, Character, Dialogue, etc.)
/// https://fountain.io/syntax#section-action
fn action<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    map(context("action", some_line), |s: &str| {
        Line::Action(s.to_string())
    })(i)
}

/// Matches any sequence of non-line-ending characters, terminated by a line ending.
fn some_line<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    terminated(not_line_ending, line_ending)(i)
}

/// Parses a Dialogue. Dialogue is any text following a Character or Parenthetical element.
/// https://fountain.io/syntax#section-dialogue
fn dialogue<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    map(terminated(not_line_ending, line_ending), |s: &str| {
        Line::Dialogue(s.to_string())
    })(i)
}

/// Parses a Parenthetical. Parentheticals are wrapped in parentheses () and end in newline.
/// https://fountain.io/syntax#section-paren
fn parenthetical<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    let parser = terminated(in_parens, cut(line_ending));
    map(context("parenthetical", parser), |s: &str| {
        Line::Parenthetical(s.to_string())
    })(i)
}

/// Matches "(x)" and returns "x"
fn in_parens<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    delimited(char('('), is_not(")"), char(')'))(i)
}

/// Parses a Speaker. A speaker is simply a Fountain "Character" element,
/// i.e. any line entirely in uppercase and ends in newline. I renamed it "Speaker" interally
/// to avoid confusion with a CS character i.e. a byte.
/// https://fountain.io/syntax#section-character
fn speaker<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    let parser = terminated(no_lower, line_ending);
    map(context("speaker", parser), |s| Line::Speaker {
        name: strip_suffix(" ^", s),
        is_dual: s.ends_with('^'),
    })(i)
}

/// Parses a Transition, which ends with "TO:"
/// https://fountain.io/syntax#section-trans
fn transition_to<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    let p = verify(terminated(no_lower, line_ending), |s: &str| {
        s.ends_with("TO:")
    });
    let parser = map(p, |s| Line::Transition(s.to_owned()));
    context("transition_to", parser)(i)
}

/// Parses a Forced Transition, which either starts with >
/// https://fountain.io/syntax#section-trans
fn transition_forced<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    let p = preceded(tag("> "), some_line);
    let parser = map(p, |s| Line::Transition(s.to_owned()));
    context("transition_forced", parser)(i)
}

/// Parses a Scene Heading. A Scene Heading is any line that has a blank line following it, and either begins with INT or EXT.
/// A Scene Heading always has at least one blank line preceding it.
/// https://fountain.io/syntax#section-slug
fn scene<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    let parse_scene_type = alt((tag("INT"), tag("EXT")));
    let parser = tuple((parse_scene_type, tag(". "), not_line_ending, line_ending));
    map(context("scene", parser), |(scene_type, _, desc, _)| {
        Line::Scene(format!("{}. {}", scene_type, desc))
    })(i)
}

/// Parses a Lyric. You create a Lyric by starting with a line with a tilde ~. Fountain will remove
/// the '~' and leave it up to the app to style the Lyric appropriately. Lyrics are always forced.
/// There is no "automatic" way to get them.
/// https://fountain.io/syntax#section-lyric
fn lyric<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Line, E> {
    let parser = preceded(char('~'), some_line);
    map(context("lyric", parser), |s| Line::Lyric(s.to_owned()))(i)
}
fn titlepage_val<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let chars = "\n\r:";
    let parser = take_while1(move |c| !chars.contains(c));
    context("titlepage_val", parser)(i)
}

/// Match a single key-value titlepage item, e.g.
/// Title:
///     THE RING
fn titlepage_item<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&str, &str), E> {
    let parser = tuple((titlepage_val, char(':'), multispace1, some_line));
    map(context("titlepage_item", parser), |(key, _, _, val)| {
        (key, val)
    })(i)
}

/// Matches the document's TitlePage
fn titlepage<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, TitlePage, E> {
    map(context("Title page", many0(titlepage_item)), |items| {
        let mut m = TitlePage::default();
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

/// Parses a string slice into a Fountain document. Your input string should end in a
/// newline for parsing to succeed.
/// ```
/// use fountain::data::{Document, Line};
/// use nom::error::VerboseError;
///
/// const SCREENPLAY: &str = "\
/// KANE
/// First thing I'm going to do when we get back is eat some decent food.
/// ";
///
/// // Parse the Fountain-structured plaintext into a fountain::data::Document
/// let parse_result = fountain::parse_document::<VerboseError<&str>>(&SCREENPLAY);
/// let expected_lines = vec![
///     Line::Speaker{name: "KANE".to_owned(), is_dual: false},
///     Line::Dialogue("First thing I'm going to do when we get back is eat some decent \
/// food.".to_owned()),
/// ];
/// let expected = Document { lines: expected_lines, ..Default::default() };
/// assert_eq!(Ok(("", expected)), parse_result);
/// ```
pub fn document<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    text: &'a str,
) -> IResult<&'a str, Document, E> {
    let parser = pair(
        opt(terminated(titlepage, opt(line_ending))), // Documents may begin with a title page
        separated_list0(line_ending, block), // Documents must then contain screenplay lines
    );

    map(parser, |(titlepage, blocks)| {
        let lines: Vec<_> = blocks.into_iter().flatten().collect();
        Document {
            lines,
            titlepage: titlepage.unwrap_or_default(),
        }
    })(text)
}

/// A block is either:
/// - Speaker then dialogue
/// - Speaker then parenthetical then dialogue
/// - Some Fountain element which is not speaker, dialogue or parenthetical.
fn block<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<Line>, E> {
    context(
        "block",
        alt((
            map(transition_forced, singleton),
            map(transition_to, singleton),
            map(lyric, singleton),
            map(scene, singleton),
            spd_block,
            sd_block,
            map(action, singleton),
        )),
    )(i)
}

/// Creates a vector containing only the given element.
fn singleton<T>(t: T) -> Vec<T> {
    vec![t]
}

// Speaker then dialogue
fn sd_block<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<Line>, E> {
    let parser = context("sd block", pair(speaker, dialogue));
    map(parser, |lines| vec![lines.0, lines.1])(i)
}

// Speaker then parenthetical then dialogue
fn spd_block<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<Line>, E> {
    let parser = context("spd block", tuple((speaker, parenthetical, dialogue)));
    map(parser, |lines| vec![lines.0, lines.1, lines.2])(i)
}

fn strip_suffix(suffix: &str, string: &str) -> String {
    if let Some(stripped) = string.strip_suffix(suffix) {
        stripped.to_owned()
    } else {
        string.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::{ErrorKind, VerboseError};

    #[test]
    fn test_strip_suffix() {
        assert_eq!(strip_suffix(" ^", "Adam ^"), "Adam");
        assert_eq!(strip_suffix(" ^", "Adam"), "Adam");
    }

    #[test]
    fn test_titlepage() {
        let input_text = "\
Title: MUPPET TREASURE ISLAND
Author:
    Jerry Juhl
Pages:
    223
";
        let output = titlepage::<VerboseError<&str>>(input_text);
        let expected = TitlePage {
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
            Line::Speaker {
                name: "MRS. THOMPSON".to_owned(),
                is_dual: false,
            },
        ));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_transition() {
        let input_text = "FADE TO:\n";
        let output = transition_to::<VerboseError<&str>>(input_text);
        let expected = Ok(("", Line::Transition("FADE TO:".to_owned())));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_forced_transition() {
        let input_text = "> Burn to white.\n";
        let output = transition_forced::<(&str, ErrorKind)>(input_text);
        let expected = Ok(("", Line::Transition("Burn to white.".to_owned())));
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
    fn test_lyric() {
        let input_text = "~For he is an Englishman!\n";
        let output = lyric::<(&str, ErrorKind)>(input_text);
        let expected = Ok(("", Line::Lyric("For he is an Englishman!".to_owned())));
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
            Line::Speaker {
                name: "LIBRARIAN".to_string(),
                is_dual: false,
            },
            Line::Dialogue("Is anyone there?".to_string()),
        ];
        assert_eq!(output, Ok(("", expected)));
    }

    #[test]
    fn test_spd_block() {
        let input_text = "LIBRARIAN\n(scared)\nIs anyone there?\n";
        let output = spd_block::<(&str, ErrorKind)>(input_text);
        let expected = vec![
            Line::Speaker {
                name: "LIBRARIAN".to_string(),
                is_dual: false,
            },
            Line::Parenthetical("scared".to_string()),
            Line::Dialogue("Is anyone there?".to_string()),
        ];
        assert_eq!(output, Ok(("", expected)));
    }

    #[test]
    fn test_parenthetical() {
        let input_text = "(gasping)\n";
        let output = parenthetical::<(&str, ErrorKind)>(input_text);
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

CUT TO:

EXT. YOGA RETREAT

> Fade out
";
        let output = document::<VerboseError<&str>>(input_text);
        assert!(output.is_ok());
        let (unparsed, output) = output.unwrap();
        dbg!(&output);
        dbg!(&unparsed);
        assert_eq!(
            output.lines,
            vec![
                Line::Scene("INT. Public library".to_owned()),
                Line::Action("Lights up on a table, totally empty except for a book.".to_owned(),),
                Line::Speaker {
                    name: "LIBRARIAN".to_owned(),
                    is_dual: false
                },
                Line::Parenthetical("scared".to_owned(),),
                Line::Dialogue("Is anyone there?".to_owned(),),
                Line::Transition("CUT TO:".to_owned(),),
                Line::Scene("EXT. YOGA RETREAT".to_owned(),),
                Line::Transition("Fade out".to_owned(),),
            ]
        );
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
            Line::Speaker{name: "PAULINE".to_string(), is_dual: false},
            Line::Parenthetical("cheerily".to_string()),
            Line::Dialogue("Welcome back to In Conversation, I'm your host Pauline Rogers and today we're talking to renowned horror writer Stephen King. Great to have you here, Stephen.".to_string()),
            Line::Speaker{name: "STEPHEN KING".to_string(), is_dual: false},
            Line::Dialogue("Thanks for having me, Pauline.".to_string()),
            Line::Speaker{name: "PAULINE".to_string(), is_dual: false},
            Line::Dialogue("My pleasure. Now, I'm sure you get asked this all the time, but, where do you get your ideas from?".to_string()),
        ];
        let expected_titlepage = TitlePage {
            title: Some("Stephen King Interview".to_string()),
            ..Default::default()
        };
        let expected = Document {
            lines: expected_lines,
            titlepage: expected_titlepage,
        };
        assert!(output.is_ok());
        let (unparsed, output) = output.unwrap();
        dbg!(&output);
        dbg!(&unparsed);
        assert_eq!(output, expected);
    }
}
