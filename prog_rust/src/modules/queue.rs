pub fn queue() {
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

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
struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }
}
