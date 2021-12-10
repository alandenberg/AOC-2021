use itertools::Itertools;

mod util;

fn main()
{
    let input = util::read_input_file("day10.txt");
    let lines = util::split_string_by_string(&input, "\n");

    let error_points = get_syntax_error_point_sum(&lines);
    println!("Part 1: Solution={}", error_points);

    let incomplete_lines = get_incomplete_lines(&lines);
    let incomplete_scores = get_incomplete_scores(&incomplete_lines);
    let middle_score = get_middle_score(&incomplete_scores);
    println!("Part 2: Solution={}", middle_score);
}

fn get_syntax_error_point_sum(lines: &Vec<String>) -> i32
{
    let mut error_points = 0;

    for line in lines
    {
        error_points += get_syntax_error_points_from_line(&line);
    }

    return error_points;
}

fn get_syntax_error_points_from_line(line: &String) -> i32
{
    let mut open_chunks: Vec<char> = vec![];
    let mut has_syntax_error = false;

    let characters = line.chars();
    for character in characters
    {
        if is_opening_character(character)
        {
            open_chunks.push(character);
        }
        else // is_closing_character
        {
            let opening_character = get_opening_character(character);

            if *open_chunks.last().unwrap() != opening_character
            {
                has_syntax_error = true;
            }
            else
            {
                open_chunks.pop();
            }
        }

        if has_syntax_error
        {
            return get_syntax_error_points(character);
        }
    }

    return 0;
}

fn get_incomplete_lines(lines: &Vec<String>) -> Vec<String>
{
    let mut incomplete_lines: Vec<String> = vec![];

    for line in lines
    {
        let error_points = get_syntax_error_points_from_line(&line);
        if error_points == 0
        {
            incomplete_lines.push(line.to_string());
        }
    }

    return incomplete_lines;
}

fn get_incomplete_scores(incomplete_lines: &Vec<String>) -> Vec<u64>
{
    let mut scores: Vec<u64> = vec![];

    for line in incomplete_lines
    {
        let mut score: u64 = 0;
        let missing_characters = get_completion_characters(&line);
        for character in missing_characters
        {
            score = score * 5 + get_completion_points(character);
        }
        scores.push(score);
    }

    return scores;
}

fn get_middle_score(scores: &Vec<u64>) -> u64
{
    let sorted_scores: Vec<u64> = scores.iter().sorted().map(|x| *x).collect();
    return sorted_scores[sorted_scores.len() / 2];
}

fn get_completion_characters(line: &String) -> Vec<char>
{
    let mut open_chunks: Vec<char> = vec![];

    let characters = line.chars();
    for character in characters
    {
        if is_opening_character(character)
        {
            open_chunks.push(character);
        }
        else // is_closing_character
        {
            open_chunks.pop();
        }
    }

    let mut completion_characters: Vec<char> = vec![];

    while !open_chunks.is_empty()
    {
        let opening_character = open_chunks.pop().unwrap();
        let closing_character = get_closing_character(opening_character);
        completion_characters.push(closing_character);
    }

    return completion_characters;
}

fn is_opening_character(character: char) -> bool
{
    return match character
    {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn get_opening_character(character: char) -> char
{
    return match character
    {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Illegal closing character"),
    }
}

fn get_closing_character(character: char) -> char
{
    return match character
    {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Illegal opening character"),
    }
}

fn get_syntax_error_points(character: char) -> i32
{
    return match character
    {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Illegal character"),
    }
}

fn get_completion_points(character: char) -> u64
{
    return match character
    {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Illegal character"),
    }
}

#[test]
fn test_day10_part1_example()
{
    let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let error_points = get_syntax_error_point_sum(&lines);

    assert_eq!(error_points, 26397);
}

#[test]
fn test_day10_part1_solution()
{
    let input = util::read_input_file("day10.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let error_points = get_syntax_error_point_sum(&lines);

    assert_eq!(error_points, 464991);
}

#[test]
fn test_day10_part2_example()
{
    let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let incomplete_lines = get_incomplete_lines(&lines);
    let incomplete_scores = get_incomplete_scores(&incomplete_lines);
    let middle_score = get_middle_score(&incomplete_scores);

    assert_eq!(middle_score, 288957);
}

#[test]
fn test_day10_part2_solution()
{
    let input = util::read_input_file("day10.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let incomplete_lines = get_incomplete_lines(&lines);
    let incomplete_scores = get_incomplete_scores(&incomplete_lines);
    let middle_score = get_middle_score(&incomplete_scores);

    assert_eq!(middle_score, 3662008566);
}