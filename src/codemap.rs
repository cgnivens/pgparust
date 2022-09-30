// For actually generating a map that points spans back to positions within
// the parsed file

use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;
use std::cmp;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize, Deserialize};
use crate::lexer::{Token, TokenKind};


#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Span(usize);


impl Span {
    /// Dummy span for testing
    pub(crate) fn dummy() -> Span {
        Span(0)
    }
}

// This is the map of ids to
// the files
#[derive(Debug)]
pub struct CodeMap {
    next_id: Rc<AtomicUsize>,
    files: Vec<Rc<FileMap>>,
}


impl CodeMap {
    pub fn new() -> CodeMap {
        let next_id = Rc::new(AtomicUsize::new(1));
        let files = Vec::new();
        CodeMap{next_id, files}
    }

    // Need to be able to insert a file into the codemap
    pub fn insert_file<C, F>(&mut self, filename: F, contents: C) -> Rc<FileMap>
    where F: Into<String>,
          C: Into<String> // Need these to be hashable
    {
        let filemap = FileMap {
            name: filename.into(),
            contents: contents.into(),
            items: RefCell::new(HashMap::new()),
            next_id: Rc::clone(&self.next_id),
        };
        let fm = Rc::new(filemap);
        self.files.push(Rc::clone(&fm));

        fm
    }

    // Need to be able to look up the span in the file
    pub fn lookup(&self, span: Span) -> &str {
        for filemap in &self.files {
            if let Some(substr) = filemap.lookup(span) {
                return substr;
            }
        }

        panic!("Tried to lookup {:?} but it wasn't in any of the FileMaps... This is a bug!", span)
    }

    // The files that the CodeMap contains
    pub fn files(&self) -> &[Rc<FileMap>] {
        self.files.as_slice()
    }


}

impl Default for CodeMap {
    fn default() -> CodeMap {
        CodeMap::new()
    }
}


// This is the map of each span
// to its location in the file
#[derive(Clone, Debug)]
pub struct FileMap {
    name: String,
    contents: String,
    next_id: Rc<AtomicUsize>,
    items: RefCell<HashMap<Span, Range<usize>>>
}

impl FileMap {
    // Return file name
    pub fn filename(&self) -> &str {
        &self.name
    }

    // Get contents
    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn lookup(&self, span: Span) -> Option<&str> {
        let range = match self.range_of(span) {
            Some(r) => r,
            None => return None,
        };

        match self.contents.get(range.clone()) {
            Some(substr) => Some(substr),
            None => panic!("FileMap thinks it contains {:?}, \
                but the range ({:?}) doesn't point to anything valid!", span, range),
        }
    }

    pub fn range_of(&self, span: Span) -> Option<Range<usize>> {
        self.items.borrow().get(&span).cloned()
    }

    pub fn insert_span(&self, start: usize, end: usize) -> Span {
        debug_assert!(self.contents.is_char_boundary(start),
            "Start doesn't lie on a char boundary");
        debug_assert!(self.contents.is_char_boundary(end),
            "End doesn't lie on a char boundary");
        debug_assert!(start < self.contents.len(),
            "Start lies outside the content string");
        debug_assert!(end <= self.contents.len(),
            "End lies outside the content string");

        let range = start..end;

        // Need to reverse-lookup the range to see if we are already
        // tracking this span
        if let Some(existing) = self.reverse_lookup(&range) {
            return existing;
        }

        // otherwise, increment the memory id of the touched region
        let span_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let span = Span(span_id);

        self.items.borrow_mut().insert(span, range);
        span
    }

    // Iterate through all of the spans within a file
    // to find one that exists. Each Range should only be in
    // a file once
    pub fn reverse_lookup(&self, needle: &Range<usize>) -> Option<Span> {
        self.items
            .borrow()
            .iter()
            .find(|&(_, range)| range == needle)
            .map(|(span, _)| span)
            .cloned()
    }

    // Merge overlapping spans into one (from the same FileMap)
    pub fn merge(&self, left: Span, right: Span) -> Span {
        let left = self.range_of(left).expect("Can only merge spans from the same FileMap");
        let right = self.range_of(right).expect("Can only merge spans from the same FileMap");

        let start = cmp::min(left.start, right.start);
        let end = cmp::max(left.end, right.end);

        self.insert_span(start, end)
    }

    // Register tokens into a stream
    pub fn register_tokens(&self, tokens: Vec<(TokenKind, usize, usize)>) -> Vec<Token> {
        let mut registered = Vec::new();

        for (kind, start, end) in tokens {
            let span = self.insert_span(start, end);
            let token = Token::new(span, kind);
            registered.push(token);
        }

        registered
    }
}


// TODO: Need to write tests here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        assert!(true);
    }
}
