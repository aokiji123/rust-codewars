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

// Write a function named setAlarm/set_alarm/set-alarm/setalarm (depending on language) which receives two parameters. The first parameter, employed, is true whenever you are employed and the second parameter, vacation is true whenever you are on vacation.
fn set_alarm(employed: bool, vacation: bool) -> bool {
  match (employed, vacation) {
    (true, false) => true,
    _ => false
  }
}

// Return an array, where the first element is the count of positives numbers and the second element is sum of negative numbers. 0 is neither positive nor negative.
// If the input is an empty array or is null, return an empty array.
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
  if input.is_empty() {
    return vec![];
  }

  let (mut positive_count, mut negative_sum) = (0, 0);

  for n in input {
    if n > 0 {
      positive_count += 1;
    } else {
      negative_sum += n;
    }
  }

  vec![positive_count, negative_sum]
}

// We need a function that can transform a string into a number. What ways of achieving this do you know?
fn string_to_number(s: &str) -> i32 {
  s.parse::<i32>().unwrap()
}

// Your task is to find the first element of an array that is not consecutive.
fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
  // .windows(n) - gives the slice of n numbers

  for i in arr.windows(2) {
    if i[1] != i[0] + 1 {
      return Some(i[1]);
    }
  }

  None
}

// A hero is on his way to the castle to complete his mission. However, he's been told that the castle is surrounded with a couple of powerful dragons! each dragon takes 2 bullets to be defeated, our hero has no idea how many bullets he should carry.. Assuming he's gonna grab a specific given number of bullets and move forward to fight another specific given number of dragons, will he survive?
fn hero(bullets: u16, dragons: u16) -> bool {
  bullets >= dragons * 2
}

// This function should test if the factor is a factor of base.
fn check_for_factor(base: i32, factor: i32) -> bool {
  base % factor == 0
}

// Write a program that finds the summation of every number from 1 to num (both inclusive). The number will always be a positive integer greater than 0. Your function only needs to return the result, what is shown between parentheses in the example below is how you reach that result and it's not part of it, see the sample tests.
fn summation(n: i32) -> i32 {
  (1..=n).sum()

  // alternative
  // n * (n + 1) / 2
}

// Write a function that accepts a non-negative integer n and a string s as parameters, and returns a string of s repeated exactly n times.
fn repeat_str(src: &str, count: usize) -> String {
  src.repeat(count)
}

// Write a function to split a string and convert it into an array of words.
fn string_to_array(s: &str) -> Vec<String> {
  s.split(' ').map(|word| word.to_string()).collect()
}