use std::fs::File;
use std::fmt;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    closest_x: i64,
    closest_y: i64
}

impl Sensor {
    pub fn new(x: i64, y: i64, closest_x: i64, closest_y:i64) -> Sensor { 
        Sensor {
            x,
            y,
            closest_x,
            closest_y
        }
    }
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sensor ({},{}) => closest ({},{})", self.x, self.y, self.closest_x, self.closest_y)
    }
}

fn main() {
    read_sensors("./Input.txt");
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
    let mut sensors = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_in) = line {
                let tokens: Vec<&str> = line_in.split("=").skip(1).collect();
                let nums:Vec<i64> = tokens.iter()
                    .map(|token| {
                        let num_text:String = token.chars()
                            .take_while(|&c| c >= '0' && c <= '9' || c == '-')
                            .collect();
                        num_text.parse::<i64>().unwrap()
                    }).collect();
                let sensor: Sensor = Sensor::new(nums[0], nums[1], nums[2], nums[3]);
                println!("{}", sensor);
                sensors.push(sensor)
            }
        }
    }

    sensors
}
