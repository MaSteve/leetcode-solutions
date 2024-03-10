use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    HashSet::<i32>::from_iter(nums1)
        .intersection(&HashSet::from_iter(nums2))
        .map(|&v| v)
        .collect()
}

fn main() {}
