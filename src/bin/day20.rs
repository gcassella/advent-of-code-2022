use std::fs;

struct Item {
    pos: usize,
    val: i64,
}

impl Item {
    pub(crate) fn clone(&self) -> Item {
        Item {
            pos: self.pos,
            val: self.val,
        }
    }
}

fn main() {
    let input = fs::read_to_string(r".\src\data\day20_1.txt").expect("Error opening file");
    let mut shift_list: Vec<Item> = input
        .split("\r\n")
        .enumerate() // sorry, unix users
        .map(|(i, x)| Item {
            pos: i,
            val: x.parse::<i64>().unwrap() * 811589153,
        })
        .collect();
    let n = shift_list.len();

    for _ in 0..10 {
        for i in 0..n {
            let old_pos = shift_list.iter().position(|x| x.pos == i).unwrap();
            let item = shift_list[old_pos].clone();
            if item.val == 0 {
                continue;
            }
            // Today I learned the remainder and modulo operations are different for negative
            // numbers
            let new_pos = (old_pos as i64 + item.val).rem_euclid(n as i64 - 1);
            let new_pos = new_pos as usize;
            shift_list.remove(old_pos);
            shift_list.insert(new_pos, item);
        }
    }

    // find zero
    let zero_pos = shift_list.iter().position(|x| x.val == 0).unwrap();
    let thou_pos = (zero_pos + 1000) % n;
    let twothou_pos = (zero_pos + 2000) % n;
    let threethou_pos = (zero_pos + 3000) % n;

    println!(
        "{} {} {}",
        shift_list[thou_pos].val, shift_list[twothou_pos].val, shift_list[threethou_pos].val
    );
    println!(
        "{}",
        shift_list[thou_pos].val + shift_list[twothou_pos].val + shift_list[threethou_pos].val
    );
}
