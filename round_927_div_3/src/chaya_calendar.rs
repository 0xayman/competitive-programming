fn next_multiple(x: u32, n: u32) -> u32 {
    if x == 0 || n == 0 {
        panic!("x and n must be positive integers.");
    }
    n + (x - n % x)
}

fn chaya_calendar(years: Vec<u32>, n: u32) -> u32 {
    let mut apoc = years[0];

    for i in 1..n as usize {
        if years[i] > apoc {
            apoc += years[i] - apoc;
        } else if years[i] == apoc {
            apoc += years[i]
        } else if years[i] < apoc {
            let next_apoc = next_multiple(years[i], apoc);
            apoc += next_apoc - apoc;
        }
    }

    apoc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaya_calendar() {
        assert_eq!(chaya_calendar(vec![3, 2, 4, 5, 9, 18], 6), 36);
        assert_eq!(chaya_calendar(vec![1, 2, 3, 4, 5], 5), 5);
        assert_eq!(chaya_calendar(vec![1, 1, 1, 1, 1], 5), 5);
        assert_eq!(chaya_calendar(vec![50, 30, 711, 200, 503, 1006], 6), 2012);
    }
}
