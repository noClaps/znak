/// This macro can be used to apply a struct field to a value in the provided
/// template. Your field type must implement [std::fmt::Display], or must be a
/// [Vec] of a type that implements [Tmpl].
///
/// ```ignore
/// apply_field!(self, field, template); // for self.field: T where T: Display
/// apply_field!(self, [field], template); // for self.field: Vec<T> where T: Tmpl
/// ```
#[macro_export]
macro_rules! apply_field {
    ($self:ident, $field:ident, $template:ident) => {
        let $template =
            $template.replace(&format!("{{{{ {} }}}}", stringify!($field)), &$self.$field);
    };
    ($self:ident, [$field:ident], $template:ident) => {
        let $template = {
            let start = $template
                .find(&format!("{{{{ for {} }}}}", stringify!($field)))
                .unwrap()
                + format!("{{{{ for {} }}}}", stringify!($field)).len();
            let mut end = 0;
            let mut depth = 0;
            while end < $template[start..].len() {
                if $template[start + end..].starts_with("{{ end }}") {
                    if depth == 0 {
                        break;
                    } else {
                        depth -= 1;
                    }
                }
                if $template[start + end..].starts_with("{{ for") {
                    depth += 1
                }
                end += 1;
            }
            let inner = &$template[start..start + end];
            let replacement = $self.$field.render(inner);
            $template.replace(
                &format!("{{{{ for {} }}}}{}{{{{ end }}}}", stringify!($field), inner),
                &replacement,
            )
        };
    };
}

/// The Tmpl trait allows you to use the data from any struct to render a
/// template. The fields of the struct will be used as the data.
///
/// For instance, if you have a field named `title`, that will be used to
/// replace `{{ title }}` in the template.
///
/// To loop over a field of type [Vec], you can use a `{{ for }} ... {{ end }}`
/// loop. For instance, if your struct has a field named `entries` that is a
/// `Vec<Entry>`, such that `Entry` implements [Tmpl], then you can use
/// `{{ for entries }} ... {{ end }}`. Inside the loop, you can write a
/// template with the fields of `entries`.
///
/// ```
/// use tmpl::{Tmpl, apply_field};
///
/// struct Entry {
///     title: String,
///     description: String
/// }
/// impl Tmpl for Entry {
///     fn render(&self, template: &str) -> String {
///         apply_field!(self, title, template);
///         apply_field!(self, description, template);
///         template
///     }
/// }
///
/// struct Data {
///     page_title: String,
///     entries: Vec<Entry>
/// }
/// impl Tmpl for Data {
///     fn render(&self, template: &str) -> String {
///         apply_field!(self, page_title, template);
///         apply_field!(self, [entries], template);
///         template
///     }
/// }
/// ```
pub trait Tmpl {
    /// The `render` function takes your data struct and renders it to a
    /// [String] using the template you provide.
    fn render(&self, template: &str) -> String;
}

impl<T: Tmpl> Tmpl for Vec<T> {
    fn render(&self, template: &str) -> String {
        let mut out = String::new();
        for item in self {
            out += &item.render(template);
        }
        out
    }
}
impl Tmpl for String {
    fn render(&self, _template: &str) -> String {
        self.to_string()
    }
}
impl Tmpl for &str {
    fn render(&self, _template: &str) -> String {
        self.to_string()
    }
}
