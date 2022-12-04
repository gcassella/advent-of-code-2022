use std::fs;

fn main() {
    let input = fs::read_to_string("day4_1.txt").expect("Error opening file");
    let mut part1_accumulator = 0;
    let mut part2_accumulator = 0;

    for line in input.split("\r\n") {
        // split lines in the form "a,b-c,d" into [a, b] and [c, d]
        let ranges = line.split_once(",");
        let (first_range, second_range) = ranges.unwrap();

        let first_range: Vec<usize> = first_range
            .split('-')
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();
        let second_range: Vec<usize> = second_range
            .split('-')
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();
        // It feels like there should be clever ways to do these boolean
        // comparisons, but with such a limited input specification (we only
        // ever compare 4 numbers for even the most complicated input), it seems
        // unnecessary to optimize. All the other clever solutions I have seen
        // also do the same number of comparisons, just obfuscated under the
        // hood.
        if (first_range[0] <= second_range[0]) && (first_range[1] >= second_range[1]) {
            // --xxxxxxxxx--
            // ----xxxxx----
            part1_accumulator += 1;
        } else if (first_range[0] >= second_range[0]) && (first_range[1] <= second_range[1]) {
            // ----xxxxx----
            // --xxxxxxxxx--
            part1_accumulator += 1;
        }

        if (second_range[0] <= first_range[0]) && (first_range[0] <= second_range[1]) {
            part2_accumulator += 1;
        } else if (second_range[0] <= first_range[1]) && (first_range[1] <= second_range[1]) {
            part2_accumulator += 1;
        } else if (first_range[0] <= second_range[0]) && (second_range[0] <= first_range[1]) {
            part2_accumulator += 1;
        } else if (first_range[0] <= second_range[1]) && (second_range[1] <= first_range[1]) {
            println!("{:?} {:?} {}", first_range, second_range, part2_accumulator);
        }
    }
    println!("{}", part1_accumulator);
    println!("{}", part2_accumulator);
}
