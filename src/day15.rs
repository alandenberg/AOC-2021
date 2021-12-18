mod util;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

trait NodeLogic {
    fn get_neighbors(&mut self, row_size: usize) -> Vec<usize>;
    fn update_cost(&mut self, new_cost: i32, new_predecessor: usize);
}

#[derive(Clone, Copy, Debug)]
struct Node {
    index: usize,
    enter_cost: i32,
    cost: i32,
    predecessor: usize,
}

impl NodeLogic for Node {
    fn get_neighbors(&mut self, row_size: usize) -> Vec<usize> {
        let mut neighbors = vec![];

        if (self.index + 1) % row_size > 0 && self.predecessor != self.index + 1 {
            neighbors.push(self.index + 1); // right
        }

        if self.index % row_size > 0 && self.predecessor != self.index - 1 {
            neighbors.push(self.index - 1); // left
        }

        if self.index >= row_size && self.predecessor != self.index - row_size {
            neighbors.push(self.index - row_size); // up
        }

        if self.index + row_size < row_size * row_size && self.predecessor != self.index + row_size {
            neighbors.push(self.index + row_size); // down
        }

        return neighbors;
    }

    fn update_cost(&mut self, new_cost: i32, new_predecessor: usize) {
        self.cost = new_cost;
        self.predecessor = new_predecessor;
    }
}

#[derive(Clone, Copy, Debug)]
struct Candidate {
    index: usize,
    cost: i32,
}

fn main() {
    let input = util::read_input_file("day15.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let cave = parse_cave(&lines);

    let mut nodes = build_nodes(&cave);
    let cost = find_path_with_lowest_cost(&mut nodes);
    println!("Part 1: Solution={}", cost);

    let extended_cave = extend_cave(&cave, 5);
    let mut nodes = build_nodes(&extended_cave);
    let cost = find_path_with_lowest_cost(&mut nodes);
    println!("Part 2: Solution={}", cost);
}

fn parse_cave(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut cave: Vec<Vec<i32>> = vec![];

    for line in lines {
        let line = line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        cave.push(line);
    }

    return cave;
}

fn extend_cave(cave: &Vec<Vec<i32>>, size_factor: usize) -> Vec<Vec<i32>> {
    let original_size = cave.len();
    let new_size = original_size * size_factor;
    let mut extended_cave = vec![vec![0; new_size]; new_size];

    for step_y in 0..size_factor {
        for step_x in 0..size_factor {
            for y in 0..cave.len() {
                for x in 0..cave.len() {
                    let new_value = (cave[y][x] + step_y as i32 + step_x as i32 - 1) % 9 + 1;
                    extended_cave[y + step_y * original_size][x + step_x * original_size] = new_value;
                }
            }
        }
    }

    return extended_cave;
}

fn find_path_with_lowest_cost(nodes: &mut Vec<Node>) -> i32 {
    // Add start node to queue
    let mut queue = vec![Candidate { index: 0, cost: 0 }];
    let row_size = (nodes.len() as f64).sqrt() as usize;

    while queue.len() > 0 {
        dijkstra_step(nodes, &mut queue, row_size);
    }

    // Lowest cost is stored in target node
    return nodes[row_size * row_size - 1].cost;
}

fn dijkstra_step(nodes: &mut Vec<Node>, queue: &mut Vec<Candidate>, row_size: usize) {
    queue.sort_by(|x, y| x.cost.cmp(&y.cost));

    let mut current_node = nodes[queue[0].index];

    if current_node.index == row_size * row_size - 1 {
        queue.clear();
        return;
    }

    let neighbors = current_node.get_neighbors(row_size);

    for neighbor_index in neighbors {
        let neighbor_node = &mut nodes[neighbor_index];
        let new_cost = current_node.cost + neighbor_node.enter_cost;
        if new_cost < neighbor_node.cost {
            neighbor_node.update_cost(new_cost, current_node.index);
            queue.push(Candidate { index: neighbor_index, cost: new_cost });
        }
    }

    queue.remove(0);
}

fn build_nodes(cave: &Vec<Vec<i32>>) -> Vec<Node> {
    let mut nodes = vec![];

    let size = cave.len();
    for y in 0..size {
        for x in 0..size {
            // predecessor > size*size-1 means that we have no predecessor because it would be out of bounds
            nodes.push(Node { index: nodes.len(), enter_cost: cave[y][x], cost: i32::MAX, predecessor: size*size });
        }
    }

    // Special case: start node
    nodes[0].enter_cost = 0;
    nodes[0].cost = 0;

    return nodes;
}

#[test]
fn test_day15_part1_example() {
    let input = String::from("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");
    let lines = util::split_string_by_string(&input, "\n");
    let cave = parse_cave(&lines);
    let mut nodes = build_nodes(&cave);
    let cost = find_path_with_lowest_cost(&mut nodes);

    assert_eq!(cost, 40);
}

#[test]
fn test_day15_part1_solution() {
    let input = util::read_input_file("day15.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let cave = parse_cave(&lines);
    let mut nodes = build_nodes(&cave);
    let cost = find_path_with_lowest_cost(&mut nodes);

    assert_eq!(cost, 415);
}

#[test]
fn test_day15_part2_example() {
    let input = String::from("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");
    let lines = util::split_string_by_string(&input, "\n");
    let cave = parse_cave(&lines);
    let extended_cave = extend_cave(&cave, 5);
    let mut nodes = build_nodes(&extended_cave);
    let cost = find_path_with_lowest_cost(&mut nodes);

    assert_eq!(cost, 315);
}

#[test]
fn test_day15_part2_solution() {
    let input = util::read_input_file("day15.txt");
    let lines = util::split_string_by_string(&input, "\n");
    let cave = parse_cave(&lines);
    let extended_cave = extend_cave(&cave, 5);
    let mut nodes = build_nodes(&extended_cave);
    let cost = find_path_with_lowest_cost(&mut nodes);

    assert_eq!(cost, 2864);
}