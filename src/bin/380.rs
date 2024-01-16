use rand::seq::SliceRandom;
use std::collections::HashMap;

struct RandomizedSet {
    mapping: HashMap<i32, usize>,
    list: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            list: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if !self.mapping.contains_key(&val) {
            self.mapping.insert(val, self.list.len());
            self.list.push(val);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.mapping.get(&val) {
            self.list[idx] = self.list[self.list.len() - 1];
            self.mapping.entry(self.list[idx]).and_modify(|e| *e = idx);
            self.list.pop();
            self.mapping.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        *self.list.choose(&mut rand::thread_rng()).unwrap()
    }
}

fn main() {}
