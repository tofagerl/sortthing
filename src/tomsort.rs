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
    #[test]
    fn simple_sort_test() {
        let mut testData = vec!(5, 4, 2, 3, 1);
        let correctResult = vec!(1, 2, 3, 4, 5);
        let incorrectResult = vec!(2, 2, 2, 2, 2);
        sort(&mut testData);
        assert_eq!(vec_compare(&correctResult, &testData), true);
        assert_eq!(vec_compare(&incorrectResult, &testData), false);
    }
    fn vec_compare(va: &Vec<PartialOrd>, vb: &Vec<PartialOrd>) -> bool {
        (va.len() == vb.len()) &&
            va.iter()
                .zip(vb)
                .all(|(a, b)| a == b)
    }
}

