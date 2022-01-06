use std::{collections::BTreeMap, iter::FromIterator};

pub struct EncryptedName<'a> {
    pub id: usize,
    pub data: &'a str,
    pub hash: &'a str,
}

impl<'a> EncryptedName<'a> {
    pub fn new(input: &'a str) -> Self {
        EncryptedName {
            id: input[input.len() - 10..input.len() - 7]
                .parse::<usize>()
                .unwrap(),
            data: &input[0..input.len() - 11],
            hash: &input[input.len() - 6..input.len() - 1],
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hash = String::new();
        let mut char_map: BTreeMap<char, u32> = BTreeMap::new();
        let tokens = self.data.split('-').collect::<Vec<&str>>();
        for token in tokens {
            for c in token.chars() {
                let mut cnt = 1;
                if char_map.contains_key(&c) {
                    cnt = char_map.get(&c).unwrap() + 1;
                }
                char_map.insert(c, cnt);
            }
        }

        let mut pairs = Vec::from_iter(char_map);
        pairs.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

        let pairs_of_interest = pairs.windows(5).nth(0).unwrap();
        for pair in pairs_of_interest {
            hash.push(pair.0);
        }
        hash
    }

    pub fn get_decrypted_data(&self) -> String {
        let mut decrypted = String::new();

        for c in self.data.chars() {
            match c {
                '-' => decrypted.push(' '),
                _ => {
                    let find_index = |c: &char| -> usize {
                        let mut index: usize = 0;
                        for char in "abcdefghijklmnopqrstuvwxyz".chars() {
                            if char == *c {
                                return index;
                            }
                            index += 1;
                        }
                        panic!("Unknown char!");
                    };
                    let find_char = |index: usize| -> char {
                        "abcdefghijklmnopqrstuvwxyz".chars().nth(index).unwrap()
                    };

                    let new_index = (find_index(&c) + self.id) % "abcdefghijklmnopqrstuvwxyz".len();
                    decrypted.push(find_char(new_index));
                }
            }
        }
        decrypted
    }
}

pub fn part_1(data: &str) -> String {
    let mut sum: u32 = 0;
    for line in data.lines() {
        let item = EncryptedName::new(line);
        if item.hash == item.calculate_hash() {
            sum += 1;
        }
    }
    sum.to_string()
}

pub fn part_2(data: &str) -> String {
    let mut room_id: usize = 0;
    for line in data.lines() {
        let item = EncryptedName::new(line);
        let encrypted_data = item.get_decrypted_data();
        if encrypted_data == "northpole object storage" {
            room_id = item.id;
        }
    }
    room_id.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_case = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]";
        assert_eq!(part_1(&test_case), "1514");
    }

    #[test]
    fn test_2() {
        let test_case = "bcfhvdczs-cpxsqh-ghcfous-324[chsfb]";
        assert_eq!(part_2(&test_case), "324");
    }
}
