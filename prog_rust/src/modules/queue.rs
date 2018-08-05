pub fn queue() {
    let mut q = Queue::<char>::new();

    q.push('0');
    q.push('1');
    // assert_eq!(q.pop(), Some('0'));
    println!("{:?}", q.pop());

    q.push('=');
    // assert_eq!(q.pop(), Some('1'));
    println!("{:?}", q.pop());

    // assert_eq!(q.pop(), Some('='));
    println!("{:?}", q.pop());

    // assert_eq!(q.pop(), None);
    println!("{:?}", q.pop());
}

// first-in, first-out
struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.older.is_empty() {
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}
