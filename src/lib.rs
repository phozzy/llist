use std::iter::{Iterator, FusedIterator};

enum LList {
    Cons(i32, Box<LList>),
    Nil,
}

struct LListIter<'a> {
    list: Option<&'a LList>,
}

impl Iterator for LListIter<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            Some(&LList::Cons(head, ref tail)) => {
                self.list = Some(tail);
                Some(head)
            },
            _ => None
        }
    }
}

impl LList {
    fn iter(&self) -> LListIter {
        LListIter { list: Some(self) }
    }
}

impl FusedIterator for LListIter<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llist_iterator() {
        let list = LList::Cons(1, Box::new(LList::Cons(2, Box::new(LList::Cons(3, Box::new(LList::Nil))))));
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_llist_iterator_empty() {
        let list = LList::Nil;
        let mut iter = list.iter();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_llist_iterator_fused() {
        let list = LList::Cons(1, Box::new(LList::Cons(2, Box::new(LList::Cons(3, Box::new(LList::Nil))))));
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

}
