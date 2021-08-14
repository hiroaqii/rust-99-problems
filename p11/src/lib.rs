/// Modified run-length encoding.
/// Modify the result of problem P10 in such a way that if an element has no duplicates it is simply copied into the result list.
/// Only elements with duplicates are transferred as (N E) lists.

#[derive(Debug, PartialEq)]
pub enum Item<T: Copy + PartialEq> {
    One(T),
    Many((usize, T)),
}

pub fn encode_modified<T: Copy + PartialEq>(vec: &Vec<T>) -> Vec<Item<T>> {
    let mut ret: Vec<Item<T>> = Vec::new();
    match vec.len() {
        0 => ret,
        _ => {
            let mut n = 1;
            let mut t = &vec[0];
            for i in 1..vec.len() {
                if t == &vec[i] {
                    n += 1
                } else {
                    ret.push(get_item(n, t));
                    t = &vec[i];
                    n = 1;
                }
            }
            ret.push(get_item(n, t));
            ret
        }
    }
}

fn get_item<T: Copy + PartialEq>(n: usize, t: &T) -> Item<T> {
    match n {
        1 => Item::One(*t),
        _ => Item::Many((n, *t)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec: Vec<i32> = Vec::new();
        let ret: Vec<Item<i32>> = vec![];
        assert_eq!(encode_modified(&vec), ret);
    }

    #[test]
    fn test2() {
        let vec = vec![
            "a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e",
        ];

        assert_eq!(
            encode_modified(&vec),
            vec![
                Item::Many((4, "a")),
                Item::One("b"),
                Item::Many((2, "c")),
                Item::Many((2, "a")),
                Item::One("d"),
                Item::Many((4, "e"))
            ]
        )
    }
}
