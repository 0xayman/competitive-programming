use std::collections::HashMap;
trait Not {
    fn not(&self) -> Self;
}

impl Not for String {
    fn not(&self) -> Self {
        let mut not_str = String::new();
        for c in self.chars() {
            if c == '0' {
                not_str.push('1');
            } else {
                not_str.push('0');
            }
        }
        not_str
    }
}

fn vlad_and_division(arr: Vec<u32>) -> u32 {
    let mut hashmap: HashMap<String, u32> = HashMap::new();
    let mut max_groups = arr.len();

    for num in &arr {
        let bin_num = format!("{:031b}", num);
        let not_bin_num = bin_num.not();

        *hashmap.entry(not_bin_num.clone()).or_insert(0) += 1;

        if hashmap.contains_key(&bin_num) && hashmap[&bin_num] > 0 && hashmap[&not_bin_num] > 0 {
            max_groups -= 1;

            *hashmap.get_mut(&bin_num).unwrap() -= 1;
            *hashmap.get_mut(&not_bin_num).unwrap() -= 1;
        }
    }

    max_groups as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlad_and_division() {
        assert_eq!(vlad_and_division(vec![1, 4, 3, 4]), 4);
        assert_eq!(vlad_and_division(vec![0, 2147483647]), 1);
        assert_eq!(
            vlad_and_division(vec![
                476319172, 261956880, 2136179468, 1671164475, 1885526767
            ]),
            3
        );
        assert_eq!(
            vlad_and_division(vec![1335890506, 811593141, 1128223362]),
            2
        );
        assert_eq!(
            vlad_and_division(vec![688873446, 627404104, 1520079543, 1458610201]),
            2
        );
        assert_eq!(
            vlad_and_division(vec![61545621, 2085938026, 1269342732, 1430258575]),
            3
        );
        assert_eq!(vlad_and_division(vec![0, 0, 2147483647, 2147483647]), 2);
        assert_eq!(vlad_and_division(vec![0, 0, 2147483647]), 2);
        assert_eq!(
            vlad_and_division(vec![
                1858058912, 289424735, 1858058912, 2024818580, 1858058912, 289424735, 122665067,
                289424735
            ]),
            4
        );
    }
}
