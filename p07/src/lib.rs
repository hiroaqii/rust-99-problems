#[derive(Debug)]
pub enum Item<T> {
    One(T),
    Many(Vec<Item<T>>),
}

//Flatten a nested list structure.
//Transform a list, possibly holding lists as elements into a `flat' list by replacing each list with its elements (recursively).
pub fn flatten<T: Copy>(items: &Item<T>) -> Vec<T> {
    match items {
        Item::One(t) => vec![*t],
        Item::Many(vec) => vec
            .iter()
            .flat_map(|item| flatten(&item).into_iter())
            .collect::<Vec<T>>(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let item = Item::One("a");
        assert_eq!(flatten(&item), vec!["a"]);
    }

    #[test]
    fn test2() {
        let item = Item::Many(vec![
            Item::One("a"),
            Item::Many(vec![
                Item::One("b"),
                Item::Many(vec![Item::One("c"), Item::One("d")]),
                Item::One("e"),
            ]),
        ]);
        println!("{:?}", item);
        assert_eq!(flatten(&item), vec!["a", "b", "c", "d", "e"]);
    }
}
