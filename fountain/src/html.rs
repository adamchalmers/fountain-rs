use super::data::*;

impl Line {
    fn as_html(&self) -> String {
        match self {
            Line::Scene(s) => format!("<p class='scene'>{}</p>", s),
            Line::Action(s) => format!("<p class='action'>{}</p>", s),
            Line::Dialogue(s) => format!("<p class='dialogue'>{}</p>", s),
            Line::Speaker(s) => format!("<p class='speaker'>{}</p>", s),
            Line::Parenthetical(s) => format!("<p class='parenthetical'>({})</p>", s),
            Line::Transition(s) => format!("<p class='parenthetical'>({})</p>", s),
        }
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
        let nodes: Vec<_> = self.lines.iter().map(|l| l.as_html()).collect();
        format!(
            "<div>\n{}\n{}\n</div>\n",
            if self.titlepage == TitlePage::default() {
                "".to_owned()
            } else {
                self.titlepage.as_html()
            },
            nodes.join("\n")
        )
    }
}
