use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

struct Sensor {
    pos: (i32, i32),
    beacon_dist: i32,
}

fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn intersect(a: &Range<i32>, b: &Range<i32>) -> Range<i32> {
    max(a.start, b.start)..min(a.end, b.end)
}

fn merge(a: &Range<i32>, b: &Range<i32>) -> Range<i32> {
    let start = min(a.start, b.start);
    let end = max(a.end, b.end);
    start..end
}

/// Given a vector of ranges, returns a vector of non-overlapping ranges whose union is identical
/// to that of the input.
fn merge_many(ranges: &mut Vec<Range<i32>>) -> Vec<Range<i32>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut out_ranges: Vec<Range<i32>> = vec![];
    for range in ranges {
        if let Some(last) = out_ranges.last_mut() {
            if intersect(range, last).len() > 0 {
                *last = merge(last, range);
                continue;
            }
        }

        out_ranges.push(range.clone());
    }
    out_ranges
}

fn draw_region(
    sensors: &Vec<Sensor>,
    beacons: &Vec<(i32, i32)>,
    xlim: (i32, i32),
    ylim: (i32, i32),
) {
    for y in ylim.0..ylim.1 {
        print!("{:04} ", y);
        'outer: for x in xlim.0..xlim.1 {
            for beacon in beacons {
                if *beacon == (x, y) {
                    print!("B");
                    continue 'outer;
                }
            }
            for sensor in sensors {
                if sensor.pos == (x, y) {
                    print!("S");
                    continue 'outer;
                }
                if manhattan(sensor.pos, (x, y)) <= sensor.beacon_dist {
                    print!("#");
                    continue 'outer;
                }
            }
            print!(".");
        }
        print!("\n")
    }
}

fn main() {
    lazy_static! {
        static ref SENS_RE: Regex =
            Regex::new(r"x=(-?\d+), y=(-?\d+).+x=(-?\d+), y=(-?\d+)").unwrap();
    }
    // Init input reader
    let file = File::open(r".\src\data\day15_1.txt").unwrap();
    let filebuf = BufReader::new(file);
    let all_lines: Vec<String> = filebuf.lines().map(|x| x.unwrap()).collect();
    //
    let mut sensors: Vec<Sensor> = vec![];
    let mut beacons: Vec<(i32, i32)> = vec![];
    for line in all_lines {
        let captures = SENS_RE.captures(&line[..]).unwrap();
        let pos: (i32, i32) = (
            captures[1].parse::<i32>().unwrap(),
            captures[2].parse::<i32>().unwrap(),
        );
        let beacon_pos: (i32, i32) = (
            captures[3].parse::<i32>().unwrap(),
            captures[4].parse::<i32>().unwrap(),
        );
        sensors.push(Sensor {
            pos,
            beacon_dist: manhattan(pos, beacon_pos),
        });
        beacons.push(beacon_pos);
    }

    // Sort beacons on the x coordinate and find the furthest left and right
    sensors.sort_by(|a, b| (a.pos.0 - a.beacon_dist).cmp(&(b.pos.0 - b.beacon_dist)));
    let xmin = sensors[0].pos.0 - sensors[0].beacon_dist;
    sensors.sort_by(|a, b| (a.pos.0 + a.beacon_dist).cmp(&(b.pos.0 + b.beacon_dist)));
    let xmax = sensors[sensors.len() - 1].pos.0 + sensors[sensors.len() - 1].beacon_dist;
    println!("xmin: {}, xmax: {}", xmin, xmax);
    sensors.sort_by(|a, b| (a.pos.1 - a.beacon_dist).cmp(&(b.pos.1 - b.beacon_dist)));
    let ymin = sensors[0].pos.1 - sensors[0].beacon_dist;
    sensors.sort_by(|a, b| (a.pos.1 + a.beacon_dist).cmp(&(b.pos.1 + b.beacon_dist)));
    let ymax = sensors[sensors.len() - 1].pos.1 + sensors[sensors.len() - 1].beacon_dist;
    println!("ymin: {}, ymax: {}", ymin, ymax);

    // draw_region(&sensors, &beacons, (xmin, xmax+1), (ymin, ymax+1));

    // Part 1, count the blocked squares on probe row
    // Check the row y=2000000
    let probe_row = 2000000;
    let ranges = scanned_on_row(&sensors, probe_row);
    let mut beacons_in_probe_row: Vec<(i32, i32)> = vec![];
    let mut accumulator: i32 = 0;
    for range in ranges {
        accumulator += range.len() as i32;
    }
    let mut beacons_in_probe_row: Vec<(i32, i32)> = vec![];
    for beacon in beacons.iter() {
        if !beacons_in_probe_row.contains(&beacon) && beacon.1 == probe_row {
            beacons_in_probe_row.push(*beacon);
        }
    }
    accumulator -= beacons_in_probe_row.len() as i32;
    println!("{}", accumulator);

    // Part 2, find unoccupied square in region (0..4000000, 0..4000000)
    for probe_row in 0..4000000 {
        let ranges = scanned_on_row(&sensors, probe_row);

        let mut accumulator: i32 = 0;
        for i in 0..ranges.len() {
            let range_a = &ranges[i];
            accumulator += intersect(range_a, &(0..4000000)).len() as i32;
        }

        if accumulator != 4000000 {
            println!(
                "{}",
                ranges[0].end as i64 * 4000000 as i64 + probe_row as i64
            );
            break;
        }
    }
}

/// Find the squares scanned on the probe_row, returned as a vector of non-overlapping ranges
/// whose union is the set of all squares scanned.
fn scanned_on_row(sensors: &Vec<Sensor>, probe_row: i32) -> Vec<Range<i32>> {
    // for each sensor, get the xrange occupied by its scan radius on the probe row
    let mut ranges: Vec<Range<i32>> = vec![];
    for sensor in sensors.iter() {
        let row_dist: i32 = (probe_row - sensor.pos.1).abs(); // how many rows from sensor to probe?
        let scan_width: i32 = sensor.beacon_dist - row_dist; // how long is the chord formed on this row?
        if scan_width < 0 {
            // Sensor doesn't scan this row at all
            continue;
        }
        ranges.push(sensor.pos.0 - scan_width..sensor.pos.0 + scan_width + 1);
    }
    // merge overlapping ranges
    ranges = merge_many(&mut ranges);
    ranges
}
