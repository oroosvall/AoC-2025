use std::collections::{HashMap, HashSet};

type Coord = (u64, u64, u64);

fn to_coord(s: &str) -> Coord {
    let p: Vec<_> = s.split(',').collect();
    (
        p[0].parse().unwrap(),
        p[1].parse().unwrap(),
        p[2].parse().unwrap(),
    )
}

fn dist2(c1: &Coord, c2: &Coord) -> u64 {
    let x = c2.0 - c1.0;
    let y = c2.1 - c1.1;
    let z = c2.2 - c1.2;
    x.pow(2) + y.pow(2) + z.pow(2)
}

fn distances(coords: &Vec<Coord>) -> HashMap<u64, Vec<(&Coord, &Coord)>> {
    let mut map: HashMap<u64, Vec<(&Coord, &Coord)>> = HashMap::new();

    for i in 0..coords.len() - 1 {
        for j in (i + 1)..coords.len() {
            let p1 = &coords[i];
            let p2 = &coords[j];
            let dist = dist2(p1, p2);
            map.entry(dist)
                .and_modify(|e| e.push((p1, p2)))
                .or_insert(vec![(p1, p2)]);
        }
    }

    map
}

fn insert_into_sets(sets: &Vec<HashSet<Coord>>, link: (&Coord, &Coord)) -> Vec<HashSet<Coord>> {
    let mut new_sets = Vec::new();

    let mut set = HashSet::new();
    set.insert(link.0.clone());
    set.insert(link.1.clone());

    for s in sets {
        let mut added = false;
        if s.contains(link.0) {
            for p in s {
                set.insert(p.clone());
            }
            added = true;
        }
        if s.contains(link.1) {
            for p in s {
                set.insert(p.clone());
            }
            added = true;
        }

        if !added {
            new_sets.push(s.clone());
        }
    }

    new_sets.push(set);

    new_sets
}

pub fn part_1(vec: &Vec<String>) -> usize {
    let coords: Vec<_> = vec.iter().map(|pos| to_coord(&pos)).collect();
    let map = distances(&coords);
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();

    let mut sets: Vec<HashSet<Coord>> = Vec::new();

    for i in 0..coords.len() {
        // println!("{} -> {:?}", keys[i], map[keys[i]]);
        sets = insert_into_sets(&sets, map[keys[i]][0]); // assumtion that there always is only pair with the same distance
    }

    sets.sort_by(|a, b| b.len().cmp(&a.len()));

    sets.iter().take(3).map(|s| s.len()).product()
}

pub fn part_2(vec: &Vec<String>) -> u64 {
    let coords: Vec<_> = vec.iter().map(|pos| to_coord(&pos)).collect();
    let map = distances(&coords);
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();

    let mut sets: Vec<HashSet<Coord>> = Vec::new();

    let mut i = 0;
    let d;
    loop {
        let link = map[keys[i]][0];
        sets = insert_into_sets(&sets, link);
        if sets[0].len() == coords.len() {
            d = link.0.0 * link.1.0;
            break;
        }
        i += 1;
    }
    d
}
