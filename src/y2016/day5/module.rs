use std::ops::Add;

pub fn part_1(input: &str) -> String {
    let mut result = String::new();
    let mut index = 0;
    while result.len() < 8 {
        let to_check = input.to_string().add(&index.to_string());
        let hash = md5::compute(to_check.as_bytes());
        if (hash[0] == 0) && (hash[1] == 0) && (hash[2] <= 0x0f) {
            match hash[2] {
                0x0a => result.push('a'),
                0x0b => result.push('b'),
                0x0c => result.push('c'),
                0x0d => result.push('d'),
                0x0e => result.push('e'),
                0x0f => result.push('f'),
                _ => result.push((hash[2] + 0x30) as char),
            }
        }
        index += 1;
    }
    result
}

pub fn part_2(input: &str) -> String {
    let mut result: [char; 8] = [0x0 as char; 8];
    let mut index = 0;
    let mut inserted = 0;
    while inserted < 8 {
        let to_check = input.to_string().add(&index.to_string());
        let hash = md5::compute(to_check.as_bytes());
        if (hash[0] == 0) && (hash[1] == 0) && (hash[2] < 0x08) {
            if result[hash[2] as usize] == 0 as char {
                match hash[3] & 0xf0 {
                    0xa0 => result[hash[2] as usize] = 'a',
                    0xb0 => result[hash[2] as usize] = 'b',
                    0xc0 => result[hash[2] as usize] = 'c',
                    0xd0 => result[hash[2] as usize] = 'd',
                    0xe0 => result[hash[2] as usize] = 'e',
                    0xf0 => result[hash[2] as usize] = 'f',
                    _ => result[hash[2] as usize] = (((hash[3] & 0xf0) >> 4) + 0x30) as char,
                }
                inserted += 1;
            }
        }
        index += 1;
    }
    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let door_id = "abc";
        assert_eq!(part_1(&door_id), "18f47a30");
    }

    #[test]
    fn test_2() {
        let test_case = "abc";
        assert_eq!(part_2(&test_case), "05ace8e3");
    }
}
