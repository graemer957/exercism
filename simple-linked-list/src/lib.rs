struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while current.is_some() {
            count += 1;
            current = &current.as_ref().unwrap().next;
        }

        count
    }

    pub fn push(&mut self, element: T) {
        let node = Some(Box::new(Node {
            value: element,
            next: None,
        }));

        match self.head.as_mut() {
            None => self.head = node,
            Some(mut current) => {
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = node;
            }
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.as_ref()?;

        if self.head.as_ref().unwrap().next.is_none() {
            return Some(self.head.take().unwrap().value);
        }

        let mut last = self.head.as_mut().unwrap();
        while let Some(node) = last.next.as_ref() {
            if node.next.is_none() {
                break;
            }
            last = last.next.as_mut().unwrap();
        }

        Some(last.next.take().unwrap().value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref()?;

        if self.head.as_ref()?.next.is_none() {
            return Some(&self.head.as_ref()?.value);
        }

        let mut last = self.head.as_ref().unwrap();
        while let Some(node) = last.next.as_ref() {
            if node.next.is_none() {
                break;
            }
            last = last.next.as_ref().unwrap();
        }

        Some(&last.next.as_ref()?.value)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();

        while let Some(node) = self.pop() {
            result.push(node);
        }

        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = SimpleLinkedList::new();

        for item in iter {
            result.push(item);
        }

        result
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result = Self::new();

        let mut next = linked_list.head.take();
        while let Some(node) = next {
            result.push(node.value);
            next = node.next;
        }

        result
    }
}
