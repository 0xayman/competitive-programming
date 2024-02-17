fn permutation_printing(n: u32) -> Vec<u32> {
    let mut permutation: Vec<u32> = Vec::with_capacity(n as usize);

    for i in 1..=n / 2 {
        permutation.push(n - i + 1);
        permutation.push(i);
    }

    if n % 2 != 0 {
        permutation.push(n / 2 + 1);
    }

    permutation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_printing() {
        assert_eq!(permutation_printing(4), vec![4, 1, 3, 2]);
        assert_eq!(permutation_printing(3), vec![3, 1, 2]);
    }
}
