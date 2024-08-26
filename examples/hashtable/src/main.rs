//! HashTable Data Structure
//!     - performance
//! 
//! 1. create a HashTable struct, define a hash() method
//! 2. collision resolution, open addressing, define hash() method
//! 
//! Links
//!     - HashMap
//!     - std::hash https://doc.rust-lang.org/stable/std/hash/trait.Hash.html
//!
//! Chanllenges
//!     - caches 
//!     - define Hash::hash() for other Rust types: String, i32, i64 ...

// use std::hash::Hasher;

// fn hash_key<Key: std::hash::Hash>(k: &Key) -> usize {
//     let mut hasher = std::hash::DefaultHasher::new();
//     k.hash(&mut hasher);
//     hasher.finish() as usize 
// }

trait Hash {
    fn hash(&self) -> usize;
}

impl Hash for usize {
    // if key1 != key2, hash(key1) != hash(key2)
    // if hash(key1) == hash(key2), key1 == key2
    fn hash(&self) -> usize {
        *self
    }
}

#[derive(Debug)]
struct HashTable<Key, Value> {
    data: Vec<Option<(Key, Value)>>
}

impl<Key, Value> HashTable<Key, Value> 
where 
    // Key: std::hash::Hash + Clone,
    Key: Hash + Clone + PartialEq,
    Value: Clone
{
    fn new() -> Self {
        const INITIAL_SIZE: usize = 7;
        Self { data: vec![None; INITIAL_SIZE] }
    }

    fn extend(&mut self) {
        let mut new_hashtable = Self { data: vec![None; self.data.len() * 2]};
        for i in 0..self.data.len() {
            if let Some((k, v)) = self.data.get(i).unwrap() {
                new_hashtable.insert(k.to_owned(), v.to_owned());
            }
        }

        *self = new_hashtable;
    }

    fn get_index(&self, k: &Key) -> usize {
        // hash_key(&k) % self.data.len()
        let mut index = k.hash() % self.data.len();
        while let Some((old_key, _)) = self.data.get(index).unwrap() {
            if old_key == k {
                return index;
            } else {
                index = (index + 1) % self.data.len();
            }
        }

        index 
    }

    fn insert(&mut self, k: Key, v: Value) {
        if self.data.iter().all(|kv| kv.is_some()) {
            self.extend();
        }

        if let Some(old_value) = self.get_mut(&k) {
            *old_value = v;
        } else {
            // a hash method "hash_key": key -> index
            let index = self.get_index(&k);
            self.data[index] = Some((k, v));
        }

        // let index = self.get_index(&k);
        // self.data[index] = Some((k, v));
    }

    fn get(&self, k: &Key) -> Option<&Value> {
        let index = self.get_index(&k);
        self.data.get(index).unwrap().as_ref()
            .map(|(_, v)| v)
    }

    fn get_mut(&mut self, k: &Key) -> Option<&mut Value> {
        let index = self.get_index(&k);
        self.data.get_mut(index).unwrap().as_mut()
            .map(|(_, v)| v)
    }

    fn clear(&mut self) {
        let size = self.data.len();
        self.data.clear();
        self.data = vec![None; size];
    }
}

fn main() {
    let mut hash_table: HashTable<usize, usize> = HashTable::new();
    hash_table.insert(1, 1);
    hash_table.insert(2, 2);
    dbg!(&hash_table);
    assert!(hash_table.get(&1) == Some(&1));
    assert!(hash_table.get(&2) == Some(&2));

    hash_table.insert(8, 8);
    hash_table.insert(9, 9);
    dbg!(&hash_table);
    assert!(hash_table.get(&1) == Some(&1));
    assert_eq!(hash_table.get(&2), Some(&2));
    assert!(hash_table.get(&8) == Some(&8));
    assert!(hash_table.get(&9) == Some(&9));

    hash_table.clear();
    dbg!(&hash_table);
    for i in 0..20 {
        hash_table.insert(i, i);
    }
    dbg!(&hash_table);
}
