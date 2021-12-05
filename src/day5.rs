mod util;

struct Point 
{
    x: i32,
    y: i32,
}

trait LineLogic
{
    fn is_horizontal(&self) -> bool;
    fn is_vertical(&self) -> bool;
    fn is_diagonal(&self) -> bool;
    fn get_all_points(&self) -> Vec<Point>;
}

struct Line
{
    start: Point,
    end: Point,
}

impl LineLogic for Line
{
    fn is_horizontal(&self) -> bool
    {
        return self.start.y == self.end.y;
    }

    fn is_vertical(&self) -> bool
    {
        return self.start.x == self.end.x;
    }

    fn is_diagonal(&self) -> bool
    {
        return !self.is_vertical() && !self.is_horizontal();
    }

    fn get_all_points(&self) -> Vec<Point>
    {
        let mut points: Vec<Point> = vec![];

        let range_x = self.end.x - self.start.x;
        let step_x = if range_x > 0 { 1 } else if range_x == 0 { 0 } else { -1 };
        let range_y = self.end.y - self.start.y;
        let step_y = if range_y > 0 { 1 } else if range_y == 0 { 0 } else { -1 };

        let mut x = self.start.x;
        let mut y = self.start.y;

        let point = Point { x: x, y: y };
        points.push(point);

        while x != self.end.x || y != self.end.y
        {
            x += step_x;
            y += step_y;
            let point = Point { x: x, y: y };
            points.push(point);
        }

        return points;
    }
}

fn main()
{
    let input = util::read_input_file("day5.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let parsed_lines = parse_lines(&lines);
    let canvas = draw_lines_on_canvas(&parsed_lines, 1000, true);
    let count = count_values_greater(&canvas, 1);

    println!("Part 1: Solution={}", count);

    let canvas_with_diagonal = draw_lines_on_canvas(&parsed_lines, 1000, false);
    let count_with_diagonal = count_values_greater(&canvas_with_diagonal, 1);

    println!("Part 2: Solution={}", count_with_diagonal);
}

fn parse_lines(lines: &Vec<String>) -> Vec<Line>
{
    let mut parsed_lines: Vec<Line> = vec![];

    for line_string in lines
    {
        let parts = util::split_string_by_string(&line_string, " -> ");
        assert_eq!(parts.len(), 2);

        let start_coordinates = util::split_ints_by_string(&parts[0], ",");
        let end_coordinates = util::split_ints_by_string(&parts[1], ",");

        let start_point = Point { x: start_coordinates[0], y: start_coordinates[1] };
        let end_point = Point { x: end_coordinates[0], y: end_coordinates[1] };
        let parsed_line = Line { start: start_point, end: end_point };
        parsed_lines.push(parsed_line);
    }

    return parsed_lines;
}

fn draw_lines_on_canvas(lines: &Vec<Line>, size: usize, ignore_diagonal: bool) -> Vec<Vec<i32>>
{
    let mut canvas = vec![vec![0; size]; size];

    for line in lines
    {
        if line.is_diagonal() && ignore_diagonal
        {
            continue;
        }

        let points = line.get_all_points();
        for point in points
        {
            canvas[point.x as usize][point.y as usize] += 1;
        }
    }

    return canvas;
}

fn count_values_greater(canvas: &Vec<Vec<i32>>, greater_as: i32) -> i32
{
    let mut count = 0;

    for x in 0..canvas.len()
    {
        for y in 0..canvas[x].len()
        {
            if canvas[x][y] > greater_as
            {
                count += 1;
            }
        }
    }

    return count;
}

#[test]
fn test_day5_part1_example()
{
    let input = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let parsed_lines = parse_lines(&lines);
    let canvas = draw_lines_on_canvas(&parsed_lines, 10, true);
    let count = count_values_greater(&canvas, 1);

    assert_eq!(count, 5);
}

#[test]
fn test_day5_part1_solution()
{
    let input = util::read_input_file("day5.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let parsed_lines = parse_lines(&lines);
    let canvas = draw_lines_on_canvas(&parsed_lines, 1000, true);
    let count = count_values_greater(&canvas, 1);

    assert_eq!(count, 5632);
}

#[test]
fn test_day5_part2_example()
{
    let input = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let parsed_lines = parse_lines(&lines);
    let canvas = draw_lines_on_canvas(&parsed_lines, 10, false);
    let count = count_values_greater(&canvas, 1);

    assert_eq!(count, 12);
}

#[test]
fn test_day5_part2_solution()
{
    let input = util::read_input_file("day5.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let parsed_lines = parse_lines(&lines);
    let canvas = draw_lines_on_canvas(&parsed_lines, 1000, false);
    let count = count_values_greater(&canvas, 1);

    assert_eq!(count, 22213);
}