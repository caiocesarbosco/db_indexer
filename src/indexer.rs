use indexmap::IndexMap;
use crate::data::Data;

#[derive(Eq, Hash)]
pub struct Key {
    key: Vec<u8>
}

impl From<i64> for Key {
    fn from(key:i64) -> Key {
        Key {
            key: key.to_be_bytes().to_vec()
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

pub struct Entry<'a> {
    string_col: Data<'a>,
    num_col: Data<'a>
}

impl <'a>Entry<'a> {
    pub fn new(str_col: Data<'a>, number_col: Data<'a>) -> Entry<'a> {
        Entry {
            string_col: str_col,
            num_col: number_col
        }
    }
}
pub struct Indexer<'a> {
    index: IndexMap<Key, Entry<'a>>
}

impl <'a>Indexer<'a> {

    pub fn new() -> Indexer<'a> {
        Indexer {
            index: IndexMap::new()
        }
    }

    pub fn entry(&mut self, key: Key, value: Entry<'a>) {
        self.index.insert(key, value);
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inserting_data() {
        let tab: [(Key,Entry);4] = [(0.into(), Entry::new(Data::new_from_str("String A"), Data::new_from_number(4))), (2.into(), Entry::new(Data::new_from_str("String B"), Data::new_from_number(9))), (10.into(), Entry::new(Data::new_from_str("String C"), Data::new_from_number(75))), (0.into(), Entry::new(Data::new_from_str("q"), Data::new_from_number(3)))];
        let mut indexer = Indexer::new();
        for row in tab {
            indexer.entry(row.0, row.1);
        }
        assert_eq!(indexer.index.len(), 3);
    }

}

