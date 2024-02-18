enum Cell {
    Thorn,
    Coin,
    Empty,
}

fn get_cell_type(c: char) -> Cell {
    match c {
        '.' => Cell::Empty,
        '@' => Cell::Coin,
        '*' => Cell::Thorn,
        _ => panic!("Invalid character"),
    }
}

fn thorns_and_coins(path: Vec<char>, n: u32) -> u32 {
    let mut coins = 0;
    let mut i = 0;

    while i < n - 1 {
        match get_cell_type(path[(i + 1) as usize]) {
            Cell::Coin => {
                coins += 1;
                i += 1;
            }
            Cell::Empty => {
                if i + 2 < n {
                    match get_cell_type(path[(i + 2) as usize]) {
                        Cell::Coin => {
                            coins += 1;
                            i += 2;
                        }
                        Cell::Empty => {
                            i += 2;
                        }
                        Cell::Thorn => {
                            i += 1;
                        }
                    }
                } else {
                    i += 1;
                }
            }
            Cell::Thorn => {
                if i + 2 < n {
                    match get_cell_type(path[(i + 2) as usize]) {
                        Cell::Coin => {
                            coins += 1;
                            i += 2;
                        }
                        Cell::Empty => {
                            i += 2;
                        }
                        Cell::Thorn => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    coins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thorns_and_coins() {
        let str1 = ".@@*@.**@@".to_string();
        let path1: Vec<char> = str1.chars().collect();
        assert_eq!(thorns_and_coins(path1, 10), 3);

        let str2 = ".@@@@".to_string();
        let path2: Vec<char> = str2.chars().collect();
        assert_eq!(thorns_and_coins(path2, 5), 4);

        let str3 = ".@@..@***..@@@*".to_string();
        let path3: Vec<char> = str3.chars().collect();
        assert_eq!(thorns_and_coins(path3, 15), 3);
    }
}
