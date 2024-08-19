use std::{collections::HashMap};
use std::io::{self, BufRead};
#[allow(dead_code)]

// first line of the input is the number of test cases
// each test case has first line as the size of the string
// and the second line as the string itself
fn read<R: BufRead>(reader: R) -> Vec<String> {
    let mut lines = reader.lines();
    let n_cases = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut strings = Vec::new();

    for _ in 0..n_cases {
        let _ = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let string = lines.next().unwrap().unwrap();
        strings.push(string);
    }
    strings
}

fn count_balloons(seq: Vec<String>) -> Vec<u8> {
    let mut balloons: Vec<u8> = Vec::new();
    for text in seq.iter() {
        let mut problem_set: HashMap<char, u8> = HashMap::new();
        let mut balloon: u8 = 0;
        for chr in text.chars() {
            if problem_set.contains_key(&chr) {
                // Problem solved by another team before.
                balloon += 1;
            } else {
                // Problem solved first time.
                balloon += 2;
                problem_set.insert(chr, 1);
            }
        }
        balloons.push(balloon);
    }

    return balloons;
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let seq = read(stdin.lock());
    let result = count_balloons(seq);
    for r in result {
        println!("{}", r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn cases() -> Vec<(String, Vec<u8>)> {
        return vec![
            ("1\n3\nABA".to_string(), vec![5]),
            ("6\n3\nABA\n1\nA\n3\nORZ\n5\nBAAAA\n4\nBKPT\n10\nCODEFORCES".to_string(), vec![5, 2, 6, 7, 8, 17]),
        ]
    }

    #[test]
    fn test_count_balloons() {

        for case in cases() {
            let (input, expected) = case;
            let result = count_balloons(read(Cursor::new(input)));
            assert_eq!(result, expected);
        }
    }
}