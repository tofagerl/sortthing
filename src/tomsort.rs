pub fn sort<T: PartialOrd>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        let mut small = i;
        for j in (i + 1)..list.len() {
            if list[j] < list[small] {
                small = j;
            }
        }
        list.swap(small, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn simple_sort_test() {
        let mut test_data = vec!(5, 4, 2, 3, 1);
        let correct_result = vec!(1, 2, 3, 4, 5);
        let incorrect_result = vec!(2, 2, 2, 2, 2);
        sort(&mut test_data);
        assert_eq!(vec_compare(&correct_result, &test_data), true);
        assert_eq!(vec_compare(&incorrect_result, &test_data), false);
    }

    #[test]
    fn word_sort_test() {
        let mut file_contents = fs::read_to_string("test_strings.json").expect("No such file 'testdata.json'");
        let mut list: Vec<String> = serde_json::from_str(&file_contents).expect("Could not deserialize contents");
        sort(&mut list)
    }

    fn vec_compare<T>(va: &Vec<T>, vb: &Vec<T>) -> bool
        where T: std::cmp::PartialOrd{
        (va.len() == vb.len()) &&
            va.iter()
                .zip(vb)
                .all(|(a, b)| a == b)
    }
}

