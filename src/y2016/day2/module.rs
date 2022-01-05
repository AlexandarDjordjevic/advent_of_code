pub fn get_keycode(prev_code: u8, sequence: &str) -> u8 {
    let mut code = prev_code;
    for ch in sequence.chars() {
        match ch {
            'L' => match code {
                1 | 4 | 7 => (),
                _ => code = code - 1,
            },
            'R' => match code {
                3 | 6 | 9 => (),
                _ => code = code + 1,
            },
            'U' => match code {
                1 | 2 | 3 => (),
                _ => code = code - 3,
            },
            'D' => match code {
                7 | 8 | 9 => (),
                _ => code = code + 3,
            },
            _ => {
                panic!("Invalid input");
            }
        };
    }
    code
}

pub fn part_1(data: &str) -> String {
    let mut current_code: u8 = 5;
    let mut key_code = String::new();
    for line in data.lines() {
        current_code = get_keycode(current_code, &line);
        key_code.push((current_code + 0x30) as char);
    }
    key_code
}
/*
ULL RRDDD LURDL UUUUD
        1
      2 3 4
    5 6 7 8 9
      A B C
        D
*/
pub fn get_keycode_v2(prev_code: u8, sequence: &str) -> u8 {
    let mut code = prev_code;
    for ch in sequence.chars() {
        match ch {
            'L' => match code {
                1 | 2 | 5 | 0xA | 0xD => (),
                _ => code = code - 1,
            },
            'R' => match code {
                1 | 4 | 9 | 0xC | 0xD => (),
                _ => code = code + 1,
            },
            'U' => match code {
                1 | 2 | 4 | 5 | 9 => (),
                3 => code = 1,
                0xD => code = 0xB,
                _ => code = code - 4,
            },
            'D' => match code {
                5 | 0xA | 0xD | 0xC | 9 => (),
                1 => code = 3,
                0xB => code = 0xD,
                _ => code = code + 4,
            },
            _ => {
                panic!("Invalid input");
            }
        };
    }
    code
}

pub fn part_2(data: &str) -> String {
    let mut current_code: u8 = 5;
    let mut key_code = String::new();
    let to_str = |val: u8| match val {
        0xa => 'A',
        0xb => 'B',
        0xc => 'C',
        0xd => 'D',
        _ => (val + 0x30) as char,
    };
    for line in data.lines() {
        current_code = get_keycode_v2(current_code, &line);
        key_code.push(to_str(current_code));
    }

    key_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_seq = "ULL\nRRDDD\nLURDL\nUUUUD\n";
        let key_code = part_1(&test_seq);
        assert_eq!("1985", key_code);
    }

    #[test]
    fn test_2() {
        let test_seq = "ULL\nRRDDD\nLURDL\nUUUUD\n";
        let key_code = part_2(&test_seq);
        assert_eq!("5DB3", key_code);
    }
}
