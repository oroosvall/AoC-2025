fn invalid_in_even_range(start: &str, end: &str) -> Vec<i64> {
    let mut v = Vec::new();

    let middle = start.len() / 2;

    let rs: i64 = start.parse().unwrap();
    let re: i64 = end.parse().unwrap();

    let (r1_lower, _r2_lower) = start.split_at(middle);
    let (r1_upper, _r2_upper) = end.split_at(middle);

    let r1_l: i64 = r1_lower.parse().unwrap_or(start.parse().unwrap());
    let r1_u: i64 = r1_upper.parse().unwrap_or(end.parse().unwrap());

    // println!("{}, {}", r1_l, r1_u);

    for i in r1_l..(r1_u + 1) {
        let new_nr: i64 = format!("{}{}", i, i).parse().unwrap();
        if new_nr >= rs && new_nr <= re {
            v.push(new_nr);
        }
    }

    return v;
}

fn invalid_ids(s: &str) -> Vec<i64> {
    let r: Vec<&str> = s.split('-').collect();

    let start = r[0];
    let end = r[1];

    // Even lenght e.g 11 2222
    if start.len() == end.len() && start.len() % 2 == 0 {
        let re = invalid_in_even_range(start, end);
        // println!("{} -> {:?}", s, re);
        return re;
    }
    // Odd length e.g 333 99999, is valid but should be ignored in part 1
    else if start.len() == end.len() && start.len() % 2 == 1 {
        return vec![];
    }
    // One even and one odd
    else {
        let slen = start.len();
        let elen = end.len();

        let lut = ["10", "99", "1000", "9999", "100000", "999999"];

        // start is even
        if slen % 2 == 0 {
            let e1 = lut[elen - 2];
            return invalid_in_even_range(start, e1);
        } else {
            let s1 = lut[slen - 1];
            return invalid_in_even_range(s1, end);
        }

        // println!("{:?} is not same length", r)
        // return vec![];
    }
}

pub fn part_1(vec: &Vec<String>) -> i64 {
    let ranges: Vec<&str> = vec.iter().flat_map(|f| f.split(',')).collect();
    ranges.iter().flat_map(|r| invalid_ids(&r)).sum()
}

pub fn part_2(vec: &Vec<String>) -> &str {
    let _ranges: Vec<&str> = vec.iter().flat_map(|f| f.split(',')).collect();
    "Not solved"
}
