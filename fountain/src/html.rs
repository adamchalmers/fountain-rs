use super::data::*;
use super::utils::*;

impl Line {
    fn as_html(&self) -> String {
        match self {
            Line::Scene(s) => format!("<p class='scene'>{}</p>", s),
            Line::Action(s) => format!("<p class='action'>{}</p>", s),
            Line::Dialogue(s) => format!("<p class='dialogue'>{}</p>", s),
            Line::Speaker { name, is_dual: _ } => format!("<p class='speaker'>{}</p>", name),
            Line::Parenthetical(s) => format!("<p class='parenthetical'>({})</p>", s),
            Line::Transition(s) => format!("<p class='parenthetical'>({})</p>", s),
        }
    }
}

enum Displayable {
    Text(Line),
    DualDialogueStart,
    DualDialogueEnd,
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
    let mut nodes = Vec::<String>::new();
    let mut in_dual_dialogue = false;
    for line in lines {
        nodes.push(line.as_html());
        // If it's a Speaker line, consider rendering Dual Dialogue.
    }
    nodes
}

// Find the start/end bounds of the dual dialogue block, indicated by a carated Speaker block
// at the index provided.
fn dual_dialogue_bounds(lines: &[Line], dual_dialogue_carat: usize) -> Option<DualDialogue> {
    let start = position_before(&lines, dual_dialogue_carat, |line| line.is_speaker());
    let end = position_after(&lines, dual_dialogue_carat, |line| line.is_dialogue());
    match (start, end) {
        (Some(start), Some(end)) => Some(DualDialogue { start, end }),
        _ => None,
    }
}

struct DualDialogue {
    pub start: usize,
    pub end: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_dialogue_bounds() {}
}
