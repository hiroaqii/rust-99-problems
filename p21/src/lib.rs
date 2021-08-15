/// Insert an element at a given position into a list.

fn insert_at<T: Copy>(v: T, vec: &Vec<T>, n: usize) -> Vec<T> {
    let mut ret = vec.clone();
    ret.insert(n - 1, v);
    ret
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec = vec!["a", "b", "c", "d"];
        assert_eq!(insert_at("alfa", &vec, 2), (vec!["a", "alfa", "b", "c", "d"]))
    }
}
