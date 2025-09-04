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

    pub fn get(&self, id: StrId) -> &str {
        &self.data[id.0]
    }

    fn add(&self, string: &str) -> StrId {
        let id = StrId(self.data.push(string.to_owned()));
        self.map.pin().insert(string.to_owned(), id);

        id
    }
}

pub static DEFAULT: LazyLock<Interner> = LazyLock::new(|| Interner::new());

#[cfg(test)]
mod tests {
    use crate::Interner;

    #[test]
    fn test_intern() {
        let interner = Interner::new();

        let str1 = "str 1";
        let str2 = "str 2";

        let id1 = interner.intern(str1);
        let id2 = interner.intern(str1);
        let id3 = interner.intern(str2);

        assert_eq!(interner.get(id1), str1);
        assert_eq!(interner.get(id2), str1);
        assert_eq!(interner.get(id3), str2);
    }
}
