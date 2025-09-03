use std::sync::LazyLock;

use boxcar::Vec;
use papaya::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrId(usize);

#[derive(Debug, Default, Clone)]
pub struct Interner {
    map: HashMap<String, StrId>,
    data: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&self, string: &str) -> StrId {
        if let Some(&idx) = self.map.pin().get(string) {
            return idx;
        }

        self.add(string)
    }

    fn add(&self, string: &str) -> StrId {
        let id = StrId(self.data.push(string.to_owned()));
        self.map.pin().insert(string.to_owned(), id);

        id
    }
}

pub static DEFAULT: LazyLock<Interner> = LazyLock::new(|| Interner::new());
