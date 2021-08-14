/// Pack consecutive duplicates of list elements into sublists.
/// If a list contains repeated elements they should be placed in separate sublists.
pub fn pack<T: Copy + PartialEq>(vec: &Vec<T>) -> Vec<Vec<T>> {
    let mut ret: Vec<Vec<T>> = Vec::new();
    for x in vec {
        if ret.last().is_none() || ret.last().unwrap().last().unwrap() != x {
            ret.push(vec![*x])
        } else {
            ret.last_mut().unwrap().push(*x);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec: Vec<i32> = Vec::new();
        let ret: Vec<Vec<i32>> = vec![];
        assert_eq!(pack(&vec), ret);
    }

    #[test]
    fn test2() {
        let vec = vec!["a"];
        assert_eq!(pack(&vec), vec![vec!["a"]]);
    }

    #[test]
    fn test3() {
        let vec = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        assert_eq!(
            pack(&vec),
            vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['b'],
                vec!['c', 'c'],
                vec!['a', 'a'],
                vec!['d'],
                vec!['e', 'e', 'e', 'e']
            ]
        );
    }
}
