use std::ops::{AddAssign, SubAssign};

#[derive(Default)]
pub struct Event {
    pub subscribers: Vec<fn()>
}

impl Event {
    pub fn subscribe(&mut self, event_delegate: fn()) {
        self.subscribers.push(event_delegate);
    }
}

impl AddAssign<fn()> for Event {
    fn add_assign(&mut self, rhs: fn()) {
        self.subscribers.push(rhs);
    }
}

impl SubAssign<fn()> for Event {
    fn sub_assign(&mut self, rhs: fn()) {
        let index = self.subscribers
        .iter()
        .position(|sub| sub == &rhs)
        .expect("The provided 'rhs' argument could not be found in the list of subscribers");

        self.subscribers.swap_remove(index);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_add_asign_operator_works() {
        let mut some_event = Event::default();

        fn subscribers_fn() {
            println!("some logic");
        }

        some_event += subscribers_fn;

        assert_eq!(some_event.subscribers.len(), 1);
    }

    #[test]
    fn event_sub_asign_operator_works() {
        let mut some_event = Event::default();

        fn subscribers_fn() {
            println!("some logic");
        }

        fn subscribers_fn_to_remove() {
            println!("some logic");
        }

        some_event += subscribers_fn;
        some_event += subscribers_fn_to_remove;

        some_event -= subscribers_fn_to_remove;

        let ptr: fn() = subscribers_fn;
        assert_eq!(some_event.subscribers.len(), 1);
        assert_eq!(some_event.subscribers[0], ptr);
    }
}
