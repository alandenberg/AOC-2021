mod util;

fn main()
{
    let input = util::read_input_file("day1.txt");
    let lines = util::get_lines_from_string(input, "\n");
    let measurements = util::parse_lines_as_ints(&lines);

    let larger_than_previous = count_measurements_larger_than_previous(&measurements);
    println!("Part 1: Measurements larger than previous: {}", larger_than_previous);

    let triples_larger_than_previous = count_measurements_triples_larger_than_previous(&measurements);
    println!("Part 2: Measurment triples larger than previous: {}", triples_larger_than_previous);
}

fn count_measurements_larger_than_previous(measurements: &Vec<i32>) -> i32
{
    let mut larger_than_previous: i32 = 0;

    for i in 1..measurements.len()
    {
        if measurements[i] > measurements[i-1]
        {
            larger_than_previous += 1;
        }
    }

    return larger_than_previous;
}

fn count_measurements_triples_larger_than_previous(measurements: &Vec<i32>) -> i32
{
    let mut larger_than_previous: i32 = 0;
    let mut previous_triple_sum: i32 = -1;

    for i in 2..measurements.len()
    {
        let triple_sum = measurements[i] + measurements[i-1] + measurements[i-2];
        if previous_triple_sum > 0 && triple_sum > previous_triple_sum
        {
            larger_than_previous += 1;
        }
        previous_triple_sum = triple_sum;
    }

    return larger_than_previous;
}

#[test]
fn test_day1_part1_example()
{
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let larger_than_previous = count_measurements_larger_than_previous(&measurements);

    assert_eq!(larger_than_previous, 7);
}

#[test]
fn test_day1_part1_solution()
{
    let input = util::read_input_file("day1.txt");
    let lines = util::get_lines_from_string(input, "\n");
    let measurements = util::parse_lines_as_ints(&lines);

    let larger_than_previous = count_measurements_larger_than_previous(&measurements);

    assert_eq!(larger_than_previous, 1532);
}

#[test]
fn test_day1_part2_example()
{
    let measurements = vec![607, 618, 618, 617, 647, 716, 769, 792];

    let larger_than_previous = count_measurements_triples_larger_than_previous(&measurements);

    assert_eq!(larger_than_previous, 5);
}

#[test]
fn test_day1_part2_solution()
{
    let input = util::read_input_file("day1.txt");
    let lines = util::get_lines_from_string(input, "\n");
    let measurements = util::parse_lines_as_ints(&lines);

    let larger_than_previous = count_measurements_triples_larger_than_previous(&measurements);

    assert_eq!(larger_than_previous, 1571);
}
