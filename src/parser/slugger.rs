use std::collections::HashMap;

pub struct Slugger {
    ids: HashMap<String, usize>,
}

impl Slugger {
    pub fn new() -> Self {
        Self {
            ids: HashMap::new(),
        }
    }
    pub fn slug(&mut self, content: &str) -> String {
        let mut slug = content.to_lowercase().replace(char::is_whitespace, "-");
        match self.ids.get_mut(&slug) {
            Some(id) => {
                slug += "-";
                slug += &id.to_string();
                *id += 1;
            }
            None => _ = self.ids.insert(slug.clone(), 1),
        }
        slug
    }
}
