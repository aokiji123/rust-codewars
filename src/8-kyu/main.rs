// Implement a function which convert the given boolean value into its string representation.
fn boolean_to_string(b: bool) -> String {
  b.to_string()
}

// Write a function that removes the spaces from the string, then return the resultant string.
fn no_space(x : String) -> String{
  let mut new_string = String::new();
  for i in x.chars() {
      if !i.is_whitespace() {
          new_string.push(i);
      }
  } 
  new_string

  // alternative:
  // x.replace(" ", "")
}

// Build a function that returns an array of integers from n to 1 where n>0
fn reverse_seq(n: u32) -> Vec<u32> {
  let mut v: Vec<u32> = (1..=n).collect();
  v.reverse();
  v
}

// Let's play! You have to return which player won! In case of a draw return Draw!.
fn rps(p1: &str, p2: &str) -> &'static str {
  if p1 == p2 {
      return "Draw!"
  } 

  match (p1, p2) {
      ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => "Player 1 won!",
      _ => "Player 2 won!",
  }
}

// Your task is to create a function that does four basic mathematical operations.
// The function should take three arguments - operation(string/char), value1(number), value2(number).
// The function should return result of numbers after applying the chosen operation.
fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
  match operator {
      '+' => value1 + value2,
      '-' => value1 - value2,
      '*' => value1 * value2,
      _ => value1 / value2,
  }
}

// Write a function to convert a name into initials. This kata strictly takes two words with one space in between them.
// The output should be two capital letters with a dot separating them.
fn abbrev_name(name: &str) -> String {
  let initials: Vec<String> = name
  .split(' ')
  .map(|s| s.chars().next().unwrap().to_uppercase().to_string())
  .collect();

  initials.join(".")
}

// Given a non-empty array of integers, return the result of multiplying the values together in order.
fn grow(nums: Vec<i32>) -> i32 {
  let mut result: i32 = 1;
  for i in nums {
      result *= i;
  }

  result

  // alternative
  // nums.iter().product()
}

// In this simple assignment you are given a number and have to make it negative.
fn make_negative(n: i32) -> i32 {
  match n {
      n if n > 0 => -n,
      _ => n,
  }

  // alternative:
  // -n.abs()
  // match n.is_positive() -> returns true / false
}

// Given a string, you have to return a string in which each character (case-sensitive) is repeated once.
fn double_char(s: &str) -> String {
  s.chars().map(|c| format!("{}{}", c, c)).collect()
}

// Your task is to make two functions ( max and min, or maximum and minimum, etc., depending on the language ) that receive a list of integers as input, and return the largest and lowest number in that list, respectively.
// Each function returns one number.
fn minimum(arr: &[i32]) -> i32 {
  *arr.iter().min().unwrap()
}

fn maximum(arr: &[i32]) -> i32 {
  *arr.iter().max().unwrap()
}