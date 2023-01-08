use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

fn main() {
    let sensors = read_sensors("./Input.txt");
    let after_max_coord: i64 = 4000000;
    for n in 2639657..2639658 {
        let unusable_count = not_usable_count(n, &sensors, 0, after_max_coord);
        
        if unusable_count < after_max_coord {
            println!(" ==> {}: usable positions: {}", n, after_max_coord-unusable_count);
        }
        
        let x: i64 = 3435885;
        let y: i64 = 2639657;
        println!("Result: {}", x*after_max_coord+y);
    }    
}

fn not_usable_count(line: i64, sensors: &Vec<Sensor>, min_x: i64, after_max_x: i64) -> i64 {
    let mut unusable_count: i64 = 0;

    let mut sensor_ranges: Vec<(&Sensor, Range<i64>)> = sensors.iter().map(|s|(s, s.no_beacon_range(line))).collect();
    sensor_ranges.retain(|r| r.1.start != r.1.end);
    sensor_ranges.sort_by(|a, b| {
        a.1.start
            .cmp(&b.1.start)
            .then_with(|| (a.1.end - a.1.start).cmp(&(b.1.end - b.1.start)))
    });

    let mut range_seen: i64 = min_x;
    for n in 0..sensor_ranges.len() {
        let curr = &sensor_ranges[n];

        let start = curr.1.start;
        let end = curr.1.end;
        
        let coverage = if end < range_seen || start >= after_max_x {
            0
        } else if start >= range_seen && start < after_max_x {
            cmp::min(end, after_max_x) - start
        } else if start < range_seen && range_seen < after_max_x {
            cmp::min(end, after_max_x) - range_seen
        } else {
            0
        };        

        println!(" ◦ Sensor ({},{}) with {}..{} adds: {} (range_seen: {})", curr.0.x, curr.0.y, curr.1.start, curr.1.end, coverage, range_seen);

        range_seen = cmp::max(range_seen, curr.1.end);
        unusable_count += coverage;
    }

    unusable_count
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

    sensors
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
            dist: manhattan_dist(x, y, closest_x, closest_y),
        }
    }

    pub fn no_beacon_range(&self, line_num: i64) -> Range<i64> {
        let line_dist = (line_num - self.y).abs();

        if line_dist > self.dist {
            return 0..0;
        }

        let remaining_x = self.dist - line_dist;
        let left = self.x - remaining_x;
        let right = self.x + remaining_x + 1;

        left..right
    }
}