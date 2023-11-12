//! A crate that implements the observer pattern.

pub mod subscriber;

use std::{ops::{AddAssign, SubAssign}, rc::Rc, ptr};
use subscriber::Subscriber;

#[derive(Default)]
pub struct Event {
    pub subscribers: Vec<Rc<dyn Subscriber>>,
}

impl Event {
    pub fn subscribe(&mut self, subscriber: Rc<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    pub fn unsubscribe(&mut self, subscriber: Rc<dyn Subscriber>) {
        let index = self.subscribers
        .iter()
        .position(|existing_sub| ptr::eq(Rc::as_ptr(existing_sub), Rc::as_ptr(&subscriber)))
        .expect("The provided 'rhs' argument could not be found in the list of subscribers");
        
        self.subscribers.swap_remove(index);
    }
}

impl AddAssign<Rc<dyn Subscriber>> for Event {
    fn add_assign(&mut self, rhs: Rc<dyn Subscriber>) {
        self.subscribers.push(rhs);
    }
}

impl SubAssign<Rc<dyn Subscriber>> for Event {
    fn sub_assign(&mut self, rhs: Rc<dyn Subscriber>) {
        let index = self.subscribers
        .iter()
        .position(|subscriber| ptr::eq(Rc::as_ptr(subscriber), Rc::as_ptr(&rhs)))
        .expect("The provided 'rhs' argument could not be found in the list of subscribers");
        
        self.subscribers.swap_remove(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SomeSubscriber {}

    #[derive(Debug)]
    struct AnotherSubscriber {}
    
    impl Subscriber for SomeSubscriber {
        fn update(&self) {
            println!("SomeSubscriber was notified...");
        }
    }

    impl Subscriber for AnotherSubscriber {
        fn update(&self) {
            println!("AnotherSubscriber was notified...");
        }
    }

    #[test]
    fn event_add_assign_operator_works_on_subscribers() {
        let subscriber = SomeSubscriber {};
        let mut some_event = Event::default();
        let rc_sub = Rc::new(subscriber);

        some_event += rc_sub.clone();

        assert_eq!(some_event.subscribers.len(), 1);
    }

    #[test]
    fn event_sub_assign_operator_works_on_subscribers() {
        let some_subscriber = SomeSubscriber {};
        let another_subscriber = AnotherSubscriber {};

        let mut some_event = Event::default();
        let rc_some_sub: Rc<dyn Subscriber> = Rc::new(some_subscriber);
        let rc_another_sub: Rc<dyn Subscriber> = Rc::new(another_subscriber);

        some_event += rc_some_sub.clone();
        some_event += rc_another_sub.clone();

        some_event -= rc_some_sub;
  
        assert_eq!(some_event.subscribers.len(), 1);
        assert!(Rc::ptr_eq(&some_event.subscribers[0], &rc_another_sub))
    }

    #[test]
    #[should_panic(expected = "argument could not be found in the list of subscribers")]
    fn event_sub_assign_operator_panics_if_type_did_not_subscribe_before_removing() {
        let some_subscriber = SomeSubscriber {};

        let mut some_event = Event::default();
        let rc_some_sub: Rc<dyn Subscriber> = Rc::new(some_subscriber);

        some_event -= rc_some_sub;
    }
}
