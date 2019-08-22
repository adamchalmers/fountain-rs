use super::data::*;
use super::utils::*;

const DD_START: &str = "<div class='dual-dialogue'>";
const DD_END: &str = "</div> <!-- end dual dialogue -->";

fn line_as_html(line: &Line) -> String {
    match line {
        Line::Scene(s) => format!("<p class='scene'>{}</p>", s),
        Line::Action(s) => format!("<p class='action'>{}</p>", s),
        Line::Dialogue(s) => format!("<p class='dialogue'>{}</p>", s),
        Line::Speaker { name, is_dual: _ } => format!("<p class='speaker'>{}</p>", name),
        Line::Parenthetical(s) => format!("<p class='parenthetical'>({})</p>", s),
        Line::Transition(s) => format!("<p class='parenthetical'>({})</p>", s),
    }
}

impl TitlePage {
    fn as_html(&self) -> String {
        let title = format!(
            "<h1 class='titlepage'>{}</h1>",
            self.title.clone().unwrap_or_else(|| "Untitled".to_string())
        );
        let author = format!(
            "<h3 class='titlepage'>By {}</h3>",
            self.author
                .clone()
                .unwrap_or_else(|| "Author unknown".to_string())
        );
        let other: Vec<_> = self
            .other
            .iter()
            .map(|(k, v)| format!("<h5>{}: {}", k, v))
            .collect();
        let pagebreak = "<p class='page-break'></p>".to_string();
        format!(
            "{}\n{}\n{}\n{}\n",
            title,
            author,
            other.join("\n"),
            pagebreak
        )
    }
}

/// Renders HTML representation of a Fountain document. Root element is a div.
impl Document {
    pub fn as_html(&self) -> String {
        format!(
            "<div>\n{}\n{}\n</div>\n",
            if self.titlepage == TitlePage::default() {
                "".to_owned()
            } else {
                self.titlepage.as_html()
            },
            as_nodes(&self.lines).join("\n")
        )
    }
}

fn as_nodes(lines: &[Line]) -> Vec<String> {
    // Render all the lines
    let mut nodes = Vec::<String>::new();
    for line in lines {
        nodes.push(line_as_html(&line));
    }
    // Now go back and add dual dialogue elements
    let n = lines.len();
    for i in 0..n {
        if let Line::Speaker { is_dual: true, .. } = lines[i] {
            if let Some(dd) = dual_dialogue_bounds(&lines, i) {
                nodes.insert(dd.start, DD_START.to_owned());
                nodes.insert(dd.end + 1, DD_END.to_owned());
            }
        }
    }
    nodes
}

// Find the start/end bounds of the dual dialogue block, indicated by a carated Speaker block
// at the index provided.
fn dual_dialogue_bounds(lines: &[Line], dual_dialogue_carat: usize) -> Option<DualDialogue> {
    let start = position_before(&lines, dual_dialogue_carat, |line| line.is_speaker());
    let end = position_after(&lines, dual_dialogue_carat, |line| line.is_dialogue());
    match (start, end) {
        (Some(start), Some(end)) => Some(DualDialogue {
            start,
            end: end + 1,
        }),
        _ => None,
    }
}

struct DualDialogue {
    pub start: usize,
    pub end: usize,
}
