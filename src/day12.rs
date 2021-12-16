use std::collections::HashMap;
use std::collections::HashSet;

mod util;

#[derive(Clone)]
struct Cave
{
    name: String,
    connections: HashSet<String>,
    is_big: bool,
}

fn main()
{
    let input = util::read_input_file("day12.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);

    let paths_once = find_paths_once(&caves);
    println!("Part 1: Solution={}", paths_once);

    let paths_twice = find_paths_twice(&caves);
    println!("Part 2: Solution={}", paths_twice);
}

fn parse_caves(lines: &Vec<String>) -> HashMap<String, Cave>
{
    let mut all_caves: HashMap<String, Cave> = HashMap::new();

    for line in lines
    {
        let parts = util::split_string_by_string(line, "-");
        assert_eq!(parts.len(), 2);

        let is_big1 = parts[0].to_ascii_uppercase() == parts[0];
        let mut cave1 = if all_caves.contains_key(&parts[0]) { all_caves[&parts[0]].clone() } else { Cave { name: parts[0].to_string(), connections: HashSet::new(), is_big: is_big1 } };

        let is_big2 = parts[1].to_ascii_uppercase() == parts[1];
        let mut cave2 = if all_caves.contains_key(&parts[1]) { all_caves[&parts[1]].clone() } else { Cave { name: parts[1].to_string(), connections: HashSet::new(), is_big: is_big2 } };

        cave1.connections.insert(cave2.name.to_string());
        cave2.connections.insert(cave1.name.to_string());

        all_caves.insert(cave1.name.to_string(), cave1);
        all_caves.insert(cave2.name.to_string(), cave2);
    }

    return all_caves;
}

fn find_paths_once(caves: &HashMap<String, Cave>) -> usize
{
    return count_sub_paths_once("start", &caves, &mut vec![]);
}

fn count_sub_paths_once(cave_name: &str, caves: &HashMap<String, Cave>, visited: &mut Vec<String>) -> usize
{
    if cave_name == "end"
    {
        return 1;
    }

    let cave = caves.get(cave_name).unwrap();
    if cave.is_big == false
    {
        visited.push(cave_name.to_string());
    }

    let mut count: usize = 0;
    for connected_cave_name in &cave.connections 
    {
        if !visited.contains(connected_cave_name)
        {
            count += count_sub_paths_once(connected_cave_name, caves, &mut visited.clone());
        }
    }

    return count;
}

fn find_paths_twice(caves: &HashMap<String, Cave>) -> usize
{
    let small_caves = caves.values().filter(|x| x.name != "start" && x.name != "end" && !x.is_big);

    // The number of possible paths consists of:
    // paths_that_visit_small_caves_only_once + paths_that_visit_small_caves_twice
    let once_path_count = count_sub_paths_once("start", caves, &mut vec![]);

    let mut path_count: usize = once_path_count;
    for small_cave in small_caves
    {
        // We have to subtract once_path_count because we already added it once at the beginning and don't want to count it several times
        path_count += count_sub_paths_twice("start", caves, &mut vec![], &small_cave.name, &mut false) - once_path_count;
    }

    return path_count;
}

fn count_sub_paths_twice(cave_name: &str, caves: &HashMap<String, Cave>, visited: &mut Vec<String>, double_visit_cave_name: &str, has_double_visited: &mut bool) -> usize
{
    if cave_name == "end"
    {
        return 1;
    }

    let cave = caves.get(cave_name).unwrap();
    if cave.is_big == false
    {
        if double_visit_cave_name == cave_name
        {
            if *has_double_visited
            {
                visited.push(cave_name.to_string());
            }
            *has_double_visited = true;
        }
        else
        {
            visited.push(cave_name.to_string()); 
        }
    }

    let mut count: usize = 0;
    for connected_cave_name in &cave.connections 
    {
        if !visited.contains(connected_cave_name)
        {
            count += count_sub_paths_twice(connected_cave_name, caves, &mut visited.clone(), double_visit_cave_name, &mut has_double_visited.clone());
        }
    }

    return count;
}

#[test]
fn test_day12_part1_example1()
{
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_once(&caves);

    assert_eq!(paths, 10);
}

#[test]
fn test_day12_part1_example2()
{
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_once(&caves);

    assert_eq!(paths, 19);
}

#[test]
fn test_day12_part1_example3()
{
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_once(&caves);

    assert_eq!(paths, 226);
}

#[test]
fn test_day12_part1_solution()
{
    let input = util::read_input_file("day12.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_once(&caves);

    assert_eq!(paths, 4754);
}

#[test]
fn test_day12_part2_example1()
{
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_twice(&caves);

    assert_eq!(paths, 36);
}

#[test]
fn test_day12_part2_example2()
{
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_twice(&caves);

    assert_eq!(paths, 103);
}

#[test]
fn test_day12_part2_example3()
{
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW".to_string();
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_twice(&caves);

    assert_eq!(paths, 3509);
}

#[test]
fn test_day12_part2_solution()
{
    let input = util::read_input_file("day12.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let caves = parse_caves(&lines);
    let paths = find_paths_twice(&caves);

    assert_eq!(paths, 143562);
}