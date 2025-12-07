pub fn part_1(vec: &Vec<String>) -> u64 {
    let mut items: Vec<Vec<&str>> = vec
        .iter()
        .map(|l| l.split_ascii_whitespace().into_iter().collect())
        .collect();

    let ops = items.pop().unwrap();

    let mut sum = 0;

    for (idx, op) in ops.iter().enumerate() {
        let mut nums = vec![];
        for i in &items {
            let n: u64 = i[idx].parse().unwrap();
            nums.push(n);
        }
        if *op == "*" {
            sum += nums.iter().product::<u64>();
        } else {
            sum += nums.iter().sum::<u64>();
        }
    }
    sum
}

pub fn part_2(vec: &Vec<String>) -> u64 {
    let mut items: Vec<&String> = vec.iter().map(|l| l).collect();

    let ops = items.pop().unwrap();
    let mut op = ' ';

    let mut sum = 0;

    let mut nums: Vec<u64> = vec![];

    // for i in &items {
    //     println!("{:?}", i);
    // }

    for (idx, o) in ops.chars().enumerate() {
        if o != ' ' {
            op = o;
        }
        let mut n = String::new();
        for i in &items {
            let ch = i.chars().nth(idx).unwrap();
            if ch != ' ' {
                n.push(ch);
            }
        }
        if n.len() != 0 {
            nums.push(n.parse::<u64>().unwrap());
        } else {
            // println!("{:?}, {}", nums, op);
            if op == '*' {
                sum += nums.iter().product::<u64>();
            } else if op == '+' {
                sum += nums.iter().sum::<u64>();
            } else {
                panic!("No op!!!");
            }
            nums = vec![];
            op = ' ';
        }
    }

    // println!("{:?}, {}", nums, op);
    if op == '*' {
        sum += nums.iter().product::<u64>();
    } else if op == '+' {
        sum += nums.iter().sum::<u64>();
    } else {
        panic!("No op!!!");
    }

    sum
}
