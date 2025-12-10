#[derive(Debug)]
struct Machine {
    desired: String,
    buttons: Vec<Vec<i32>>,
    joltage: Vec<i32>,
}

impl Machine {
    fn new(l: &str) -> Self {
        let mut pat = l.split(' ');
        let tmp = pat.next().unwrap();
        let desired: String = tmp.chars().skip(1).take(tmp.len() - 2).collect();
        let mut buttons = vec![];
        for _ in 0..pat.clone().count() - 1 {
            let b = pat.next().unwrap();
            let nums: String = b.chars().skip(1).take(b.len() - 2).collect();
            let indices: Vec<i32> = nums.split(',').map(|part| part.parse().unwrap()).collect();
            buttons.push(indices);
        }
        let tmp = pat.next().unwrap();
        let nums: String = tmp.chars().skip(1).take(tmp.len() - 2).collect();
        let joltage = nums.split(',').map(|pat| pat.parse().unwrap()).collect();

        Machine {
            desired: desired,
            buttons: buttons,
            joltage: joltage,
        }
    }
}

fn initialize_machine(cur_states: &Vec<String>, buttons: &Vec<Vec<i32>>) -> Vec<String> {
    let mut new_states = vec![];

    for cs in cur_states {
        for b in buttons {
            let mut s: Vec<char> = cs.clone().chars().collect();
            for idx in b {
                let i = *idx as usize;
                if s[i] == '.' {
                    s[i] = '#';
                } else {
                    s[i] = '.';
                }
            }
            let ss: String = s.into_iter().collect();
            if !new_states.contains(&ss) {
                new_states.push(ss);
            }
        }
    }

    new_states
}

fn initialize_procedure(l: &str) -> i32 {
    let m = Machine::new(l);

    let empty: String = vec!['.'; m.desired.len()].into_iter().collect();
    let mut states = vec![empty];

    let mut depth = 0;
    loop {
        depth += 1;

        states = initialize_machine(&states, &m.buttons);
        let found = states.iter().find(|&st| *st == m.desired);
        if let Some(_s) = found {
            break;
        }
    }

    depth
}

fn next_combination(comb: &mut [i32]) -> bool {
    let i = comb.iter().rposition(|&v| v != 0).unwrap();
    if i == 0 {
        return false;
    }
    let v = comb[i];
    comb[i - 1] += 1;
    comb[i] = 0;
    comb[comb.len() - 1] = v - 1;
    true
}

fn is_button_available(i: usize, mask: u32) -> bool {
    mask & (1 << i) > 0
}

fn configure_machine(target: &Vec<i32>, available_buttons: u32, buttons: &Vec<Vec<i32>>) -> u64 {
    if target.iter().all(|v| *v == 0) {
        return 0;
    }
    
    let (idx, &min_non_zero) = target
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v > 0)
        .min_by_key(|&(i, &v)| {
            (
                // lowest number of buttons
                buttons
                    .iter()
                    .enumerate()
                    .filter(|&(j, b)| {
                        is_button_available(j, available_buttons) && b.contains(&(i as i32))
                    })
                    .count(),
                // highest joltage value (negative because we're using `min_by_key`)
                -(v as isize),
            )
        })
        .unwrap();

    let selected_buttons: Vec<_> = buttons
        .iter()
        .enumerate()
        .filter(|&(i, btn)| {
            is_button_available(i, available_buttons) && btn.contains(&(idx as i32))
        })
        .collect();

    let mut result = u64::MAX;
    // selected_buttons.len();

    if !selected_buttons.is_empty() {
        let mut to_press = vec![0; selected_buttons.len()];
        to_press[selected_buttons.len() - 1] = min_non_zero;

        let mut new_mask = available_buttons;
        for (i, _) in selected_buttons.iter() {
            new_mask &= !(1 << i);
        }

        loop {
            let mut can_count = true;
            let mut new_target = target.clone();

            'outer: for (&count, &btn) in to_press.iter().zip(&selected_buttons) {
                if count == 0 {
                    continue;
                }
                for &b in btn.1 {
                    let idx = b as usize;
                    if new_target[idx] >= count {
                        new_target[idx] -= count;
                    } else {
                        can_count = false;
                        break 'outer;
                    }
                }
            }

            if can_count {
                let r = configure_machine(&new_target, new_mask, buttons);
                if r != u64::MAX {
                    result = result.min(min_non_zero as u64 + r);
                }
            }

            if !next_combination(&mut to_press) {
                break;
            }
        }
    }

    result
}


fn configure_procedure(l: &str) -> u64 {
    let m = Machine::new(l);
    // println!("{:?}", m);

    configure_machine(&m.joltage, (1 << m.buttons.len()) - 1, &m.buttons)
}

pub fn part_1(vec: &Vec<String>) -> i32 {
    vec.iter().map(|l| initialize_procedure(l)).sum()
}

// Needed help from this
// https://github.com/michel-kraemer/adventofcode-rust/blob/main/2025/day10/src/main.rs
pub fn part_2(vec: &Vec<String>) -> u64 {
    vec.iter().map(|l| configure_procedure(l)).sum()
}
