fn is_tls(input: &str) -> bool {
    let windows = input.as_bytes().windows(4);
    for win in windows {
        if (win[0] == win[3]) && (win[1] == win[2]) && (win[0] != win[1]) {
            return true;
        }
    }
    false
}

pub fn part_1(input: &str) -> String {
    let mut tls_count = 0;
    for line in input.lines() {
        let mut tokens = line.split(&['[', ']'][..]);
        let before_hypernet = tokens.next().unwrap();
        let hypernet = tokens.next().unwrap();
        let after_hypernet = tokens.next().unwrap();
        if !is_tls(hypernet) && (is_tls(before_hypernet) || is_tls(after_hypernet)) {
            tls_count += 1;
        }
    }
    tls_count.to_string()
}

pub fn part_2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_case = "acca[mnop]qrst\nabba[mnop]qrst\nabcd[bddb]xyyx\naaaa[qwer]tyui\nioxxoj[asdfgh]zxcvbn\nmacca[amnnmt]qrst\n";
        assert_eq!(part_1(&test_case), "3");
    }

    #[test]
    fn test_2() {
        let test_case = "asdatazcba[mntmn]asdatabbaqrssadt\n";
        assert_eq!(part_1(&test_case), "1");
    }
}
