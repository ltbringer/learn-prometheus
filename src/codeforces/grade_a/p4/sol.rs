// The smallest even value for a watermelon split is 2.
const SMALLEST_HALF: u8 = 2;

fn is_even(x: u8) -> bool {
    x % 2 == 0
}

fn solution(x: u8) -> String {
    // If the weight is less than the smallest half, it is impossible to split the watermelon.
    if x <= SMALLEST_HALF {
        return "NO".to_string();
    }
    let part = x - SMALLEST_HALF;
    if is_even(part) {
        return "YES".to_string();
    }
    "NO".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cases() -> Vec<(u8, String)> {
        return vec![
            (8, "YES".to_string()),
            (6, "YES".to_string()),
            (1, "NO".to_string()),
            (2, "NO".to_string()),
            (3, "NO".to_string()),
            (99, "NO".to_string())
        ]
    }

    #[test]
    fn test_solution() {
        for (input, expected) in cases() {
            assert_eq!(solution(input), expected);
        }
    }
}
