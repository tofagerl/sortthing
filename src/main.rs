use std::fs;
use std::time::{Duration, Instant};

mod tomsort;

fn main() {
    let file_contents = fs::read_to_string("test_strings.json").expect("No such file 'testdata.json'");
    let list: Vec<String> = serde_json::from_str(&file_contents).expect("Could not deserialize file contents");
    let result = run_all_sorts(&list);
    println!("{}", result)
}

fn run_all_sorts<T: PartialOrd + Clone>(list: &Vec<T>) -> String {
    let mut result_string = "".to_owned();
    let tom_sort = run_a_sort(&list.to_vec(), "tom sort", tomsort::sort);
    result_string.push_str(&tom_sort);
    return result_string;
}

fn run_a_sort<T: PartialOrd + Clone>(list: &Vec<T>, name_of_algorithm: &str, algorithm: fn(&mut Vec<T>)) -> String {
    let mut mutable_list = list.to_vec();
    let start = Instant::now();
    algorithm(&mut mutable_list);
    let duration = start.elapsed();
    return format!("{}: {:#?}", name_of_algorithm, duration);
}



