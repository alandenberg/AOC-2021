use substring::Substring;

mod util;

fn main() {
    let input = util::read_input_file("day13.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let (mut paper, instructions) = extract_paper_and_instructions(&lines);

    for i in 0..instructions.len() {
        paper = fold_paper(&paper, &instructions[i]);
        let sum: usize = paper.iter().map(|y| y.iter().filter(|x| **x == true).count()).sum();

        if i == 0 {
            println!("Part 1: Solution={}", sum);
        }
    }

    let final_output = print_paper(&paper);
    println!("Part 2:\n{}", final_output);
}

fn extract_paper_and_instructions(lines: &Vec<String>) -> (Vec<Vec<bool>>, Vec<String>) {
    let dots: Vec<String> = lines.iter().filter(|x| !x.starts_with("fold along")).map(|x| String::from(x)).collect();
    let instructions: Vec<String> = lines.iter().filter(|x| x.starts_with("fold along")).map(|x| String::from(x)).collect();
    let (width, height) = get_paper_size_from_instructions(&instructions);
    let paper = fill_paper(&dots, width, height);
    return (paper, instructions);
}

fn fill_paper(dots: &Vec<String>, width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut paper: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for dot in dots {
        let dot_parts = util::split_generics_by_string::<usize>(dot, ",");
        assert_eq!(dot_parts.len(), 2);

        let x = dot_parts[0];
        let y = dot_parts[1];
        paper[y][x] = true;
    }

    return paper;
}

fn fold_paper(paper: &Vec<Vec<bool>>, instruction_line: &str) -> Vec<Vec<bool>> {
    let x_size: usize;
    let y_size: usize;

    let (axis, number) = parse_instruction(instruction_line);

    let paper_x_size = paper[0].len();
    let paper_y_size = paper.len();

    if axis == "x" {
        x_size = (paper_x_size as f32 / 2f32).floor() as usize;
        y_size = paper_y_size;
        assert_eq!(x_size, number as usize);
    } else {
        x_size = paper_x_size;
        y_size = (paper_y_size as f32 / 2f32).floor() as usize;
        assert_eq!(y_size, number as usize);
    }

    let mut folded_paper: Vec<Vec<bool>> = vec![vec![false; x_size]; y_size];

    for y in 0..y_size {
        for x in 0..x_size {
            let folded_x = if axis == "x" { paper_x_size - x - 1 } else { x };
            let folded_y = if axis == "y" { paper_y_size - y - 1 } else { y };
            folded_paper[y][x] = paper[y][x] || paper[folded_y][folded_x];
        }
    }

    return folded_paper;
}

fn print_paper(paper: &Vec<Vec<bool>>) -> String {
    let mut output = String::new();

    for line in paper {
        let mut dots = String::new();
        for dot in line {
            dots += if *dot { "#" } else { "." };
        }
        output += &dots;
        output += "\n";
    }

    return output;
}

fn parse_instruction(instruction: &str) -> (&str, i32) {
    let instruction = instruction.substring(11, instruction.len());
    let parts: Vec<&str> = instruction.split("=").collect();
    let axis = parts[0];
    let number = parts[1].parse::<i32>().unwrap();
    return (axis, number);
}

fn get_paper_size_from_instructions(instructions: &Vec<String>) -> (usize, usize) {
    let (axis1, number1) = parse_instruction(&instructions[0]);
    let (axis2, number2) = parse_instruction(&instructions[1]);

    if axis1 == "x" {
        assert_eq!(axis2, "y");
        return ((number1 * 2 + 1) as usize, (number2 * 2 + 1) as usize);
    } else {
        assert_eq!(axis2, "x");
        return ((number2 * 2 + 1) as usize, (number1 * 2 + 1) as usize);
    }
}

#[test]
fn test_day13_part1_example() {
    let input = String::from("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5");
    let lines = util::split_string_by_string(&input, "\n");
    let (paper, instructions) = extract_paper_and_instructions(&lines);

    let folded_paper = fold_paper(&paper, &instructions[0]);
    let sum: usize = folded_paper.iter().map(|y| y.iter().filter(|x| **x == true).count()).sum();
    assert_eq!(sum, 17);

    let folded_paper2 = fold_paper(&folded_paper, &instructions[1]);
    let sum2: usize = folded_paper2.iter().map(|y| y.iter().filter(|x| **x == true).count()).sum();
    assert_eq!(sum2, 16);
}

#[test]
fn test_day13_part1_solution() {
    let input = util::read_input_file("day13.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let (paper, instructions) = extract_paper_and_instructions(&lines);

    let folded_paper = fold_paper(&paper, &instructions[0]);
    let sum: usize = folded_paper.iter().map(|y| y.iter().filter(|x| **x == true).count()).sum();
    assert_eq!(sum, 687);
}

#[test]
fn test_day13_part2_solution() {
    let input = util::read_input_file("day13.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let (mut paper, instructions) = extract_paper_and_instructions(&lines);

    for i in 0..instructions.len() {
        paper = fold_paper(&paper, &instructions[i]);
    }

    let sum: usize = paper.iter().map(|y| y.iter().filter(|x| **x == true).count()).sum();
    assert_eq!(sum, 98);
}