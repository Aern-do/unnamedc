use crate::Source;
use ariadne::Cache;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Debug, Default, Clone)]
pub struct SourceCache<'s> {
    sources: HashMap<&'s str, ariadne::Source<&'s str>>,
}

impl<'s> SourceCache<'s> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, source: Source<'s>) {
        self.sources.insert(source.file_name, source.content.into());
    }

    pub fn get(&self, id: &'s str) -> Option<&ariadne::Source<&'s str>> {
        self.sources.get(id)
    }
}

impl<'s> Cache<&'s str> for SourceCache<'s> {
    type Storage = &'s str;

    fn fetch(&mut self, id: &&'s str) -> Result<&ariadne::Source<Self::Storage>, impl Debug> {
        self.get(id).ok_or(Box::new("unknown source"))
    }

    fn display<'a>(&self, id: &'a &'s str) -> Option<impl Display + 'a> {
        Some(id)
    }
}

impl<'s> Cache<&'s str> for &SourceCache<'s> {
    type Storage = &'s str;

    fn fetch(&mut self, id: &&'s str) -> Result<&ariadne::Source<Self::Storage>, impl Debug> {
        self.get(id).ok_or(Box::new("unknown source"))
    }

    fn display<'a>(&self, id: &'a &'s str) -> Option<impl Display + 'a> {
        Some(id)
    }
}
