trait DigitIter {
    fn iter_digits(&self) -> DigitIterImpl;
}

struct DigitIterImpl {
    n: usize,
}

impl Iterator for DigitIterImpl {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            let digit = self.n % 10;
            self.n /= 10;
            Some(digit as u8)
        }
    }
}

impl DigitIter for usize {
    fn iter_digits(&self) -> DigitIterImpl {
        DigitIterImpl { n: *self }
    }
}

fn init() -> Vec<usize> {
    let ten_pow_5 = 10_usize.pow(5);
    let multiplier = 2 * ten_pow_5;
    let mut dp: Vec<usize> = Vec::with_capacity(multiplier + 1);
    dp.push(0);

    // use `iter_digits` to iterate over digits

    for n in 1..=multiplier {
        let mut sum = 0;

        for digit in (n / 10).iter_digits() {
            sum += digit as usize;
        }

        // Calculate and push using array indexing
        dp.push(dp.last().unwrap() + sum + n % 10);
    }

    dp
}

fn vlad_and_sum_of_digits(n: usize, dp: &Vec<usize>) -> usize {
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlad_and_sum_of_digits() {
        let dp = init();

        assert_eq!(vlad_and_sum_of_digits(12, &dp), 51);
        assert_eq!(vlad_and_sum_of_digits(1, &dp), 1);
        assert_eq!(vlad_and_sum_of_digits(2, &dp), 3);
        assert_eq!(vlad_and_sum_of_digits(3, &dp), 6);
        assert_eq!(vlad_and_sum_of_digits(1434, &dp), 18465);
        assert_eq!(vlad_and_sum_of_digits(2024, &dp), 28170);
        assert_eq!(vlad_and_sum_of_digits(200000, &dp), 4600002);
    }
}
