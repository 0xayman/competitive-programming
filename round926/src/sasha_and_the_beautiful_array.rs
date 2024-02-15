fn sasha_and_the_beautiful_array(n: usize, arr: Vec<usize>) -> usize {
    let mut arr = arr;
    arr.sort_unstable_by(|a, b| b.cmp(a));

    let mut sum = 0;
    for i in 0..n - 1 {
        sum += arr[i] - arr[i + 1]
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sasha_and_the_beautiful_array() {
        assert_eq!(sasha_and_the_beautiful_array(3, vec![2, 1, 3]), 2);
        assert_eq!(sasha_and_the_beautiful_array(3, vec![69, 69, 69]), 0);
        assert_eq!(
            sasha_and_the_beautiful_array(5, vec![100, 54, 80, 43, 90]),
            57
        );
        assert_eq!(sasha_and_the_beautiful_array(4, vec![3, 4, 3, 3]), 1);
        assert_eq!(sasha_and_the_beautiful_array(2, vec![2, 1]), 1);
    }
}
