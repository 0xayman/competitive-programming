fn vlad_and_the_best_of_five(str: &str) -> char {
    let mut winner = 'A';
    let mut count = 0;

    for c in str.chars() {
        match c {
            'A' => count += 1,
            'B' => count -= 1,
            _ => continue, // Skip other characters
        }

        winner = if count > 0 { 'A' } else { 'B' };
    }

    winner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlad_and_the_best_of_five() {
        assert_eq!(vlad_and_the_best_of_five("ABABB"), 'B');
        assert_eq!(vlad_and_the_best_of_five("ABABA"), 'A');
        assert_eq!(vlad_and_the_best_of_five("BBBAB"), 'B');
        assert_eq!(vlad_and_the_best_of_five("AAAAA"), 'A');
        assert_eq!(vlad_and_the_best_of_five("BBBBB"), 'B');
        assert_eq!(vlad_and_the_best_of_five("BABAA"), 'A');
        assert_eq!(vlad_and_the_best_of_five("AAAAB"), 'A');
        assert_eq!(vlad_and_the_best_of_five("BAAAA"), 'A');
    }
}
