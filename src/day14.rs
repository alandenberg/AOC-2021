use std::collections::HashMap;
use substring::Substring;

mod util;

fn main() {
    let input = util::read_input_file("day14.txt");
    let mut lines = util::split_string_by_string(&input, "\n");
    let polymers = lines.remove(0);
    let rules = parse_insertion_rules(&lines);

    let result = perform_polymerization(&polymers, &rules, 10);
    let (min_polymer, max_polymer) = find_least_and_most_common_polymer(&result);
    println!("Part 1: Solution={}", max_polymer - min_polymer);
}

fn parse_insertion_rules(lines: &Vec<String>) -> Vec<(&str, &str)> {
    let mut rules: Vec<(&str, &str)> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        assert_eq!(parts.len(), 2);
        rules.push((parts[0], parts[1]));
    }

    return rules;
}

fn perform_polymerization(polymers: &String, rules: &Vec<(&str, &str)>, steps: i32) -> String {
    let mut result = String::from(polymers);

    for _ in 0..steps {
        let mut step_result = String::new();
        
        for i in 0..result.len()-1
        {
            let pair = result.substring(i, i + 2);
            step_result += pair.substring(0, 1);

            for rule in rules {
                if pair == rule.0 {
                    step_result += rule.1;
                    break;
                }
            }
        }

        step_result += result.substring(result.len()-1, result.len());
        result = step_result;
    }

    return result;
}

fn find_least_and_most_common_polymer(polymers: &String) -> (u64, u64) {
    let mut count: HashMap<char, u64> = HashMap::new();

    for polymer in polymers.chars() {
        if !count.contains_key(&polymer) {
            count.insert(polymer, 1);
        } else {
            let value = count.get(&polymer).unwrap();
            count.insert(polymer, value + 1);
        }
    }

    let min = count.values().min().unwrap();
    let max = count.values().max().unwrap();

    return (*min, *max);
}

#[test]
fn test_day14_part1_example() {
    let input = String::from("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C");
    let mut lines = util::split_string_by_string(&input, "\n");
    let polymers = lines.remove(0);
    let rules = parse_insertion_rules(&lines);
    let result = perform_polymerization(&polymers, &rules, 4);

    assert_eq!(result, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
}

#[test]
fn test_day14_part1_solution() {
    let input = util::read_input_file("day14.txt");
    let mut lines = util::split_string_by_string(&input, "\n");
    let polymers = lines.remove(0);
    let rules = parse_insertion_rules(&lines);
    let result = perform_polymerization(&polymers, &rules, 10);
    let (min_polymer, max_polymer) = find_least_and_most_common_polymer(&result);

    assert_eq!(min_polymer, 649);
    assert_eq!(max_polymer, 4480);
}