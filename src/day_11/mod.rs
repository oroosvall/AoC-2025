use std::collections::HashMap;

fn to_map(vec: &Vec<String>) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();
    for l in vec {
        let parts: Vec<_> = l.split(':').collect();
        map.insert(parts[0], parts[1].split_whitespace().collect::<Vec<_>>());
    }
    map
}

fn unique_paths<'a>(
    map: &HashMap<&'a str, Vec<&'a str>>,
    cur: &'a str,
    to: &str,
    cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    if let Some(p) = cache.get(cur) {
        return *p;
    }

    if cur == to {
        return 1;
    }

    let mut cur_dist = 0;

    if map.contains_key(cur) {
        let itms = &map[cur];
        for &n in itms {
            cur_dist += unique_paths(map, n, to, cache);
        }
    }

    cache.insert(cur, cur_dist);
    cur_dist
}

pub fn part_1(vec: &Vec<String>) -> i64 {
    let map = to_map(vec);

    unique_paths(&map, "you", "out", &mut HashMap::default())
}

pub fn part_2(vec: &Vec<String>) -> i64 {
    let map = to_map(vec);

    //   srv -> dac -> fft -> out
    (unique_paths(&map, "svr", "dac", &mut HashMap::default())
        * unique_paths(&map, "dac", "fft", &mut HashMap::default())
        * unique_paths(&map, "fft", "out", &mut HashMap::default()))
    //   srv -> fft -> dac -> out
        +
        (unique_paths(&map, "svr", "fft", &mut HashMap::default())
            * unique_paths(&map, "fft", "dac", &mut HashMap::default())
            * unique_paths(&map, "dac", "out", &mut HashMap::default()))
}
