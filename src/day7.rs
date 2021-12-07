mod util;

fn main()
{
    let input = util::read_input_file("day7.txt");
    let positions = util::split_generics_by_string::<i32>(&input, ",");

    let (_, best_fuel_consumption) = get_position_with_lowest_fuel_consumption(&positions, true);
    println!("Part 1: Solution={}", best_fuel_consumption);

    let (_, best_fuel_consumption) = get_position_with_lowest_fuel_consumption(&positions, false);
    println!("Part 2: Solution={}", best_fuel_consumption);
}

fn get_position_with_lowest_fuel_consumption(positions: &Vec<i32>, constant_fuel_consumption: bool) -> (i32, i32)
{
    let mut best_position:i32 = -1;
    let mut best_fuel_consumption:i32 = i32::MAX;

    let max_position = *positions.iter().max().unwrap();
    for position in 0..=max_position
    {
        let fuel_consumption = get_fuel_consumption_for_position(&positions, position, constant_fuel_consumption);
        if fuel_consumption < best_fuel_consumption
        {
            best_position = position;
            best_fuel_consumption = fuel_consumption;
        }
    }

    return (best_position, best_fuel_consumption);
}

fn get_fuel_consumption_for_position(positions: &Vec<i32>, alignment_position: i32, constant_fuel_consumption: bool) -> i32
{
    let mut fuel_consumption: i32 = 0;

    for position in positions
    {
        let distance = if position >= &alignment_position { position - alignment_position } else { alignment_position - position };
        if constant_fuel_consumption
        {
            fuel_consumption += distance;
        }
        else
        {
            fuel_consumption += distance * (distance + 1) / 2; // Gauss summation formula
        }
    }

    return fuel_consumption;
}

#[test]
fn test_day7_part1_example()
{
    let input = "16,1,2,0,4,2,7,1,2,14".to_string();
    let positions = util::split_generics_by_string::<i32>(&input, ",");
    let (best_position, best_fuel_consumption) = get_position_with_lowest_fuel_consumption(&positions, true);

    assert_eq!(best_position, 2);
    assert_eq!(best_fuel_consumption, 37);
}

#[test]
fn test_day7_part1_solution()
{
    let input = util::read_input_file("day7.txt");
    let positions = util::split_generics_by_string::<i32>(&input, ",");
    let (best_position, best_fuel_consumption) = get_position_with_lowest_fuel_consumption(&positions, true);

    assert_eq!(best_position, 352);
    assert_eq!(best_fuel_consumption, 336131);
}

#[test]
fn test_day7_part2_example()
{
    let input = "16,1,2,0,4,2,7,1,2,14".to_string();
    let positions = util::split_generics_by_string::<i32>(&input, ",");
    let (best_position, best_fuel_consumption) = get_position_with_lowest_fuel_consumption(&positions, false);

    assert_eq!(best_position, 5);
    assert_eq!(best_fuel_consumption, 168);
}

#[test]
fn test_day7_part2_solution()
{
    let input = util::read_input_file("day7.txt");
    let positions = util::split_generics_by_string::<i32>(&input, ",");
    let (best_position, best_fuel_consumption) = get_position_with_lowest_fuel_consumption(&positions, false);

    assert_eq!(best_position, 473);
    assert_eq!(best_fuel_consumption, 92676646);
}