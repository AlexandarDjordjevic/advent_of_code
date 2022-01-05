use std::u32;

pub fn part_1(data: &str) -> String {
    data.lines()
        .filter(|line| {
            let lengths = line
                .split_ascii_whitespace()
                .filter(|c| c.len() > 0)
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (lengths[0] + lengths[1] > lengths[2])
                && (lengths[0] + lengths[2] > lengths[1])
                && (lengths[1] + lengths[2] > lengths[0])
        })
        .count()
        .to_string()
}

pub fn parse_line(line: &str) -> Vec<u32> {
    line.split_ascii_whitespace()
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

pub fn part_2(data: &str) -> String {
    let mut count = 0;
    let mut iter = data.lines().into_iter();
    while let (Some(l1), Some(l2), Some(l3)) = (iter.next(), iter.next(), iter.next()) {
        let c1 = parse_line(l1);
        let c2 = parse_line(l2);
        let c3 = parse_line(l3);
        for i in 0..3 {
            if (c1[i] + c2[i] > c3[i]) && (c1[i] + c3[i] > c2[i]) && (c2[i] + c3[i] > c1[i]) {
                count += 1;
            }
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_case = "    5   10   25\n    5   10   13\n   14   11    2\n11 3 9";
        assert_eq!(part_1(&test_case), "2");
    }

    #[test]
    fn test_2() {
        let test_case =
            "101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603";
        assert_eq!(part_2(&test_case), "6");
    }
}
