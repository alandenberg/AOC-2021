mod util;

fn main()
{
    let input = util::read_input_file("day9.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let heightmap = convert_to_heightmap(&lines);

    let low_points = get_low_points(&heightmap);
    let risk_sum = get_risk_level_sum(&low_points);

    println!("Part 1: Solution={}", risk_sum);
}

fn convert_to_heightmap(lines: &Vec<String>) -> Vec<Vec<i32>>
{
    let mut heightmap: Vec<Vec<i32>> = vec![vec![0; lines[0].len()]; lines.len()];

    for y in 0..lines.len()
    {
        let characters: Vec<char> = lines[y].chars().collect();
        for x in 0..characters.len()
        {
            heightmap[y][x] = characters[x].to_digit(10).unwrap() as i32;
        }
    }

    return heightmap;
}

fn get_low_points(heightmap: &Vec<Vec<i32>>) -> Vec<(i32, usize, usize)>
{
    let mut low_points: Vec<(i32, usize, usize)> = vec![];

    for y in 0..heightmap.len()
    {
        for x in 0..heightmap[y].len()
        {
            let height = heightmap[y][x];

            if x > 0 && heightmap[y][x-1] <= height || x < heightmap[y].len()-1 && heightmap[y][x+1] <= height
            {
                continue;
            }

            if y > 0 && heightmap[y-1][x] <= height || y < heightmap.len()-1 && heightmap[y+1][x] <= height
            {
                continue;
            }

            low_points.push((height, x, y));
        }
    }

    return low_points;
}

fn get_risk_level_sum(low_points: &Vec<(i32, usize, usize)>) -> i32
{
    return low_points.iter().map(|x| x.0 + 1).sum();
}

#[test]
fn test_day9_part1_example()
{
    let input = "
2199943210
3987894921
9856789892
8767896789
9899965678".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let heightmap = convert_to_heightmap(&lines);

    let low_points = get_low_points(&heightmap);
    let risk_sum = get_risk_level_sum(&low_points);

    assert_eq!(risk_sum, 15);
}

#[test]
fn test_day9_part1_solution()
{
    let input = util::read_input_file("day9.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let heightmap = convert_to_heightmap(&lines);

    let low_points = get_low_points(&heightmap);
    let risk_sum = get_risk_level_sum(&low_points);

    assert_eq!(risk_sum, 530);
}