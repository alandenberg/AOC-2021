use std::cmp;
use substring::Substring;

mod util;

fn main() {
    let input = util::read_input_file("day17.txt");
    let target_area = parse_target_area(&input);
    let (y_max, hit_count) = find_initial_velocity(&target_area);
    println!("Part 1: Solution={}", y_max);
    println!("Part 2: Solution={}", hit_count);
}


#[derive(Debug, Clone, Copy)]
struct Area {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i32,
    y: i32,
}

fn find_initial_velocity(target_area: &Area) -> (i32, i32) {
    let mut hit_count = 0;
    let mut global_y_max = 0;

    for y in -1000..1000 {
        for x in 0..1000 { // negative x will never reach the target area
            let mut velocity = Velocity { x, y };
            let (hit, y_max) = hits_target_area(&mut velocity, target_area);
            if hit {
                hit_count += 1;
            }
            global_y_max = cmp::max(global_y_max, y_max);
        }
    }

    return (global_y_max, hit_count);
}

fn hits_target_area(velocity: &mut Velocity, target_area: &Area) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut y_max = 0;

    loop {
        if x >= target_area.x_min && x <= target_area.x_max && y >= target_area.y_min && y <= target_area.y_max {
            return (true, y_max);
        }

        x += velocity.x;
        y += velocity.y;
        if velocity.x != 0 {
            velocity.x = if velocity.x > 0 { velocity.x - 1} else { velocity.x + 1 };
        }
        velocity.y -= 1;
        y_max = cmp::max(y_max, y);

        if x > target_area.x_max || y < target_area.y_min {
            return (false, 0);
        }
    }
}

fn parse_target_area(line: &String) -> Area {
    let line = line.substring(13, line.len());
    let xy: Vec<&str> = line.split(", ").collect();
    assert_eq!(xy.len(), 2);
    let x_range: Vec<i32> = xy[0].trim().substring(2, xy[0].len()).split("..").map(|x| x.parse::<i32>().unwrap()).collect();
    let y_range: Vec<i32> = xy[1].trim().substring(2, xy[1].len()).split("..").map(|x| x.parse::<i32>().unwrap()).collect();
    return Area { x_min: x_range[0], x_max: x_range[1], y_min: y_range[0], y_max: y_range[1] };
}

#[test]
fn test_day17_part1_example() {
    let input = String::from("target area: x=20..30, y=-10..-5");
    let target_area = parse_target_area(&input);
    let (y_max, _) = find_initial_velocity(&target_area);
    assert_eq!(y_max, 45);
}

#[test]
fn test_day17_part1_solution() {
    let input = util::read_input_file("day17.txt");
    let target_area = parse_target_area(&input);
    let (y_max, _) = find_initial_velocity(&target_area);
    assert_eq!(y_max, 5886);
}

#[test]
fn test_day17_part2_example() {
    let input = String::from("target area: x=20..30, y=-10..-5");
    let target_area = parse_target_area(&input);
    let (_, hit_count) = find_initial_velocity(&target_area);
    assert_eq!(hit_count, 112);
}

#[test]
fn test_day17_part2_solution() {
    let input = util::read_input_file("day17.txt");
    let target_area = parse_target_area(&input);
    let (_, hit_count) = find_initial_velocity(&target_area);
    assert_eq!(hit_count, 1806);
}