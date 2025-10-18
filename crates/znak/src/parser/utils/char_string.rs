use std::{
    fmt::Display,
    ops::{Index, Range, RangeFrom},
};

#[derive(Clone)]
pub struct CharString(Vec<char>);
impl Display for CharString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}
impl From<String> for CharString {
    fn from(value: String) -> Self {
        CharString(value.chars().collect())
    }
}
impl From<&[char]> for CharString {
    fn from(value: &[char]) -> Self {
        CharString(value.to_vec())
    }
}
impl Index<usize> for CharString {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl Index<RangeFrom<usize>> for CharString {
    type Output = CharString;
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        Box::leak(Box::new(CharString::from(&self.0[index])))
    }
}
impl Index<Range<usize>> for CharString {
    type Output = CharString;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        Box::leak(Box::new(CharString::from(&self.0[index])))
    }
}
impl PartialEq<&str> for CharString {
    fn eq(&self, other: &&str) -> bool {
        &self.to_string() == other
    }
}
impl CharString {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn starts_with(&self, pat: &str) -> bool {
        self.to_string().starts_with(pat)
    }

    pub fn contains(&self, pat: &str) -> bool {
        self.to_string().contains(pat)
    }

    pub fn find(&self, pat: &str) -> Option<usize> {
        let str = self.to_string();
        let char_idxs = str.char_indices().collect::<Vec<(usize, char)>>();
        let idx = str.find(pat)?;
        char_idxs.iter().cloned().position(|(i, _)| i == idx)
    }
}
