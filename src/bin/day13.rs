use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
};
use std::cmp::Ordering;

#[derive(Debug)]
enum PacketEntry {
    List(VecDeque<PacketEntry>),
    Integer(usize)
}


fn compare_packetentry(a: PacketEntry, b: PacketEntry) -> Option<bool> {
    match (a, b) {
        (PacketEntry::List(a_l), PacketEntry::List(b_l)) => compare_list_to_list(a_l, b_l),
        (PacketEntry::List(a_l), PacketEntry::Integer(b_i)) => compare_list_to_int(a_l, b_i),
        (PacketEntry::Integer(a_i), PacketEntry::List(b_l)) => compare_int_to_list(a_i, b_l),
        (PacketEntry::Integer(a_i), PacketEntry::Integer(b_i)) => compare_int_to_int(a_i, b_i)
    }
}

fn compare_int_to_int(a: usize, b: usize) -> Option<bool> {
    if a < b {
        Some(true)
    } else if a == b {
        None
    } else {
        Some(false)
    }
}


fn compare_int_to_list(a: usize, b: VecDeque<PacketEntry>) -> Option<bool> {
    let a_l = convert_int_to_list(a);
    compare_list_to_list(a_l, b)
}


fn compare_list_to_int(a: VecDeque<PacketEntry>, b: usize) -> Option<bool> {
    let b_l = convert_int_to_list(b);
    compare_list_to_list(a, b_l)
}


fn convert_int_to_list(a: usize) -> VecDeque<PacketEntry> {
    let a_i = PacketEntry::Integer(a);
    VecDeque::from(vec![a_i])
}


fn compare_list_to_list(mut a: VecDeque<PacketEntry>, mut b: VecDeque<PacketEntry>) -> Option<bool> {
    loop {
        // check we have items to compare
        if a.len() == 0 { // either a exhausted first or both exhausted simultaneously
            if b.len() == 0 { // both exhausted simultaneously
                return None
            }
            return Some(true)
        } else if b.len() == 0 { // b exhausted first
            return Some(false)
        }
        // compare items
        let a_item = a.pop_front().unwrap();
        let b_item = b.pop_front().unwrap();
        match compare_packetentry(a_item, b_item) {
            Some(out) => return Some(out),
            None => continue
        }
    }
}


fn parse_list_from_string(line: &String) -> PacketEntry {
    let mut out: VecDeque<PacketEntry> = VecDeque::from(vec![]);
    let chars = line.chars();
    let mut parsing_num = false; // flag to check if im parsing a number
    let mut current_num: usize = 0;

    let mut interior: Vec<char> = vec![];
    let mut consuming_interior = false;
    let mut hanging_brackets = 0;

    for char in chars {
        match char {
            '[' => {
                if !consuming_interior {
                    // consume until closing bracket and make recursive list
                    consuming_interior = true;
                    continue
                } else {
                    hanging_brackets += 1;
                }
            }
            ']' => {
                if consuming_interior {
                    if hanging_brackets == 0 { // we are closing the consumption
                        out.push_back(parse_list_from_string(&interior.iter().collect::<String>()));
                        consuming_interior = false;
                        interior = vec![];
                        continue
                    } else {
                        hanging_brackets -= 1;
                    }
                }
            },
            ',' => {
                if !consuming_interior {
                    if parsing_num {
                        parsing_num = false;
                        let packet_num = PacketEntry::Integer(current_num);
                        current_num = 0;
                        out.push_back(packet_num);
                    }
                }
            } // next element,
            _ => {
                if !consuming_interior {
                    if parsing_num {
                        current_num = current_num * 10 + char.to_digit(10).unwrap() as usize;
                    } else {
                        parsing_num = true;
                        current_num = char.to_digit(10).unwrap() as usize;
                    }
                }
            }// number
        }
        if consuming_interior {
            interior.push(char);
        }
    }

    if parsing_num {
        let packet_num = PacketEntry::Integer(current_num);
        out.push_back(packet_num);
    }

    PacketEntry::List(out)
}

fn main() {
    // Init input reader
    let file = File::open("day13_1.txt").unwrap();
    let filebuf = io::BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    // Split lines into pairs of packets
    let mut accumulator = 0;
    // Part 1
    for i in 0..(all_lines.len()+1) / 3 {
        let lines: Vec<&String> = all_lines[i*3..i*3+2].iter().collect();

        let l1 = parse_list_from_string(lines[0]);
        let l2 = parse_list_from_string(lines[1]);
        println!("{:?}", l1);
        println!("{:?}", l2);
        match compare_packetentry(l1, l2) {
            Some(b) => match b {
                true => { println!("Pair {} is in the right order", i+1); accumulator += i+1; },
                false => { println!("Pair {} is not in the right order", i+1) }
            },
            None => unreachable!()
        }
        println!("--------------");
    }
    println!("Final answer for part 1 is {}", accumulator);
    // Part 2
    let mut cleaned_lines: Vec<&String> = all_lines.iter().filter(|x| !x.is_empty()).collect();
    let indicator1 = String::from("[[2]]");
    let indicator2 = String::from("[[6]]");
    cleaned_lines.push(&indicator1);
    cleaned_lines.push(&indicator2);
    cleaned_lines.sort_by(|a, b| match compare_packetentry(parse_list_from_string(a), parse_list_from_string(b)) {
        Some(b) => match b {
            true => Ordering::Less,
            false => Ordering::Greater,
        },
        None => unreachable!()
    });

    for (i, line) in cleaned_lines.iter().enumerate() {
        if line == &&String::from("[[2]]") {
            println!("Indicator 1 at position {}", i+1);
        }
        if line == &&String::from("[[6]]") {
            println!("Indicator 2 at position {}", i+1);
        }
    }
}