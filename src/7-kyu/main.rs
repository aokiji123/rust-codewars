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

// Complete the function that accepts a string parameter, and reverses each word in the string. All spaces in the string should be retained.
fn reverse_words(str: &str) -> String {
  str.split(' ').map(|word| word.chars().rev().collect::<String>()).collect::<Vec<_>>().join(" ")
}

// Your task is to write a function which returns the n-th term of the following series, which is the sum of the first n terms of the sequence (n is the input parameter).
fn series_sum(n: u32) -> String {
  if n == 0 {
    return "0.00".to_string();
  }
  let mut result: f64 = 0.0;
  let mut starting_number: f64 = 1.0;
  for _ in 1..=n {
    result += 1.0 / starting_number;
    starting_number += 3.0;
  }

  format!("{:.2}", result)
}

// Write a function that returns both the minimum and maximum number of the given list/array.
fn min_max(lst: &[i32]) -> (i32, i32) {
  (*lst.iter().min().unwrap(), *lst.iter().max().unwrap())
}

// Write a function that takes an array of strings as an argument and returns a sorted array containing the same strings, ordered from shortest to longest.
fn sort_by_length(arr: &[String]) -> Vec<String> {
  let mut result: Vec<String> = arr.to_vec();
  result.sort_by_key(|s| s.len());
  result
}
