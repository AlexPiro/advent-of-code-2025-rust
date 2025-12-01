use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_CLICK: i32 = 100;

fn part1(input_file: &str) -> i32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(input_file) {
        let mut dial_position = 50;
        for line in lines.map_while(Result::ok) {
            // println!("{}", line);
            let (a, b) = line.split_at(1);

            let side = match a {
                "L" => -1,
                "R" => 1,
                _ => 0,
            };

            let distance = b.parse::<i32>().unwrap();

            dial_position += distance * side;

            dial_position = dial_position % MAX_CLICK;

            if dial_position == 0 {
                result += 1;
            }
        }
    }

    result
}

fn part2(input_file: &str) -> i32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(input_file) {
        let mut dial_position = 50;
        for line in lines.map_while(Result::ok) {
            // println!("{}", line);
            let (a, b) = line.split_at(1);

            let side = match a {
                "L" => -1,
                "R" => 1,
                _ => 0,
            };

            let distance = b.parse::<i32>().unwrap();

            result += distance / MAX_CLICK;

            dial_position += distance * side;

            dial_position = dial_position % MAX_CLICK;

            let left_distance = distance % MAX_CLICK;
            let distance_to_zero = if side > 0 { dial_position } else { (MAX_CLICK - dial_position) % MAX_CLICK};

            if (distance_to_zero > 0 && distance_to_zero < left_distance) || dial_position == 0 {
                result += 1;
            }  

        }
    }

    result
}

fn main() {
    assert_eq!(part1("Day01/example.txt"), 3);
    println!("Part 1 solution: {}", part1("Day01/puzzle.txt"));
    assert_eq!(part2("Day01/example.txt"), 6);
    println!("Part 2 solution: {}", part2("Day01/puzzle.txt"));

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
