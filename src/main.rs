use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    closest_x: i64,
    closest_y: i64,
    dist: i64,
}

impl Sensor {
    pub fn new(x: i64, y: i64, closest_x: i64, closest_y: i64) -> Sensor {
        Sensor {
            x,
            y,
            closest_x,
            closest_y,
            dist: manhattan_dist(x, y, closest_x, closest_y)
        }
    }

    pub fn no_beacon_range(&self, line_num: i64) -> Range<i64> {
        let line_dist = (line_num - self.y).abs();

        if line_dist > self.dist {
            return 0..0; // Empty range
        }

        let remaining_x = self.dist - line_dist;
        let left = self.x - remaining_x;
        let right = self.x + remaining_x + 1;

        left..right
    }
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sensor ({},{}) => closest ({},{}) dist={}",
            self.x, self.y, self.closest_x, self.closest_y, self.dist
        )
    }
}

fn manhattan_dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn no_distress_count(line: i64, sensors: &Vec<Sensor>) -> i64 {
    let mut unusable_count: i64 = 0;

    let mut beacons_on_line: Vec<(i64, i64)> = sensors
        .iter()
        .map(|s| (s.closest_x, s.closest_y))
        .filter(|&s| s.1 == line)
        .collect();
    beacons_on_line.dedup();
    let beacon_count: usize = beacons_on_line.len();

    let mut ranges: Vec<Range<i64>> = sensors.iter().map(|s| s.no_beacon_range(line)).collect();
    ranges.retain(|r| r.start != r.end);
    ranges.sort_by(|a, b| {
        a.start
            .cmp(&b.start)
            .then_with(|| (a.end - a.start).cmp(&(b.end - b.start)))
    });

    let range_len = ranges.len();
    let mut range_seen: i64 = i64::MIN;
    for n in 0..range_len {
        let curr = &ranges[n];
        let coverage = if curr.end < range_seen {
            0
        } else if curr.start > range_seen {
            curr.end - curr.start
        } else {
            curr.end - range_seen
        };

        println!(
            "> Range: {}..{} adds: {}",
            curr.start, curr.end, coverage
        );

        range_seen = cmp::max(range_seen, curr.end);
        unusable_count += coverage;
    }
    println!();
    println!(
        "Subtracting: {} - {}, unusable positions: {}",
        unusable_count,
        beacon_count,
        unusable_count - beacon_count as i64
    );
    println!();
    
    unusable_count -= beacon_count as i64;

    unusable_count
}

fn main() {
    let sensors = read_sensors("./Input.txt");
    let count = no_distress_count(2000000, &sensors);
    println!("Unusable positions: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_sensors<P>(filename: P) -> Vec<Sensor>
where
    P: AsRef<Path>,
{
    println!();
    
    let mut sensors = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        let mut n = 0;
        for line in lines {
            if let Ok(line_in) = line {
                let tokens: Vec<&str> = line_in.split("=").skip(1).collect();
                let nums: Vec<i64> = tokens
                    .iter()
                    .map(|token| {
                        let num_text: String = token
                            .chars()
                            .take_while(|&c| c >= '0' && c <= '9' || c == '-')
                            .collect();
                        num_text.parse::<i64>().unwrap()
                    })
                    .collect();
                let sensor: Sensor = Sensor::new(nums[0], nums[1], nums[2], nums[3]);
                println!("{})  {}", n, sensor);
                sensors.push(sensor);
                n += 1;
            }
        }
    }

    println!();
    
    sensors
}

// 6607798 too high
// 6697770 too high
// 4748135 right