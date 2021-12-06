mod util;

fn main()
{
    let input = util::read_input_file("day6.txt");
    let fishes = util::split_generics_by_string::<i32>(&input, ",");
    let fish_lives = parse_fish_lives(&fishes);
    let fishes_after_80_days = simulate_days(&fish_lives, 80);
    let fishes_after_256_days = simulate_days(&fish_lives, 256);
    
    println!("Part 1: Solution={}", fishes_after_80_days);
    println!("Part 2: Solution={}", fishes_after_256_days);
}

fn parse_fish_lives(fishes: &Vec<i32>) -> Vec<u64>
{
    let mut fish_lives: Vec<u64> = vec![0; 9];

    for fish in fishes
    {
        fish_lives[*fish as usize] += 1;
    }

    return fish_lives;
}

fn simulate_days(fish_lives: &Vec<u64>, simulate_days: i32) -> u64
{
    // Use a 9-element array with one position for each possible value of 'internal timer'.
    // For each day, move all fishes from first position to last position (child fishes) and add the same number to index 6 (reset parents)

    let mut rotated_fish_lives = fish_lives.to_vec();

    for _ in 0..simulate_days
    {
        rotated_fish_lives.rotate_left(1);
        rotated_fish_lives[6] += rotated_fish_lives[8];
    }

    let sum_of_fishes: u64 = rotated_fish_lives.iter().sum();
    return sum_of_fishes;
}

#[test]
fn test_day6_part1_example()
{
    let input = "3,4,3,1,2".to_string();
    let fishes = util::split_generics_by_string::<i32>(&input, ",");
    let fish_lives = parse_fish_lives(&fishes);
    let fishes_after_18_days = simulate_days(&fish_lives, 18);
    let fishes_after_80_days = simulate_days(&fish_lives, 80);

    assert_eq!(fishes_after_18_days, 26);
    assert_eq!(fishes_after_80_days, 5934);
}

#[test]
fn test_day6_part1_solution()
{
    let input = util::read_input_file("day6.txt");
    let fishes = util::split_generics_by_string::<i32>(&input, ",");
    let fish_lives = parse_fish_lives(&fishes);
    let fishes_after_80_days = simulate_days(&fish_lives, 80);

    assert_eq!(fishes_after_80_days, 352195);
}

#[test]
fn test_day6_part2_example()
{
    let input = "3,4,3,1,2".to_string();
    let fishes = util::split_generics_by_string::<i32>(&input, ",");
    let fish_lives = parse_fish_lives(&fishes);
    let fishes_after_256_days = simulate_days(&fish_lives, 256);

    assert_eq!(fishes_after_256_days, 26984457539);
}

#[test]
fn test_day6_part2_solution()
{
    let input = util::read_input_file("day6.txt");
    let fishes = util::split_generics_by_string::<i32>(&input, ",");
    let fish_lives = parse_fish_lives(&fishes);
    let fishes_after_256_days = simulate_days(&fish_lives, 256);

    assert_eq!(fishes_after_256_days, 1600306001288);
}