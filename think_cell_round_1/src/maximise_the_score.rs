pub fn maximise_the_score(n: u32, arr: Vec<u32>) -> u32 {
    let mut arr = arr;
    arr.sort();
    arr.reverse();

    let mut sum = 0;
    for i in 0..n * 2 {
        if i % 2 != 0 {
            sum += arr[i as usize];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximise_the_score() {
        assert_eq!(maximise_the_score(1, vec![2, 3]), 2);
        assert_eq!(maximise_the_score(2, vec![1, 1, 2, 1]), 2);
        assert_eq!(maximise_the_score(3, vec![1, 1, 1, 1, 1, 1]), 3);
    }
}
