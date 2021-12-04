mod util;

fn main()
{
    let input = util::read_input_file("day3.txt");
    let lines = util::split_string_by_string(input, "\n");
    
    let columns = extract_all_bits_column_wise(&lines);
    let gamma_rate = get_gamma_rate(&columns);
    let epsilon_rate = get_epsilon_rate(&columns);
    println!("Part 1: Solution={}", gamma_rate * epsilon_rate);

    let oxygen_generator_rating = get_oxygen_generator_rating(&lines);
    let co2_scrubber_rating = get_co2_scrubber_rating(&lines);
    println!("Part 2: Solution={}", oxygen_generator_rating * &co2_scrubber_rating);
}

fn get_gamma_rate(columns: &Vec<String>) -> i32
{
    let mut binary_string = String::from("");

    for column in columns
    {
        let bit_char = get_most_common_bit(column, '1');
        binary_string.push(bit_char);
    }

    return parse_binary_string_as_decimal_int(binary_string);
}

fn get_epsilon_rate(columns: &Vec<String>) -> i32
{
    let mut binary_string = String::from("");

    for column in columns
    {
        let bit_char = get_most_common_bit(column, '1');
        binary_string.push(invert_bit(bit_char));
    }

    return parse_binary_string_as_decimal_int(binary_string);
}

fn get_oxygen_generator_rating(lines: &Vec<String>) -> i32
{
    let line = get_line_by_bit_criteria(&lines, false);
    return parse_binary_string_as_decimal_int(line);
}

fn get_co2_scrubber_rating(lines: &Vec<String>) -> i32
{
    let line = get_line_by_bit_criteria(&lines, true);
    return parse_binary_string_as_decimal_int(line);
}

fn get_line_by_bit_criteria(lines: &Vec<String>, invert_criteria: bool) -> String
{
    let line_length = lines[0].len();
    let mut matrix = vec![vec!['0' as char; line_length]; lines.len()];

    for i in 0..lines.len()
    {
        let characters: Vec<char> = lines[i].chars().collect();
        for j in 0..characters.len()
        {
            matrix[i][j] = characters[j];
        }
    }

    let mut column_idx = 0;
    let mut rows_to_iterate: Vec<usize> = (0..lines.len()).collect();
    assert_eq!(rows_to_iterate.last().unwrap(), &(lines.len()-1));

    while rows_to_iterate.len() > 1
    {
        let mut column_string: String = String::from("");

        for i in &rows_to_iterate
        {
            column_string.push(matrix[*i][column_idx]);
        }

        let mut most_common_bit_in_column: char = get_most_common_bit(&column_string, '1');
        most_common_bit_in_column = if invert_criteria { invert_bit(most_common_bit_in_column) } else { most_common_bit_in_column };

        let mut next_rows_to_iterate: Vec<usize> = vec![];
    
        for i in &rows_to_iterate
        {
            if matrix[*i][column_idx] == most_common_bit_in_column
            {
                next_rows_to_iterate.push(*i)
            }
        }

        rows_to_iterate = next_rows_to_iterate;
        column_idx += 1;
    }

    assert_eq!(rows_to_iterate.len(), 1);

    let line: String = lines[rows_to_iterate[0]].to_string();

    return line;
}

fn invert_bit(bit: char) -> char
{
    return if bit == '0' { '1' } else { '0' };
}

fn extract_all_bits_column_wise(lines: &Vec<String>) -> Vec<String>
{
    let num_of_columns = lines[0].len();
    let mut columns: Vec<String> = vec!["".to_string(); num_of_columns];

    for i in 0..lines.len()
    {
        let line = &lines[i];
        for j in 0..line.len()
        {
            let bit = line.chars().nth(j).unwrap();
            columns[j].push(bit);
        }
    }

    return columns;
}

fn get_most_common_bit(column: &String, equal_char: char) -> char
{
    let mut zeros: i32 = 0;
    let mut ones: i32 = 0;

    for i in 0..column.len()
    {
        let bit = column.chars().nth(i).unwrap();
        if bit == '0'
        {
            zeros += 1;
        }
        else
        {
            ones += 1;
        }
    }

    if zeros == ones
    {
        return equal_char;
    }

    return if zeros > ones { '0' } else { '1' };
}

fn parse_binary_string_as_decimal_int(binary_string: String) -> i32
{
    return isize::from_str_radix(&binary_string, 2).unwrap() as i32;
}

#[test]
fn test_day3_part1_example()
{
    let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".to_string();
    let lines = util::split_string_by_string(input, "\n");

    let columns = extract_all_bits_column_wise(&lines);
    let gamma_rate = get_gamma_rate(&columns);
    let epsilon_rate = get_epsilon_rate(&columns);

    assert_eq!(gamma_rate, 22);
    assert_eq!(epsilon_rate, 9);
}

#[test]
fn test_day3_part1_solution()
{
    let input = util::read_input_file("day3.txt");
    let lines = util::split_string_by_string(input, "\n");
    
    let columns = extract_all_bits_column_wise(&lines);
    let gamma_rate = get_gamma_rate(&columns);
    let epsilon_rate = get_epsilon_rate(&columns);

    assert_eq!(gamma_rate * epsilon_rate, 775304);
}

#[test]
fn test_day3_part2_example()
{
    let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".to_string();
    let lines = util::split_string_by_string(input, "\n");

    let oxygen_generator_rating = get_oxygen_generator_rating(&lines);
    let co2_scrubber_rating = get_co2_scrubber_rating(&lines);

    assert_eq!(oxygen_generator_rating, 23);
    assert_eq!(co2_scrubber_rating, 10);
}

#[test]
fn test_day3_part2_solution()
{
    let input = util::read_input_file("day3.txt");
    let lines = util::split_string_by_string(input, "\n");
    
    let oxygen_generator_rating = get_oxygen_generator_rating(&lines);
    let co2_scrubber_rating = get_co2_scrubber_rating(&lines);

    assert_eq!(oxygen_generator_rating * co2_scrubber_rating, 1370737);
}