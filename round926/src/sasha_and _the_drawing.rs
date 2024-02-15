fn sasha_and_the_drawing(n: usize, k: usize) -> usize {
    let mut result = k / 2;
    if k % 2 != 0 {
        result += 1;
    }

    let total_diagonals = 4 * n - 2;

    if k == total_diagonals {
        return 2 * n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sasha_and_the_drawing() {
        assert_eq!(sasha_and_the_drawing(3, 4), 2);
        assert_eq!(sasha_and_the_drawing(3, 3), 2);
        assert_eq!(sasha_and_the_drawing(3, 10), 6);
        assert_eq!(sasha_and_the_drawing(3, 9), 5);
        assert_eq!(sasha_and_the_drawing(4, 7), 4);
        assert_eq!(sasha_and_the_drawing(7, 11), 6);
        assert_eq!(sasha_and_the_drawing(2, 3), 2);
    }
}
