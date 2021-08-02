use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{ModelEntry, TrackKind, SliceBegin, SliceEnd};

#[derive(Debug)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    pub entries: Vec<Arc<ModelEntry>>,
}
impl Track {
    pub fn new(id: String, kind: TrackKind, entries: Vec<Arc<ModelEntry>>) -> Self {
        Self { id, kind, entries }
    }
}
impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({:?}, {}, [{}])",
            stringify!(Track),
            self.kind,
            self.id,
            self.entries.len()
        )
    }
}
impl From<notation_proto::prelude::Track> for Track {
    fn from(v: notation_proto::prelude::Track) -> Self {
        let entries = ModelEntry::new_entries(v.entries);
        Self::new(v.id, v.kind, entries)
    }
}
impl Track {
    pub fn index_of_mark(&self, begin: usize, mark: &String) -> Option<usize> {
        for i in begin..self.entries.len() {
            let entry = self.entries.get(i);
            if entry.is_some() && entry.unwrap().value.is_mark_string(mark) {
                return Some(i);
            }
        }
        None
    }
    pub fn get_entries(&self, begin: &SliceBegin, end: &SliceEnd) -> Vec<Arc<ModelEntry>> {
        let (index, count) = match (begin, end) {
            (SliceBegin::Mark(x), SliceEnd::Mark(y)) => match self.index_of_mark(0, x) {
                Some(index) => {
                    let index = index + 1;
                    match self.index_of_mark(index, y) {
                        Some(end) => (index, end - index),
                        None => (index, 0),
                    }
                }
                None => (0, 0),
            },
            (SliceBegin::Mark(x), SliceEnd::Count(y)) => match self.index_of_mark(0, x) {
                Some(index) => (index + 1, *y),
                None => (0, 0),
            },
            (SliceBegin::Index(x), SliceEnd::Mark(y)) => match self.index_of_mark(*x, y) {
                Some(end) => (*x, end - 1 - *x),
                None => (*x, 0),
            },
            (SliceBegin::Index(x), SliceEnd::Count(y)) => (*x, *y),
        };
        let mut entries = vec![];
        for i in index..(index + count) {
            let entry = self.entries.get(i);
            if entry.is_some() {
                entries.push(entry.unwrap().clone());
            }
        }
        entries
    }
}