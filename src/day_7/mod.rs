use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_1(vec: &Vec<String>) -> u64 {
    let v: Vec<_> = vec
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect();
    let mut deq = VecDeque::from_iter(v);

    let mut lines = HashSet::new();
    lines.insert(deq.pop_front().unwrap().find('S').unwrap());

    let mut splits = 0;

    for row in deq {
        let mut new_lines = HashSet::new();
        for l in lines {
            if row.chars().nth(l).unwrap() == '^' {
                new_lines.insert(l - 1);
                new_lines.insert(l + 1);
                splits += 1;
            } else {
                new_lines.insert(l);
            }
        }
        lines = new_lines;
    }

    splits
}

pub fn part_2(vec: &Vec<String>) -> usize {
    let v: Vec<_> = vec
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect();
    let mut deq = VecDeque::from_iter(v);

    let mut lines = HashMap::new();
    lines.insert(deq.pop_front().unwrap().find('S').unwrap(), 1);

    for row in deq {
        let mut new_lines = HashMap::new();
        for (l, tl) in lines {
            if row.chars().nth(l).unwrap() == '^' {
                new_lines
                    .entry(l - 1)
                    .and_modify(|e| *e += tl)
                    .or_insert(tl);
                new_lines
                    .entry(l + 1)
                    .and_modify(|e| *e += tl)
                    .or_insert(tl);
                // splits += 1;
            } else {
                new_lines.entry(l).and_modify(|e| *e += tl).or_insert(tl);
            }
        }
        lines = new_lines;
    }

    // splits
    let mut timelines = 0;
    for t in lines {
        timelines += t.1;
    }
    timelines
}
