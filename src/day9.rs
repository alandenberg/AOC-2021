use itertools::Itertools;
use std::collections::HashSet;

mod util;

fn main()
{
    let input = util::read_input_file("day9.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let heightmap = convert_to_heightmap(&lines);

    let low_points = get_low_points(&heightmap);
    let risk_sum = get_risk_level_sum(&low_points);
    println!("Part 1: Solution={}", risk_sum);

    let basin_sizes = get_basin_sizes(&heightmap, &low_points);
    let size_product = get_basin_size_product(&basin_sizes, 3);
    println!("Part 2: Solution={}", size_product);
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

fn get_basin_sizes(heightmap: &Vec<Vec<i32>>, low_points: &Vec<(i32, usize, usize)>) -> Vec<i32>
{
    let mut basin_sizes: Vec<i32> = vec![];

    for low_point in low_points
    {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let basin_size = get_basin_size(&heightmap, low_point.1, low_point.2, &mut visited);
        basin_sizes.push(basin_size);
    }

    return basin_sizes;
}

fn get_basin_size(heightmap: &Vec<Vec<i32>>, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i32
{
    if heightmap[y][x] == 9
    {
        return 0;
    }

    let mut basin_size = 1;
    visited.insert((x, y));

    if x < heightmap[y].len()-1 && !visited.contains(&(x+1, y))
    {
        basin_size += get_basin_size(heightmap, x+1, y, visited);
    }

    if x > 0 && !visited.contains(&(x-1, y))
    {
        basin_size += get_basin_size(heightmap, x-1, y, visited);
    }

    if y < heightmap.len()-1 && !visited.contains(&(x, y+1))
    {
        basin_size += get_basin_size(heightmap, x, y+1, visited);
    }

    if y > 0 && !visited.contains(&(x, y-1))
    {
        basin_size += get_basin_size(heightmap, x, y-1, visited);
    }

    return basin_size;
}

fn get_basin_size_product(basin_sizes: &Vec<i32>, n: usize) -> u64
{
    let basin_sizes_ordered: Vec<u64> = basin_sizes.iter().sorted().rev().map(|x| *x as u64).collect();
    let size_product: u64 = basin_sizes_ordered[0..n].iter().product::<u64>();
    return size_product;
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


#[test]
fn test_day9_part2_example()
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
    let basin_sizes = get_basin_sizes(&heightmap, &low_points);
    let size_product = get_basin_size_product(&basin_sizes, 3);

    assert_eq!(size_product, 1134);
}

#[test]
fn test_day9_part2_solution()
{
    let input = util::read_input_file("day9.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let heightmap = convert_to_heightmap(&lines);

    let low_points = get_low_points(&heightmap);
    let basin_sizes = get_basin_sizes(&heightmap, &low_points);
    let size_product = get_basin_size_product(&basin_sizes, 3);

    assert_eq!(size_product, 1019494);
}