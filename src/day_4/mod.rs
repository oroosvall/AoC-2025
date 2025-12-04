fn is_in_grid(pos: (i32, i32), size: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < size.0 && pos.1 >= 0 && pos.1 < size.1
}

fn to_idx(pos: (usize, usize), stride: usize) -> usize {
    pos.1 * stride + pos.0
}

fn count_neigh(grid: &Vec<char>, pos: (usize, usize), size: (usize, usize)) -> i32 {
    let poses = [
        (pos.0 as i32 - 1, pos.1 as i32),
        (pos.0 as i32 + 1, pos.1 as i32),
        (pos.0 as i32, pos.1 as i32 - 1),
        (pos.0 as i32, pos.1 as i32 + 1),
        (pos.0 as i32 - 1, pos.1 as i32 - 1),
        (pos.0 as i32 - 1, pos.1 as i32 + 1),
        (pos.0 as i32 + 1, pos.1 as i32 - 1),
        (pos.0 as i32 + 1, pos.1 as i32 + 1),
    ];

    let mut neigh_count = 0;
    for p in poses {
        if is_in_grid(p, (size.0 as i32, size.1 as i32)) {
            let idx = to_idx((p.0 as usize, p.1 as usize), size.0);
            if grid[idx] != '.' {
                neigh_count += 1;
            }
        }
    }

    neigh_count
}

fn accecible(grid: &Vec<char>, size: (usize, usize)) -> Vec<usize> {
    let mut idxs = vec![];
    for y in 0..size.1 {
        for x in 0..size.0 {
            let idx = to_idx((x, y), size.0);
            if grid[idx] == '@' {
                if count_neigh(grid, (x, y), size) < 4 {
                    idxs.push(idx);
                }
            }
        }
    }

    idxs
}

pub fn part_1(vec: &Vec<String>) -> i32 {
    let y = vec.len();
    let x = vec[0].len();
    let grid: Vec<char> = vec.iter().flat_map(|s| s.chars()).collect();
    accecible(&grid, (x, y)).len() as i32
}

pub fn part_2(vec: &Vec<String>) -> i32 {
    let y = vec.len();
    let x = vec[0].len();
    let mut grid: Vec<char> = vec.iter().flat_map(|s| s.chars()).collect();
    let mut removed = 0;
    loop {
        let idxs = accecible(&grid, (x, y));
        removed += idxs.len();
        if idxs.is_empty() {
            break;
        }
        for i in idxs {
            grid[i] = '.';
        }
    }
    removed as i32
}
