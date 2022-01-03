use std::cmp::PartialEq;

pub fn linear_search<T: PartialEq>(arr: &[T], key: &T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if item == key {
            return Some(index);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_strings() {
        let index = linear_search(&vec!["a", "b", "c", "d", "google", "zoo"], &"a");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = linear_search(&vec![1, 2, 3, 4], &4);
        assert_eq!(index, Some(3));

        let index = linear_search(&vec![1, 2, 3, 4], &3);
        assert_eq!(index, Some(2));

        let index = linear_search(&vec![1, 2, 3, 4], &2);
        assert_eq!(index, Some(1));

        let index = linear_search(&vec![1, 2, 3, 4], &1);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = linear_search(&vec![1, 2, 3, 4], &5);
        assert_eq!(index, None);
    }

    #[test]
    fn empty() {
        let index = linear_search(&vec![], &1);
        assert_eq!(index, None);
    }
}
