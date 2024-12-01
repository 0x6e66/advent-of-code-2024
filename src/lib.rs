use std::str::FromStr;

pub const INPUT_PATH: &str = "input.txt";
pub const TEST_INPUT_PATH: &str = "test_input.txt";

pub fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("File '{file_name}' not found!"))
        .lines()
        .map(|x| x.parse())
        .collect()
}
