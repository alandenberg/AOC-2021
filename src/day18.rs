mod util;

fn main() {
    let input = util::read_input_file("day18.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let snailfish_numbers = parse_snailfish_numbers(&lines);

    let number = add(&snailfish_numbers);
    let mag = calculate_magnitude(&number);
    println!("Part 1: Solution={}", mag);

    let largest_mag = get_largest_magnitude(&snailfish_numbers);
    println!("Part 2: Solution={}", largest_mag);
}

fn parse_snailfish_numbers(lines: &Vec<String>) -> Vec<Vec<(i32, i32)>> {
    return lines.iter().map(|line| parse_snailfish_number(&line)).collect();
}

fn parse_snailfish_number(line: &String) -> Vec<(i32, i32)> {
    let mut number: Vec<(i32, i32)> = vec![];
    let mut depth = 0;

    for c in line.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {},
            _ => number.push((depth, c.to_digit(10).unwrap() as i32)),
        }
    }

    return number;
}

fn add(numbers: &Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut mut_numbers = numbers.clone();

    for i in 0..numbers.len()-1 {
        let next = &mut mut_numbers.remove(1);
        let first = &mut mut_numbers[0];
        first.append(next);
        first.iter_mut().for_each(|(depth, _)| *depth += 1);
        reduce(first, 0);
    }

    return mut_numbers.remove(0);
}

fn reduce(number: &mut Vec<(i32, i32)>, start: usize) {
    // explode
    for i in start..number.len()-1 {
        if number[i].0 == 5 { // nested inside 4 pairs
            let left_value = number[i].1;
            let right_value = number[i + 1].1;
            number[i] = (4, 0);
            number.remove(i + 1);
            if i > 0 {
                number[i - 1].1 += left_value;
            }
            if i < number.len()-1 {
                number[i + 1].1 += right_value;
            }
            return reduce(number, i);
        }
    }

    // split
    for i in 0..number.len() {
        let (depth, value) = number[i];
        if value >= 10 {
            number[i] = (depth + 1, value / 2);
            number.insert(i + 1, (depth + 1, (value + 1) / 2));
            return reduce(number, i);
        }
    }
}

fn calculate_magnitude(number: &Vec<(i32, i32)>) -> i32 {
    return magnitude(number, &mut 0, 1);
}

fn magnitude(number: &Vec<(i32, i32)>, i: &mut usize, depth: i32) -> i32 {
    let left: i32 = if number[*i].0 == depth {
        *i += 1;
        number[*i - 1].1 as i32
    } else {
        magnitude(number, i, depth + 1)
    };

    let right = if number[*i].0 == depth {
        *i += 1;
        number[*i - 1].1 as i32
    } else {
        magnitude(number, i, depth + 1)
    };

    return 3 * left + 2 * right;
}

fn get_largest_magnitude(numbers: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut largest_magnitude = 0;

    for i in 0..numbers.len()-2 {
        for j in i+1..numbers.len()-1 {
            let mut two_numbers: Vec<Vec<(i32, i32)>> = vec![];

            // standard order
            two_numbers.push(numbers[i].clone());
            two_numbers.push(numbers[j].clone());
            let added_number = add(&two_numbers);
            let mag = calculate_magnitude(&added_number);
            if mag > largest_magnitude {
                largest_magnitude = mag;
            }

            // reverse order
            two_numbers.clear();
            two_numbers.push(numbers[j].clone());
            two_numbers.push(numbers[i].clone());
            let added_number = add(&two_numbers);
            let mag = calculate_magnitude(&added_number);
            if mag > largest_magnitude {
                largest_magnitude = mag;
            }
        }
    }

    return largest_magnitude;
}

#[test]
fn test_day18_part1_example1() {
    let input = String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
    let lines = util::split_string_by_string(&input, "\n");
    let snailfish_numbers = parse_snailfish_numbers(&lines);
    let number = add(&snailfish_numbers);
    let mag = calculate_magnitude(&number);
    assert_eq!(mag, 4140);
}

#[test]
fn test_day18_part1_solution() {
    let input = util::read_input_file("day18.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let snailfish_numbers = parse_snailfish_numbers(&lines);
    let number = add(&snailfish_numbers);
    let mag = calculate_magnitude(&number);
    assert_eq!(mag, 3816);
}

#[test]
fn test_day18_part2_example1() {
    let input = String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
    let lines = util::split_string_by_string(&input, "\n");
    let snailfish_numbers = parse_snailfish_numbers(&lines);
    let largest_mag = get_largest_magnitude(&snailfish_numbers);
    assert_eq!(largest_mag, 3993);
}

#[test]
fn test_day18_part2_solution() {
    let input = util::read_input_file("day18.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let snailfish_numbers = parse_snailfish_numbers(&lines);
    let largest_mag = get_largest_magnitude(&snailfish_numbers);
    assert_eq!(largest_mag, 4819);
}