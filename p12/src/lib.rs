/// Decode a run-length encoded list.
/// Given a run-length code list generated as specified in problem P11. Construct its uncompressed version.

#[derive(Debug, PartialEq)]
pub enum Item<T: Copy + PartialEq> {
    One(T),
    Many((usize, T)),
}

pub fn decode<T: Copy + PartialEq>(vec: &Vec<Item<T>>) -> Vec<T> {
    let mut ret: Vec<T> = Vec::new();
    for item in vec {
        match *item {
            Item::One(t) => ret.push(t),
            Item::Many((n, t)) => {
                for _ in 1..=n {
                    ret.push(t);
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let vec = vec![
            Item::Many((4, "a")),
            Item::One("b"),
            Item::Many((2, "c")),
            Item::Many((2, "a")),
            Item::One("d"),
            Item::Many((4, "e")),
        ];
        let ret = vec![
            "a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e",
        ];
        assert_eq!(decode(&vec), ret);
    }
}
