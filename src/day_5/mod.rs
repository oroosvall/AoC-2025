use std::ops::RangeInclusive;

pub fn part_1(vec: &Vec<String>) -> i32 {
    let mut ranges: Vec<std::ops::RangeInclusive<u64>> = vec![];
    let mut fresh_counter = 0;
    let mut mode = false;
    for l in vec {
        if l.is_empty() {
            mode = true;
            remap_range(&mut ranges);
            continue;
        }
        if mode {
            let idx: u64 = l.parse().unwrap();
            for r in &ranges {
                if r.contains(&idx) {
                    fresh_counter += 1;
                    break;
                }
            }
        } else {
            let r: Vec<u64> = l.split('-').map(|ch| ch.parse().unwrap()).collect();
            ranges.push(r[0]..=r[1]);
        }
    }
    fresh_counter
}

fn remap_range(ranges: &mut Vec<RangeInclusive<u64>>) {
    // basically VecDeque
    ranges.sort_by(|a, b| b.start().cmp(a.start()));
    // ranges.reverse();

    let mut new_r = Vec::new();
    let mut r = ranges.pop().unwrap();

    while !ranges.is_empty() {
        let contender = ranges.pop().unwrap();
        if r.contains(contender.start()) {
            let start = *r.start();
            let end = *r.end().max(contender.end());
            r = start..=end;
        } else {
            new_r.push(r);
            r = contender;
        }
    }
    new_r.push(r);

    ranges.append(&mut new_r);
}

pub fn part_2(vec: &Vec<String>) -> u64 {
    let mut ranges: Vec<std::ops::RangeInclusive<u64>> = vec![];
    for l in vec {
        if l.is_empty() {
            break;
        }
        let r: Vec<u64> = l.split('-').map(|ch| ch.parse().unwrap()).collect();
        ranges.push(r[0]..=r[1]);
    }

    remap_range(&mut ranges);

    ranges.iter().map(|r| r.end() - r.start() + 1).sum()
}
