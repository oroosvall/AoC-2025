
pub fn part_1(vec :&Vec<String>) -> i32
{
    let mut hit_zero = 0;
    let mut dial :i16 = 50;
    
    for l in vec {
        let (d, c) = l.split_at(1);
        let nr : i16 = c.parse().unwrap();

        if d == "L" {
            dial -= nr;
            while dial < 0 {
                dial += 100
            }
        } else {
            dial += nr;
            while dial >= 100 {
                dial -= 100
            }
        }

        if dial == 0 {
            hit_zero += 1;
        }
    }

    hit_zero
}

pub fn part_2(vec :&Vec<String>) -> i32
{
    let mut hit_zero = 0;
    let mut dial :i16 = 50;
    
    for l in vec {
        let (d, c) = l.split_at(1);
        let nr : i16 = c.parse().unwrap();

        if d == "L" {
            for _ in 0..nr {
                dial -= 1;
                if dial == 0 {
                    hit_zero += 1;
                }
                if dial < 0 {
                    dial = 99;
                }
            }

        } else {
            for _ in 0..nr {
                dial += 1;
                if dial == 100 {
                    dial = 0;
                }
                if dial == 0 {
                    hit_zero += 1;
                }
            }
        }
    }

    hit_zero
}

