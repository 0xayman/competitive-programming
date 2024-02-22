fn vlad_and_odd_ordering(n: usize, k: usize) -> usize {
    if k <= (n + 1) / 2 {
        k * 2 - 1
    } else {
        2 * vlad_and_odd_ordering(n / 2, k - (n + 1) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlad_and_odd_ordering() {
        assert_eq!(vlad_and_odd_ordering(7, 1), 1);
        assert_eq!(vlad_and_odd_ordering(7, 2), 3);
        assert_eq!(vlad_and_odd_ordering(7, 3), 5);
        assert_eq!(vlad_and_odd_ordering(7, 4), 7);
        assert_eq!(vlad_and_odd_ordering(7, 5), 2);
        assert_eq!(vlad_and_odd_ordering(7, 6), 6);
        assert_eq!(vlad_and_odd_ordering(7, 7), 4);
        assert_eq!(vlad_and_odd_ordering(1, 1), 1);
        assert_eq!(vlad_and_odd_ordering(34, 14), 27);
        assert_eq!(vlad_and_odd_ordering(84, 19), 37);
        assert_eq!(vlad_and_odd_ordering(1000000000, 1000000000), 536870912);
    }
}
