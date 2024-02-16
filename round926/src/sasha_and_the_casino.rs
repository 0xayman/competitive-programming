pub fn maximum_sum_less_than_k(k: usize, x: usize, a: usize) -> &'static str {
    let mut prefix_sum: usize = 1;

    let mut arr = vec![0; x + 1];
    arr[0] = 1;

    for i in 1..(x + 1) {
        arr[i] = prefix_sum / (k - 1);
        while arr[i] * (k - 1) <= prefix_sum {
            arr[i] += 1;
        }
        prefix_sum += arr[i];

        if prefix_sum > a {
            return "NO";
        }
    }

    "YES"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_sum_less_than_k() {
        assert_eq!(maximum_sum_less_than_k(2, 1, 7), "YES");
        assert_eq!(maximum_sum_less_than_k(2, 1, 1), "NO");
        assert_eq!(maximum_sum_less_than_k(2, 3, 15), "YES");
        assert_eq!(maximum_sum_less_than_k(3, 3, 6), "NO");
        assert_eq!(maximum_sum_less_than_k(4, 4, 5), "NO");
        assert_eq!(maximum_sum_less_than_k(5, 4, 7), "YES");
        assert_eq!(maximum_sum_less_than_k(4, 88, 1000000000), "NO");
        assert_eq!(maximum_sum_less_than_k(25, 69, 231), "NO");
        assert_eq!(maximum_sum_less_than_k(13, 97, 18806), "NO");
    }
}
