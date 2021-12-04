use std::fs;

#[allow(dead_code)]
pub fn read_input_file(input_file: &str) -> String
{
    let mut directory = std::env::current_exe().unwrap();
    directory.pop();

    // Unit Tests are executed in the "deps" subdirectory:
    // "target/debug/deps" for unit tests
    // "target/debug" for standard execution
    let directory_name = directory.file_name().unwrap();
    let is_executed_from_unit_test: bool = directory_name == "deps";
    if is_executed_from_unit_test
    {
        directory.pop();
    }

    let file_path = directory.join(input_file);
    let contents = fs::read_to_string(file_path).expect("Error while reading input file");
    
    return contents;
}

#[allow(dead_code)]
pub fn get_lines_from_string(contents: String, split_by: &str) -> Vec<String>
{
    let mut lines: Vec<String> = contents.split(split_by).map(String::from).collect();
    lines.retain(|x| !x.is_empty());

    return lines;
}

#[allow(dead_code)]
pub fn parse_lines_as_ints(lines: &Vec<String>) -> Vec<i32>
{
    let mut ints: Vec<i32> = vec![];
    for line in lines
    {
        let number = line.trim().parse::<i32>().unwrap();
        ints.push(number);
    }

    return ints;
}