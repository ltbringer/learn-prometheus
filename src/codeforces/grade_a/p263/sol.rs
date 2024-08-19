#[allow(dead_code)]
use std::io::{self, BufRead};

fn read<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let row: Vec<u8> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(row);
    }
    matrix
}

fn find(matrix: &Vec<Vec<u8>>, target: u8) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            if x == target {
                return (i, j);
            }
        }
    }
    // This can't happen as it is guaranteed that the target is in the matrix.
    panic!("Target not found");
}

fn count_moves(matrix: Vec<Vec<u8>>) -> u8 {
    let dims = (matrix.len(), matrix[0].len());
    let (i, j) = find(&matrix, 1);
    let center: (usize, usize) = (dims.0 / 2 as usize, dims.1 / 2 as usize).into();
    let moves = (i as i8 - center.0 as i8).abs() + (j as i8 - center.1 as i8).abs();
    moves as u8
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let matrix = read(stdin.lock());
    let result = count_moves(matrix);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn cases() -> Vec<(String, u8)> {
        return vec![
            ("0 0 0 0 0\n0 0 0 0 1\n0 0 0 0 0\n0 0 0 0 0\n0 0 0 0 0\n".to_string(), 3),
            ("0 0 0 0 0\n0 0 0 0 0\n0 0 1 0 0\n0 0 0 0 0\n0 0 0 0 0\n".to_string(), 0),
            ("0 0 0 0 0\n0 0 0 0 0\n0 0 0 0 0\n0 0 0 0 0\n0 0 0 0 1\n".to_string(), 4),
        ]
    }

    #[test]
    fn test_solution() {
        for (input, expected_output) in cases() {
            let cursor = Cursor::new(input);
            let matrix = read(cursor);
            assert_eq!(count_moves(matrix), expected_output);
        }
    }
}