use std::{collections::HashMap, iter::FromIterator};

pub fn calculation(input: &str, reverse: bool) -> String {
    let mut result = String::new();
    for i in 0..input.lines().next().unwrap().len() {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        for line in input.lines() {
            let ch = &(line.as_bytes()[i] as char);
            if char_map.contains_key(ch) {
                char_map.insert(*ch, char_map[ch] + 1);
            } else {
                char_map.insert(*ch, 1);
            }
        }
        let mut v = Vec::from_iter(char_map.iter());
        v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        if reverse {
            v.reverse();
        }
        result.push(*v[0].0);
    }
    result
}

pub fn part_1(input: &str) -> String {
    calculation(input, false)
}

pub fn part_2(input: &str) -> String {
    calculation(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_case = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        assert_eq!(part_1(&test_case), "easter");
    }

    #[test]
    fn test_2() {
        let test_case = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        assert_eq!(part_2(&test_case), "advent");
    }
}
