use std::{collections::HashMap, rc::Rc};

use tree_sitter_highlight::HighlightConfiguration;

macro_rules! create_config {
    ($language:expr, $name:expr) => {{
        let mut config = HighlightConfiguration::new(
            $language.into(),
            $name,
            include_str!(concat!("../queries/", $name, "/highlights.scm")),
            "",
            "",
        )
        .unwrap();
        config.configure(crate::RECOGNISED_NAMES);
        Rc::new(config)
    }};
    ($language:expr, $name:expr, inj) => {{
        let mut config = HighlightConfiguration::new(
            $language.into(),
            $name,
            include_str!(concat!("../queries/", $name, "/highlights.scm")),
            include_str!(concat!("../queries/", $name, "/injections.scm")),
            "",
        )
        .unwrap();
        config.configure(crate::RECOGNISED_NAMES);
        Rc::new(config)
    }};
    ($language:expr, $name:expr, inj, loc) => {{
        let mut config = HighlightConfiguration::new(
            $language.into(),
            $name,
            include_str!(concat!("../queries/", $name, "/highlights.scm")),
            include_str!(concat!("../queries/", $name, "/injections.scm")),
            include_str!(concat!("../queries/", $name, "/locals.scm")),
        )
        .unwrap();
        config.configure(crate::RECOGNISED_NAMES);
        Rc::new(config)
    }};
}

pub(crate) fn default_languages() -> HashMap<String, Rc<HighlightConfiguration>> {
    let mut languages = HashMap::with_capacity(9);

    let bash = create_config!(tree_sitter_bash::LANGUAGE, "bash");
    languages.insert("bash".to_string(), bash.clone());
    languages.insert("sh".to_string(), bash);

    let c = create_config!(tree_sitter_c::LANGUAGE, "c", inj);
    languages.insert("c".to_string(), c);

    let html = create_config!(tree_sitter_html::LANGUAGE, "html", inj);
    languages.insert("html".to_string(), html);

    let json = create_config!(tree_sitter_json::LANGUAGE, "json");
    languages.insert("json".to_string(), json);

    let python = create_config!(tree_sitter_python::LANGUAGE, "python", inj);
    languages.insert("python".to_string(), python.clone());
    languages.insert("py".to_string(), python);

    let typescript = create_config!(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        "typescript",
        inj
    );
    languages.insert("typescript".to_string(), typescript.clone());
    languages.insert("ts".to_string(), typescript);

    languages
}
