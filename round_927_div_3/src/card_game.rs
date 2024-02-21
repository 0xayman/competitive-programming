#[derive(PartialEq)]
struct Card {
    suit: char,
    rank: char,
    trump: bool,
}

// implement custom greater than operator for Card
impl std::cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // if card is trump, it is always greater than other card
        if self.trump && !other.trump {
            return Some(std::cmp::Ordering::Greater);
        }
        // if both cards are trump, compare their ranks
        if self.trump && other.trump {
            return Some(self.rank.cmp(&other.rank));
        }
        // if both cards are not trump, if they are the same suit, compare their ranks
        if self.suit == other.suit {
            return Some(self.rank.cmp(&other.rank));
        }
        None
    }
}

fn card_game(n: u16, trump: char) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_game() {
        // assert_eq!();
    }
}
