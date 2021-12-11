mod util;

trait OctopusLogic
{
    fn step(&mut self);
    fn propagate_flash(&mut self, x: usize, y: usize, increase: bool);
    fn all_flashing(&mut self) -> bool;
}

struct Grid
{
    grid: Vec<Vec<i32>>,
    flashed: Vec<Vec<bool>>,
    num_of_flashes: u64,
}

impl OctopusLogic for Grid
{
    fn step(&mut self)
    {
        let size = self.grid.len();
        self.flashed = vec![vec![false; size]; size];
        
        for y in 0..size
        {
            for x in 0..size
            {
                self.grid[y][x] += 1;
            }
        }

        for y in 0..size
        {
            for x in 0..size
            {
                self.propagate_flash(x, y, false);
            }
        }
    }

    fn propagate_flash(&mut self, x: usize, y: usize, increase: bool)
    {
        if self.flashed[y][x] == true
        {
            return;
        }

        if increase
        {
            self.grid[y][x] += 1;
        }

        if self.grid[y][x] > 9
        {
            self.flashed[y][x] = true;
            self.num_of_flashes += 1;
            self.grid[y][x] = 0;

            let size = self.grid.len();
            let mut adjacent_indexes: Vec<(usize, usize)> = vec![];

            for i in 0..=2
            {
                for j in 0..=2
                {
                    if x + i < 1 || y + j < 1 || x + i > size || y + j > size || (i == 1 && j == 1)
                    {
                        continue;
                    }

                    adjacent_indexes.push((x + i - 1, y + j - 1));
                }
            }
    
            for adjacent_index in adjacent_indexes
            {
                self.propagate_flash(adjacent_index.0, adjacent_index.1, true);
            }
        }
    }

    fn all_flashing(&mut self) -> bool
    {
        let size = self.grid.len();
        
        for y in 0..size
        {
            for x in 0..size
            {
                if self.flashed[y][x] == false
                {
                    return false;
                }
            }
        }

        return true;
    }
}

fn main()
{
    let input = util::read_input_file("day11.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let array = convert_to_int_array(&lines);
    let mut grid = Grid { grid: array, flashed: vec![], num_of_flashes: 0 };

    let num_of_flashes = get_num_of_flashes(&mut grid,  100);
    println!("Part 1: Solution={}", num_of_flashes);

    let step = get_step_where_all_flashing(&mut grid, 1000);
    println!("Part 2: Solution={}", step);
}

fn convert_to_int_array(lines: &Vec<String>) -> Vec<Vec<i32>>
{
    let size = lines.len();
    let mut array: Vec<Vec<i32>> = vec![vec![0; size]; size];

    for y in 0..lines.len()
    {
        let numbers: Vec<i32> = lines[y].chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        for x in 0..numbers.len()
        {
            array[y][x] = numbers[x];
        }
    }

    return array;
}

fn get_num_of_flashes(grid: &mut Grid, steps: usize) -> u64
{
    for _ in 0..steps
    {
        grid.step();
    }

    return grid.num_of_flashes;
}

fn get_step_where_all_flashing(grid: &mut Grid, max_steps: usize) -> usize
{
    for step in 1..=max_steps
    {
        grid.step();
        if grid.all_flashing()
        {
            return step;
        }
    }

    return 0;
}

#[test]
fn test_day11_part1_example()
{
    let input = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let array = convert_to_int_array(&lines);
    let mut grid = Grid { grid: array, flashed: vec![], num_of_flashes: 0 };
    let num_of_flashes = get_num_of_flashes(&mut grid, 100);

    assert_eq!(num_of_flashes, 1656);
}

#[test]
fn test_day11_part1_solution()
{
    let input = util::read_input_file("day11.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let array = convert_to_int_array(&lines);
    let mut grid = Grid { grid: array, flashed: vec![], num_of_flashes: 0 };
    let num_of_flashes = get_num_of_flashes(&mut grid,  100);

    assert_eq!(num_of_flashes, 1743);
}

#[test]
fn test_day11_part2_example()
{
    let input = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let array = convert_to_int_array(&lines);
    let mut grid = Grid { grid: array, flashed: vec![], num_of_flashes: 0 };
    let step = get_step_where_all_flashing(&mut grid, 1000);

    assert_eq!(step, 195);
}

#[test]
fn test_day11_part2_solution()
{
    let input = util::read_input_file("day11.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let array = convert_to_int_array(&lines);
    let mut grid = Grid { grid: array, flashed: vec![], num_of_flashes: 0 };
    let step = get_step_where_all_flashing(&mut grid, 1000);

    assert_eq!(step, 364);
}