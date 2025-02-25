use crate::types::Node;
use pulldown_latex::{config::DisplayMode, push_mathml, Parser, RenderConfig, Storage};

pub fn render_math(input: String, display_mode: DisplayMode) -> Node {
    let storage = Storage::new();
    let parser = Parser::new(&input, &storage);
    let mut output = String::new();
    let config = RenderConfig {
        display_mode,
        ..Default::default()
    };

    match push_mathml(&mut output, parser, config) {
        Ok(_) => Node::text(&output),
        Err(err) => {
            eprintln!("ERROR: Error parsing LaTeX \"{input}\": {err}");
            Node::text("")
        }
    }
}
