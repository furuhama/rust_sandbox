// test /my_module/pointer::ref_cell()

#[cfg(test)]
mod test {
    use crate::my_module::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }

            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // << cannot mutably borrow immutable field `sent_messages` >>
            // self.sent_messages.push(String::from(message));

            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
