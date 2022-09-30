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
}
