fn lr_remainders(arr: &Vec<usize>, n: usize, m: usize, commands: &Vec<char>) -> Vec<usize> {
    let mut left = 0;
    let mut right = n - 1;

    let mut order: Vec<usize> = Vec::with_capacity(n);

    for i in 0..n {
        let command = commands[i];

        match command {
            'L' => {
                order.push(arr[left]);
                left += 1;
            }
            'R' => {
                order.push(arr[right]);
                right = right.wrapping_sub(1);
            }
            _ => panic!("Invalid command: {}", command),
        }
    }

    let mut rem: usize = 1;

    let mut res: Vec<usize> = Vec::with_capacity(n);

    for i in (0..n).rev() {
        rem = (rem * order[i]) % m;
        res.push(rem);
    }

    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_lr_remainders() {
        assert_eq!(
            lr_remainders(&vec![3, 1, 4, 2], 4, 6, &vec!['L', 'R', 'R', 'L']),
            vec![0, 2, 4, 1]
        );
        assert_eq!(
            lr_remainders(&vec![1, 1, 1, 1, 1], 5, 1, &vec!['L', 'L', 'L', 'L', 'L']),
            vec![0, 0, 0, 0, 0]
        );
        assert_eq!(
            lr_remainders(
                &vec![1, 2, 3, 4, 5, 6],
                6,
                8,
                &vec!['R', 'L', 'L', 'L', 'R', 'R']
            ),
            vec![0, 0, 0, 4, 4, 4]
        );
        assert_eq!(lr_remainders(&vec![10000], 1, 10000, &vec!['R']), vec![0]);
    }
}
