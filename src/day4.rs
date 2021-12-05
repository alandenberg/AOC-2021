mod util;

trait BingoLogicPublic
{
    fn add_number(&mut self, number: i32);
    fn sum_unmarked_numbers(&self) -> i32;
}

trait BingoLogicPrivate
{
    fn check_win_condition(&mut self, row: usize, column: usize) -> bool;
}

struct BingoBoard
{
    size: usize,
    numbers: Vec<Vec<i32>>,
    is_number_drawn: Vec<Vec<bool>>,
    has_won: bool,
}

impl BingoLogicPublic for BingoBoard
{
    fn add_number(&mut self, number: i32)
    {
        for i in 0..self.size
        {
            for j in 0..self.size
            {
                if self.numbers[i][j] == number
                {
                    self.is_number_drawn[i][j] = true;
                    self.has_won = self.check_win_condition(i, j);
                    break;
                }
            }
        }
    }

    fn sum_unmarked_numbers(&self) -> i32
    {
        let mut sum: i32 = 0;

        for i in 0..self.size
        {
            for j in 0..self.size
            {
                if self.is_number_drawn[i][j] == false
                {
                    sum += self.numbers[i][j];
                }
            }
        }

        return sum;
    }
}

impl BingoLogicPrivate for BingoBoard
{
    fn check_win_condition(&mut self, row: usize, column: usize) -> bool
    {
        let mut has_column_won = true;
        for i in 0..self.size
        {
            if self.is_number_drawn[i][column] == false
            {
                has_column_won = false;
                break;
            }
        }

        if has_column_won
        {
            return true
        }

        let mut has_row_won = true;
        for j in 0..self.size
        {
            if self.is_number_drawn[row][j] == false
            {
                has_row_won = false;
                break;
            }
        }

        return has_row_won;
    }
}

fn main()
{
    let input = util::read_input_file("day4.txt");
    let parts = util::split_string_by_string(&input, "\n\n");

    let score = get_first_winning_score(&parts);
    println!("Part 1: Solution={}", score);
}

fn get_first_winning_score(parts: &Vec<String>) -> i32
{
    let (drawn_numbers, mut bingo_boards) = parse_bingo_boards(&parts);

    for i in 1..parts.len()
    {
        let bingo_board = create_bingo_board(&parts[i]);
        bingo_boards.push(bingo_board);
    }

    for drawn_number in drawn_numbers
    {
        for bingo_board in bingo_boards.iter_mut()
        {
            bingo_board.add_number(drawn_number);
            if bingo_board.has_won == true
            {
                let sum = bingo_board.sum_unmarked_numbers();
                return sum * drawn_number;
            }
        }
    }

    return 0;
}

fn get_last_winning_score(parts: &Vec<String>) -> i32
{
    let (drawn_numbers, mut bingo_boards) = parse_bingo_boards(&parts);

    for i in 1..parts.len()
    {
        let bingo_board = create_bingo_board(&parts[i]);
        bingo_boards.push(bingo_board);
    }

    for drawn_number in drawn_numbers
    {
        let mut number_of_boards = bingo_boards.len();
        let mut indexes_to_remove: Vec<usize> = vec![];

        for i in 0..number_of_boards
        {
            let bingo_board = &mut bingo_boards[i];
            bingo_board.add_number(drawn_number);
            if bingo_board.has_won == true
            {
                if number_of_boards == 1
                {
                    let sum = bingo_board.sum_unmarked_numbers();
                    return sum * drawn_number;
                }
                indexes_to_remove.push(i);
                number_of_boards -= 1;
            }
        }

        for index_to_remove in indexes_to_remove.iter().rev()
        {
            bingo_boards.remove(*index_to_remove);
        }
    }

    return 0;
}

fn parse_bingo_boards(parts: &Vec<String>) -> (Vec<i32>, Vec<BingoBoard>)
{
    let drawn_number_strings = util::split_string_by_string(&parts[0], ",");
    let drawn_numbers = util::parse_strings_as_ints(&drawn_number_strings);

    let mut bingo_boards: Vec<BingoBoard> = vec![];

    for i in 1..parts.len()
    {
        let bingo_board = create_bingo_board(&parts[i]);
        bingo_boards.push(bingo_board);
    }

    return (drawn_numbers, bingo_boards);
}

fn create_bingo_board(board_string: &String) -> BingoBoard
{
    let lines = util::split_string_by_string(board_string, "\n");

    let size: usize = lines.len();
    let mut numbers: Vec<Vec<i32>> = vec![vec![0; size]; size];
    let is_number_drawn: Vec<Vec<bool>> = vec![vec![false; size]; size];

    for i in 0..size
    {
        let number_strings = util::split_string_by_string(&lines[i], " ");
        assert_eq!(number_strings.len(), size);

        for j in 0..size
        {
            numbers[i][j] = number_strings[j].parse::<i32>().unwrap();
        }
    }

    return BingoBoard { size: size, numbers: numbers, is_number_drawn: is_number_drawn, has_won: false };
}

#[test]
fn test_day4_part1_example()
{
    let input = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7".to_string();
    let parts = util::split_string_by_string(&input, "\n\n");

    let score = get_first_winning_score(&parts);

    assert_eq!(score, 4512)
}

#[test]
fn test_day4_part1_solution()
{
    let input = util::read_input_file("day4.txt");
    let parts = util::split_string_by_string(&input, "\n\n");

    let score = get_first_winning_score(&parts);

    assert_eq!(score, 49860);
}

#[test]
fn test_day4_part2_example()
{
    let input = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7".to_string();
    let parts = util::split_string_by_string(&input, "\n\n");

    let score = get_last_winning_score(&parts);

    assert_eq!(score, 1924);
}

#[test]
fn test_day4_part2_solution()
{
    let input = util::read_input_file("day4.txt");
    let parts = util::split_string_by_string(&input, "\n\n");

    let score = get_last_winning_score(&parts);

    assert_eq!(score, 24628);
}