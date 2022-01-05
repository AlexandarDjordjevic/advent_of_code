use std::{collections::BTreeMap, iter::FromIterator};

pub fn check_hash(data: &str) {
    let collection = data.split('-').collect::<Vec<&str>>();
    //last one is hash with 1234[<hash>]
    let mut hash_with_id = collection.last().unwrap().split('[');
    let id = hash_with_id.next().unwrap();
    let tmp = hash_with_id.next().unwrap();
    //remove ] from hash
    let hash = &tmp[0..tmp.len() - 1];
    let mut char_map: BTreeMap<char, u32> = BTreeMap::new();
    for i in 0..collection.len() - 1 {
        let chars = collection[i].chars().collect::<Vec<_>>();
        for c in chars {
            let mut cnt = 1;
            if char_map.contains_key(&c) {
                cnt = char_map.get(&c).unwrap() + 1;
            }
            char_map.insert(c, cnt);
        }
    }

    let mut v = Vec::from_iter(char_map);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    let new_vec = v.windows(5);
    let mut calc_hash = String::new();
    for x in new_vec[0].unwrap() {
        calc_hash.push(x[0].0);
    }
    println!("ID: {}, Hash: {}, Calculated hash {}", id, hash, calc_hash);
}

pub fn part_1(data: &str) -> String {
    // data.lines().filter(|s| s.split('-'))
    "Test".to_owned()
}
pub fn part_2(data: &str) -> String {
    let count = 0;

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_case = "aaaaa-bbb-z-y-x-123[abxyz]\n
        a-b-c-d-e-f-g-h-987[abcde]\n
        not-a-real-room-404[oarel]\n
        totally-real-room-200[decoy]";
        check_hash("not-a-real-room-404[oarel]");
        // assert_eq!(part_1(&test_case), "1514");
    }

    #[test]
    fn test_2() {
        let test_case = "aaaaa-bbb-z-y-x-123[abxyz]\n
            a-b-c-d-e-f-g-h-987[abcde]\n
            not-a-real-room-404[oarel]\n
            totally-real-room-200[decoy]";
        assert_eq!(part_2(&test_case), "6");
    }
}
