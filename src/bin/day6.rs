use std::{collections::VecDeque, env::args, fs};

/// This week was a fun one. The code I wrote for part 1 was trivially
/// capable of solving part 2 as well. I added the `message_len` command line
/// argument to use the same binary for solving both parts.
///
/// Part 1: `day6.exe 4`
/// Part 2: `day6.exe 14`
fn main() {
    let message_len = args()
        .nth(1)
        .expect("No message length given")
        .parse()
        .unwrap();

    let input = fs::read_to_string("day6_1.txt").unwrap();
    let mut buf: VecDeque<char> = VecDeque::from(vec![]);
    let mut total = 0;
    for c in input.chars() {
        println!("{}", buf.iter().collect::<String>());
        total += 1;
        if buf.contains(&c) {
            // Break off buffer at first occurence of c
            println!("*");
            while buf[0] != c {
                buf.pop_front();
            }
            buf.pop_front();
        };
        // Push c into the end of the buffer
        buf.push_back(c);
        if buf.len() >= message_len {
            // Start-of-packet marker detected
            println!("{} {}", buf.iter().collect::<String>(), total);
            break;
        }
    }
}
