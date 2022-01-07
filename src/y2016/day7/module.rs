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
        let tokens = line.split(&['[', ']'][..]);
        let mut non_hypernets: Vec<&str> = Vec::new();
        let mut hypernets: Vec<&str> = Vec::new();
        let mut flag_hypernets = false;
        for token in tokens {
            if flag_hypernets {
                hypernets.push(token);
                flag_hypernets = false;
            } else {
                non_hypernets.push(token);
                flag_hypernets = true;
            }
        }

        let mut has_abba_in_hypernets = false;
        for hypernet in hypernets {
            if is_tls(hypernet) {
                has_abba_in_hypernets = true;
            }
        }
        if has_abba_in_hypernets {
            continue;
        }
        for non_hypernet in non_hypernets {
            if is_tls(non_hypernet) {
                tls_count += 1;
                break;
            }
        }
    }
    tls_count.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut tls_count = 0;
    for line in input.lines() {
        let tokens = line.split(&['[', ']'][..]);
        let mut non_hypernets: Vec<&str> = Vec::new();
        let mut hypernets: Vec<&str> = Vec::new();
        let mut flag_hypernets = false;
        for token in tokens {
            if flag_hypernets {
                hypernets.push(token);
                flag_hypernets = false;
            } else {
                non_hypernets.push(token);
                flag_hypernets = true;
            }
        }

        let mut has_abba_in_hypernets = false;
        for hypernet in hypernets {
            if is_tls(hypernet) {
                has_abba_in_hypernets = true;
            }
        }
        if has_abba_in_hypernets {
            continue;
        }
        for non_hypernet in non_hypernets {
            if is_tls(non_hypernet) {
                tls_count += 1;
                break;
            }
        }
    }
    tls_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_case = "pxfqsysywqfsmername[yfcktnozutkhniqyp]tjzzakrnlxrtscena[bitenzjdqfopqevroqo]zujogbgemdxiaven[dtxlpfkysfcivyrxqt]fsgjjgzltbnlvdojqvk";
        assert_eq!(part_1(&test_case), "1");
    }

    #[test]
    fn test_2() {
        let test_case = "asdatazcba[mntmn]asdatabbaqrssadt\n";
        assert_eq!(part_1(&test_case), "1");
    }
}
