use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

mod util;

fn main()
{
    let input = util::read_input_file("day8.txt");
    let lines = util::split_string_by_string(&input, "\n");
    
    let num_unique_digits = count_unique_digits(&lines);
    println!("Part 1: Solution={}", num_unique_digits);

    let num_all_digits = count_all_digits(&lines);
    println!("Part 2: Solution={}", num_all_digits);
}

fn count_unique_digits(lines: &Vec<String>) -> i32
{
    let mut sum_unique_digits = 0;

    for line in lines
    {
        let parts = util::split_string_by_string(line, "|");
        assert_eq!(parts.len(), 2);
        sum_unique_digits += count_unique_digits_in_line(&parts[1]);
    }

    return sum_unique_digits;
}

fn count_unique_digits_in_line(line: &String) -> i32
{
    let mut sum_of_unique_digits = 0;
    let parts = util::split_string_by_string(line, " ");

    for part in parts
    {
        match part.len()
        {
            2 | 3 | 4 | 7 => sum_of_unique_digits += 1,
            _ => {},
        }
    }

    return sum_of_unique_digits;
}

fn count_all_digits(lines: &Vec<String>) -> u64
{
    let mut sum_of_digits: u64 = 0;

    for line in lines
    {
        let parts = util::split_string_by_string(line, "|");
        assert_eq!(parts.len(), 2);
        let wire_mapping = get_wire_mapping_for_line(&parts[0]);
        let sub_parts = util::split_string_by_string(&parts[1], " ");

        assert_eq!(sub_parts.len(), 4);

        for i in 0..sub_parts.len()
        {
            let digit: i32 = get_digit_from_wire_mapping(&sub_parts[i], &wire_mapping);
            sum_of_digits += 10u64.pow(3 - (i as u32)) * digit as u64;
        }
    }

    return sum_of_digits;
}

fn get_wire_mapping_for_line(line: &String) -> HashMap<String, i32>
{
    let mut wire_mapping: HashMap<String, i32> = HashMap::new();
    let mut segments_one: HashSet<char> = HashSet::new();
    let mut segments_four: HashSet<char> = HashSet::new();

    let parts = util::split_string_by_string(line, " ");
    let sorted_parts: Vec<String> = parts.iter().map(|x| x.chars().sorted().collect::<String>()).collect();

    // First, find out which segments are used to display digits 1 and 4
    for segments in &sorted_parts
    {
        let number = match segments.len()
        {
            2 => 1,
            4 => 4,
            _ => -1,
        };
        match number
        {
            1 => segments_one = segments.chars().collect(),
            4 => segments_four = segments.chars().collect(),
            _ => continue,
        };
    }

    assert_ne!(segments_one.len(), 0);
    assert_ne!(segments_four.len(), 0);

    // Then, use the found segments for 1 and 4 to find out which non-unique digit is which
    for segments in &sorted_parts
    {
        // 1, 4, 7, 8 are unique
        // 2, 3, 5 have 5 segments enabled
        // 0, 6, 9 have 6 segments enabled
        let number = match segments.len()
        {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => get_number_from_ambigous_segments(&segments, &segments_one, &segments_four),
            6 => get_number_from_ambigous_segments(&segments, &segments_one, &segments_four),
            7 => 8,
            _ => -1,
        };

        assert_ne!(number, -1);

        if !wire_mapping.contains_key(segments)
        {
            wire_mapping.insert(segments.to_string(), number);
        }
    }

    return wire_mapping;
}

fn get_number_from_ambigous_segments(sorted_segments: &String, segments_one: &HashSet<char>, segments_four: &HashSet<char>) -> i32
{
    let segments: HashSet<char> = sorted_segments.chars().collect();

    // 5 segment digits:
    // 3 -> overlap(3, 1) == 2
    // 5 -> overlap(5, 4) == 3
    // 2 -> overlap(2, 4) == 2
    if segments.len() == 5
    {
        if segments_one.intersection(&segments).count() == 2
        {
            return 3;
        }
        if segments_four.intersection(&segments).count() == 3
        {
            return 5;
        }
        if segments_four.intersection(&segments).count() == 2
        {
            return 2;
        }
    }

    // 6 segment digits:
    // 6 -> overlap(6, 1) == 1
    // 9 -> overlap(9, 4) == 4
    // 0 -> overlap(0, 4) == 3
    if segments.len() == 6
    {
        if segments_one.intersection(&segments).count() == 1
        {
            return 6;
        }
        if segments_four.intersection(&segments).count() == 4
        {
            return 9;
        }
        if segments_four.intersection(&segments).count() == 3
        {
            return 0;
        }
    }

    return -1;
}

fn get_digit_from_wire_mapping(line: &String, wire_mapping: &HashMap<String, i32>) -> i32
{
    let sorted_segments: String = line.chars().sorted().collect();
    return wire_mapping[&sorted_segments];
}

#[test]
fn test_day8_part1_example()
{
    let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let num_unique_digits = count_unique_digits(&lines);

    assert_eq!(num_unique_digits, 26);
}

#[test]
fn test_day8_part1_solution()
{
    let input = util::read_input_file("day8.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let num_unique_digits = count_unique_digits(&lines);

    assert_eq!(num_unique_digits, 554);
}

#[test]
fn test_day8_part2_example()
{
    let input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let num_unique_digits = count_all_digits(&lines);

    assert_eq!(num_unique_digits, 61229);
}

#[test]
fn test_day8_part2_solution()
{
    let input = util::read_input_file("day8.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let num_unique_digits = count_all_digits(&lines);

    assert_eq!(num_unique_digits, 990964);
}