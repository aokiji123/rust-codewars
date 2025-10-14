use std::collections::BTreeSet;

// Given two integers a and b, which can be positive or negative, find the sum of all the integers between and including them and return it. If the two numbers are equal return a or b.
fn get_sum(a: i64, b: i64) -> i64 {
  (a.min(b)..=a.max(b)).sum()
}

// Take 2 strings s1 and s2 including only letters from a to z. Return a new sorted string (alphabetical ascending), the longest possible, containing distinct letters - each taken only once - coming from s1 or s2.
fn longest(a1: &str, a2: &str) -> String {
  let mut set = BTreeSet::new();

  for c in a1.chars().chain(a2.chars()) {
    set.insert(c);
  }

  set.into_iter().collect()
}