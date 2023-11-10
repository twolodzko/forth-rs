#[derive(Clone, Copy, Debug)]
pub struct Tail<T, const N: usize> {
    data: [T; N],
    position: usize,
    length: usize,
}

impl<T, const N: usize> Tail<T, N> {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[inline]
    pub fn insert(&mut self, value: T) {
        self.position = (self.position + 1) % N;
        self.data[self.position] = value;
        self.length = (self.length + 1).min(N);
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if self.is_empty() || index >= self.length {
            return None;
        }

        let index = (self.position + N - index) % N;
        let farthest = (self.position + self.length) % N;

        // Example:
        //    index out of bounds
        //     /
        // [x _ _ x x x]
        //  ^
        // position
        if index <= self.position || index >= farthest {
            Some(&self.data[index])
        } else {
            None
        }
    }

    #[inline]
    pub fn last(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        Some(&self.data[self.position])
    }

    #[inline]
    pub fn remove_last(&mut self) {
        if !self.is_empty() {
            self.position = (self.position + N - 1) % N;
            self.length -= 1;
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.position = N - 1;
        self.length = 0;
    }
}

impl<T, const N: usize> Default for Tail<T, N>
where
    T: Copy + Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            data: [T::default(); N],
            // initialize at Nth position, so the first value will be inserted at (N + 1) % N = 0
            position: N - 1,
            length: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tail;

    #[test]
    fn empty() {
        let mut tail: Tail<char, 5> = Tail::default();

        // is it empty?
        assert!(tail.is_empty());
        assert!(tail.length == 0);

        // can I extract some values?
        assert!(tail.get(0).is_none());
        assert!(tail.get(1).is_none());
        assert!(tail.get(10).is_none());

        // removing elements does not break anything
        for _ in 0..10 {
            tail.remove_last();
        }
    }

    #[test]
    fn insert_some_values() {
        let mut tail: Tail<char, 5> = Tail::default();

        tail.insert('a');
        assert_eq!(tail.get(0), Some(&'a'));
        assert!(tail.get(1).is_none());

        tail.insert('b');
        assert_eq!(tail.get(0), Some(&'b'));
        assert_eq!(tail.get(1), Some(&'a'));
        assert!(tail.get(2).is_none());

        tail.insert('c');
        assert_eq!(tail.get(0), Some(&'c'));
        assert_eq!(tail.get(1), Some(&'b'));
        assert_eq!(tail.get(2), Some(&'a'));
        assert!(tail.get(3).is_none());

        tail.remove_last();
        assert_eq!(tail.get(0), Some(&'b'));
        assert_eq!(tail.get(1), Some(&'a'));
        assert!(tail.get(2).is_none());

        tail.remove_last();
        assert_eq!(tail.get(0), Some(&'a'));
        assert!(tail.get(1).is_none());

        tail.remove_last();
        assert!(tail.is_empty());
    }

    #[test]
    fn insertions_and_deletions() {
        let mut tail: Tail<usize, 5> = Tail::default();

        for i in 0..5 {
            tail.insert(i);
        }
        // [0 1 2 3 4]
        //          ^
        assert_eq!(tail.length, 5);
        assert_eq!(Some(&4), tail.get(0));
        assert_eq!(Some(&0), tail.get(4));
        assert!(tail.get(5).is_none());

        for i in 5..8 {
            tail.insert(i);
        }
        // [5 6 7 3 4]
        //      ^
        assert_eq!(tail.length, 5);
        assert_eq!(Some(&7), tail.get(0));
        assert_eq!(Some(&5), tail.get(2));
        assert_eq!(Some(&4), tail.get(3));
        assert_eq!(Some(&3), tail.get(4));
        assert!(tail.get(5).is_none());

        tail.remove_last();
        // [5 6 _ 3 4]
        //    ^
        assert_eq!(tail.length, 4);
        assert_eq!(Some(&6), tail.get(0));
        assert_eq!(Some(&5), tail.get(1));
        assert!(tail.get(5).is_none());

        tail.remove_last();
        // [5 _ _ 3 4]
        //  ^
        assert_eq!(tail.length, 3);
        assert_eq!(Some(&5), tail.get(0));
        assert_eq!(Some(&4), tail.get(1));
        assert_eq!(Some(&3), tail.get(2));
        assert!(tail.get(4).is_none());
        assert!(tail.get(5).is_none());

        for i in 8..12 {
            tail.insert(i);
        }
        // [5 8 9 10 11]
        //           ^
        assert_eq!(tail.length, 5);
        assert_eq!(Some(&11), tail.get(0));
        assert_eq!(Some(&10), tail.get(1));
        assert_eq!(Some(&5), tail.get(4));

        tail.remove_last();
        // [5 8 9 10 _]
        //        ^
        assert_eq!(tail.length, 4);
        assert_eq!(Some(&10), tail.get(0));
        assert_eq!(Some(&8), tail.get(2));
        assert!(tail.get(5).is_none());

        for _ in 0..5 {
            tail.remove_last();
        }
        assert!(tail.is_empty());
    }
}
