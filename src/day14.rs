use std::collections::HashMap;
use substring::Substring;
use std::cmp;

mod util;

fn main() {
    let input = util::read_input_file("day14.txt");
    let mut lines = util::split_string_by_string(&input, "\n");
    let polymers = lines.remove(0);
    let rules = parse_insertion_rules(&lines);

    let (min, max) = perform_polymerization(&polymers, &rules, 10);
    println!("Part 1: Solution={}", max - min);

    let (min, max) = perform_polymerization(&polymers, &rules, 40);
    println!("Part 2: Solution={}", max - min);
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

fn perform_polymerization(polymers: &String, rules: &Vec<(&str, &str)>, steps: i32) -> (u64, u64) {
    let mut pairs: HashMap<String, u64> = HashMap::new();

    for i in 0..polymers.len() {
        let end = cmp::min(i+2, polymers.len());
        let pair = String::from(polymers.substring(i, end));
        if !pairs.contains_key(&pair) {
            pairs.insert(pair, 1);
        } else {
            pairs.insert(pair.to_string(), pairs[&pair]+1);
        }
    }

    for _ in 0..steps {
        let step_pairs = pairs.clone();
        pairs.clear();

        for pair in step_pairs.keys() {
            match rules.iter().find(|x| &x.0 == pair) {
                Some(rule) => {
                    let mut pair1 = String::from(pair.chars().nth(0).unwrap());
                    pair1 += rule.1;
                    let mut pair2 = String::from(rule.1);
                    pair2.push(pair.chars().nth(1).unwrap());

                    if !pairs.contains_key(&pair1) {
                        pairs.insert(pair1, step_pairs[pair]);
                    } else {
                        pairs.insert(pair1.to_string(), pairs[&pair1]+step_pairs[pair]);
                    }

                    if !pairs.contains_key(&pair2) {
                        pairs.insert(pair2, step_pairs[pair]);
                    } else {
                        pairs.insert(pair2.to_string(), pairs[&pair2]+step_pairs[pair]);
                    }
                },
                None => {
                    if !pairs.contains_key(pair) {
                        pairs.insert(pair.to_string(), step_pairs[pair]);
                    } else {
                        pairs.insert(pair.to_string(), pairs[pair]+step_pairs[pair]);
                    }
                },
            }
        }
    }

    let mut polymer_count: HashMap<char, u64> = HashMap::new();
    for pair in pairs {
        let polymer = pair.0.chars().nth(0).unwrap();

        if !polymer_count.contains_key(&polymer) {
            polymer_count.insert(polymer, pair.1);
        } else {
            polymer_count.insert(polymer, polymer_count[&polymer]+pair.1);
        }
    }

    let min = polymer_count.values().min().unwrap();
    let max = polymer_count.values().max().unwrap();

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
    let (min, max) = perform_polymerization(&polymers, &rules, 4);

    assert_eq!(min, 5);
    assert_eq!(max, 23);
}

#[test]
fn test_day14_part1_solution() {
    let input = util::read_input_file("day14.txt");
    let mut lines = util::split_string_by_string(&input, "\n");
    let polymers = lines.remove(0);
    let rules = parse_insertion_rules(&lines);
    let (min, max) = perform_polymerization(&polymers, &rules, 10);
    
    assert_eq!(min, 649);
    assert_eq!(max, 4480);
}

#[test]
fn test_day14_part2_solution() {
    let input = util::read_input_file("day14.txt");
    let mut lines = util::split_string_by_string(&input, "\n");
    let polymers = lines.remove(0);
    let rules = parse_insertion_rules(&lines);
    let (min, max) = perform_polymerization(&polymers, &rules, 40);
    
    assert_eq!(min, 362729313279);
    assert_eq!(max, 6088469227561);
}