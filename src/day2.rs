mod util;

struct Position
{
    forward: i32,
    depth: i32,
}

fn main()
{
    let input = util::read_input_file("day2.txt");
    let lines = util::split_string_by_string(input, "\n");
    
    let position = get_position(&lines, false);

    println!("Part 1: Solution={}", position.forward * position.depth);

    let position_with_aim = get_position(&lines, true);

    println!("Part 2: Solution={}", position_with_aim.forward * position_with_aim.depth);
}

fn get_position(lines: &Vec<String>, use_aim: bool) -> Position
{
    let mut position = Position {
        forward: 0, 
        depth: 0
    };

    let mut aim: i32 = 0;

    for line in lines
    {
        let words: Vec<String> = line.split(" ").map(String::from).collect();
        let movement: &String = &words[0];
        let value: i32 = words[1].trim().parse::<i32>().unwrap();
        if movement == "forward"
        {
            position.forward += value;
            if use_aim
            {
                position.depth += aim * value;
            }
        }
        else if movement == "down"
        {
            if use_aim
            {
                aim += value;
            }
            else
            {
                position.depth += value;
            }
        }
        else if movement == "up"
        {
            if use_aim
            {
                aim -= value;
            }
            else
            {
                position.depth -= value;
            }
        }
    }

    return position;
}

#[test]
fn test_day2_part1_example()
{
    let input: String = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
".to_string();
    let lines = util::split_string_by_string(input, "\n");
    
    let position = get_position(&lines, false);

    assert_eq!(position.forward, 15);
    assert_eq!(position.depth, 10);
}

#[test]
fn test_day2_part1_solution()
{
    let input = util::read_input_file("day2.txt");
    let lines = util::split_string_by_string(input, "\n");
    
    let position = get_position(&lines, false);

    assert_eq!(position.forward, 2085);
    assert_eq!(position.depth, 785);
}

#[test]
fn test_day2_part2_example()
{
    let input: String = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
".to_string();
    let lines = util::split_string_by_string(input, "\n");
    
    let position = get_position(&lines, true);

    assert_eq!(position.forward, 15);
    assert_eq!(position.depth, 60);
}

#[test]
fn test_day2_part2_solution()
{
    let input = util::read_input_file("day2.txt");
    let lines = util::split_string_by_string(input, "\n");
    
    let position = get_position(&lines, true);

    assert_eq!(position.forward, 2085);
    assert_eq!(position.depth, 898205);
}