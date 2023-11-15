//! A crate that implements the observer pattern.

pub mod subscriber;
use std::{ops::{AddAssign, SubAssign}, rc::Rc, ptr};
use subscriber::Subscriber;

pub mod events {
    use super::*;

    #[derive(Default)]
    pub struct Event {
        pub times_subscribers_notified: u32,
        pub times_func_subscribers_notified: u32,
        subscribers: Vec<Rc<dyn Subscriber>>,
        func_subscribers: Vec<Box<dyn Fn() -> ()>>,
        config: EventConfig,
    }

    pub struct EventConfig {
        pub subscribers_to_notify: Notify,
        pub delete_subscribers_after_notification: bool
    }

    pub enum Notify {
        All,
        OnlySubscribers,
        OnlyFuncSubscribers
    }

    impl Event {
        pub fn new(config: EventConfig) -> Event {
            Event { 
                subscribers: Default::default(), 
                func_subscribers: Default::default(), 
                times_subscribers_notified: Default::default(), 
                times_func_subscribers_notified: Default::default(), 
                config 
            }
        }

        pub fn subscribe(&mut self, subscriber: Rc<dyn Subscriber>) {
            self.subscribers.push(subscriber);
        }

        pub fn subscribe_as_fn<F>(&mut self, func: F) where F: Fn() -> () + 'static {
            let box_func = Box::new(func);
            self.func_subscribers.push(box_func);
        }

        pub fn unsubscribe(&mut self, subscriber: &Rc<dyn Subscriber>) {
            let index = self.subscribers
            .iter()
            .position(|existing_sub| ptr::eq(Rc::as_ptr(existing_sub), Rc::as_ptr(&subscriber)))
            .expect("The provided 'rhs' argument could not be found in the list of subscribers.");
            
            self.subscribers.swap_remove(index);
        }

        pub fn get_subscribers(&self) -> &Vec<Rc<dyn Subscriber>> {
            &self.subscribers
        }

        pub fn get_func_subscribers(&self) -> &Vec<Box<dyn Fn() -> ()>> {
            &self.func_subscribers
        }

        pub fn notify(&mut self) {
            match self.config.subscribers_to_notify {
                Notify::All => {
                    self.notify_subscribers();
                    self.notify_fn_subscribers();
                },
                Notify::OnlySubscribers => self.notify_subscribers(),
                Notify::OnlyFuncSubscribers => self.notify_fn_subscribers(),
            }

            if self.config.delete_subscribers_after_notification {
                self.delete_all_subscribers();
            }
        }

        fn notify_subscribers(&mut self) {
            if self.subscribers.is_empty() { return; }

            for sub in self.subscribers.iter() {
                sub.update();
            }
            self.times_subscribers_notified += 1;
        }

        fn notify_fn_subscribers(&mut self) {
            if self.func_subscribers.is_empty() { return; }

            for func in self.func_subscribers.iter() {
                func();
            }
            self.times_func_subscribers_notified += 1;
        }
        
        fn delete_all_subscribers(&mut self) {
            self.subscribers.clear();
            self.func_subscribers.clear();
        }
    }

    impl AddAssign<Rc<dyn Subscriber>> for Event {
        fn add_assign(&mut self, rhs: Rc<dyn Subscriber>) {
            self.subscribers.push(rhs);
        }
    }
    
    impl SubAssign<&Rc<dyn Subscriber>> for Event {
        fn sub_assign(&mut self, rhs: &Rc<dyn Subscriber>) {
            let index = self.subscribers
            .iter()
            .position(|subscriber| ptr::eq(Rc::as_ptr(subscriber), Rc::as_ptr(rhs)))
            .expect("The provided 'rhs' argument could not be found in the list of subscribers.");
            
            self.subscribers.swap_remove(index);
        }
    }

    impl Default for EventConfig {
        fn default() -> Self {
            Self { 
                subscribers_to_notify: Notify::All, 
                delete_subscribers_after_notification: true 
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use std::rc;

    use super::*;
    use crate::events::Event;

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

        assert_eq!(some_event.get_subscribers().len(), 1);
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

        some_event -= &rc_some_sub;
  
        assert_eq!(some_event.get_subscribers().len(), 1);
        assert!(Rc::ptr_eq(&some_event.get_subscribers()[0], &rc_another_sub))
    }

    #[test]
    fn event_unsubscribe_is_successful() {
        let mut some_event = Event::default();
        let rc_some_sub: Rc<dyn Subscriber> = Rc::new(SomeSubscriber {});

        some_event += rc_some_sub.clone();
        some_event.unsubscribe(&rc_some_sub);

        assert!(some_event.get_subscribers().is_empty());
    }

    #[test]
    #[should_panic(expected = "argument could not be found in the list of subscribers")]
    fn event_sub_assign_operator_panics_if_type_did_not_subscribe_before_removing() {
        let mut some_event = Event::default();
        let rc_some_sub: Rc<dyn Subscriber> = Rc::new(SomeSubscriber {});

        some_event -= &rc_some_sub;
    }

    #[test]
    fn event_subscribe_as_closure_works() {
        let mut some_event = Event::default();
        some_event.subscribe_as_fn(|| println!("Closure: run some logic..."));

        assert_eq!(some_event.get_func_subscribers().len(), 1);
    }

    #[test]
    fn event_notify_func_subscribers_works() {
        let mut some_event = Event::default();
        some_event.subscribe_as_fn(|| println!("Closure: run some logic..."));
        some_event.notify();
    }
}
