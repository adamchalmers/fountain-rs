use super::data::*;

fn line_to_html(line: &Line) -> String {
    match line {
        Line::Scene(s) => format!("<p class='scene'>{}</p>", s),
        Line::Action(s) => format!("<p class='action'>{}</p>", s),
        Line::Dialogue(s) => format!("<p class='dialogue'>{}</p>", s),
        Line::Speaker(s) => format!("<p class='speaker'>{}</p>", s),
        Line::Parenthetical(s) => format!("<p class='parenthetical'>({})</p>", s),
    }
}

fn metadata_to_html(m: Metadata) -> String {
    let title = format!(
        "<h1 class='metadata'>{}</h1>",
        m.title.unwrap_or("Untitled".to_string())
    );
    let author = format!(
        "<h3 class='metadata'>By {}</h3>",
        m.author.unwrap_or("Author unknown".to_string())
    );
    let other: Vec<_> = m
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

/// Renders HTML representation of a Fountain document.
pub fn to_html(document: &Document) -> String {
    let nodes: Vec<_> = document.lines.iter().map(line_to_html).collect();
    let style: Vec<_> = include_str!("style.css").split('\n').collect();
    format!(
        "
<html>
    <head>
        <style>
{}
        </style>
    </head>
    <body>
{}
{}
    </body>
</html>",
        style.join("\n"),
        document.metadata.clone().map(metadata_to_html).unwrap_or_default(),
        nodes.join("\n")
    )
}
