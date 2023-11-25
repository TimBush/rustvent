//! A crate that implements the observer pattern.

pub mod subscriber;
pub mod event_async;
pub mod macros;
use std::{rc::Rc, ptr};
use subscriber::{Subscriber, SubscriberMut};
use std::ops::{AddAssign, SubAssign};
use std::cell::RefCell;

#[macro_use]
pub mod events {
    use super::*;

    /// Provides a **synchronous** mechanism for [Subscribers](Subscriber) to
    /// register themselves with a source, in this case an [Event]. Events can then
    /// notify subscribers of state changes.
    #[derive(Default)]
    pub struct Event {
        pub times_subscribers_notified: u32,
        pub times_subscribers_mut_notified: u32,
        pub times_func_subscribers_notified: u32,
        subscribers: Vec<Rc<dyn Subscriber>>,
        subscribers_mut: Vec<Rc<RefCell<dyn SubscriberMut>>>,
        fn_subscribers: Vec<Box<dyn Fn() -> ()>>,
        config: EventConfig,
    }

    /// Provides values to configure individual [Events](Event). 
    pub struct EventConfig {
        pub subscribers_to_notify: Notify,
        pub clear_subscribers_after_notification: Clear
    }

    /// When used in conjunction with [EventConfig], this allows for configuring
    /// which subscribers to an event are notified.
    pub enum Notify {
        /// Implementors of the [Subscriber] Trait and closures. 
        All,
        /// Only implementors of the [Subscriber] Trait.
        OnlySubscribers,
        /// Only implementors of the [SubscriberMut] Trait.
        OnlySubscribersMut,
        /// Only closures.
        OnlyFnSubscribers
    }

    pub enum Clear {
        All,
        OnlySubscribers,
        OnlySubscribersMut,
        OnlyFuncSubscribers,
        None
    }

    impl Event {
        /// Creates a new [Event] with the provided [EventConfig]
        /// being used to determine the default behavior of this particular Event. 
        pub fn new(config: EventConfig) -> Event {
            Event { 
                subscribers: Default::default(), 
                subscribers_mut: Default::default(),
                fn_subscribers: Default::default(), 
                times_subscribers_notified: Default::default(), 
                times_subscribers_mut_notified: Default::default(), 
                times_func_subscribers_notified: Default::default(), 
                config, 
            }
        }

        /// Allows any implementors of the [Subscriber] Trait to subscribe
        /// to this event.  Alternatively you may use the `AddAssign` operator (`+=`) to subscribe to an event.
        /// Need to subscribe to an event with a closure? See [subscribe_as_fn()](Event::subscribe_as_fn()).
        /// # Examples
        /// ```
        /// struct NewSubscriber {}
        /// impl Subscriber for NewSubscriber {
        ///     fn update(&self) {
        ///         println!("NewSubscriber notified...");
        ///     }
        /// }
        /// 
        /// let mut some_event = Event::default();
        /// let rc_new_sub = Rc::new(NewSubscriber {});
        /// some_event.subscribe(rc_new_sub.clone());
        /// // some_event += rc_new_sub.clone() - Equivalent to the line above. 
        /// ```
        pub fn subscribe(&mut self, subscriber: Rc<dyn Subscriber>) {
            self.subscribers.push(subscriber);
        }

        pub fn subscribe_mut(&mut self, subscriber: Rc<RefCell<dyn SubscriberMut>>) {
            self.subscribers_mut.push(subscriber);
        }

        /// Subscribe to an event with a closure.
        /// Need to subscribe to an event with a Struct? See [subscribe()](Event::subscribe()).
        /// # Examples
        /// ```
        /// let mut some_event = Event::default();
        /// some_event.subscribe_as_fn(|| println!("Closure notified..."));
        /// ```
        pub fn subscribe_as_fn<F>(&mut self, func: F) where F: Fn() -> () + 'static {
            let box_func = Box::new(func);
            self.fn_subscribers.push(box_func);
        }

        /// Unsubscribe a [Subscriber] from this event.
        /// Alternatively you may use the `SubAssign` operator (`-=`) to unsubscribe to an event.
        pub fn unsubscribe(&mut self, subscriber: Rc<dyn Subscriber>) {
            let index = self.contains(subscriber)
            .expect("The provided 'subscriber' argument could not be found in the list of subscribers.");
            
            self.subscribers.swap_remove(index);
        }

        pub fn unsubscribe_mut(&mut self, subscriber: Rc<RefCell<dyn SubscriberMut>>) {
            let index = self.contains_mut(subscriber)
            .expect("The provided 'subscriber' argument could not be found in the list of subscribers.");
            
            self.subscribers_mut.swap_remove(index);
        }

        /// Get all [Subscribers](Subscriber) listening to this event.
        pub fn get_subscribers(&self) -> &Vec<Rc<dyn Subscriber>> {
            &self.subscribers
        }

        /// Get all [Mutable Subscribers](SubscriberMut) listening to this event.
        pub fn get_subscribers_mut(&self) -> &Vec<Rc<RefCell<dyn SubscriberMut>>> {
            &self.subscribers_mut
        }

        /// Get all closures listening to this event.
        pub fn get_fn_subscribers(&self) -> &Vec<Box<dyn Fn() -> ()>> {
            &self.fn_subscribers
        }

        /// Notifies subscribers.  Which subscribers are notified is determined by the configuration values
        /// defined by the [EventConfig] of this event.  If using the derive macro [rustvent_macros::Event],
        /// you may want to use the methods that are auto-generated by the macro, instead of this method.
        /// 
        /// # Examples
        /// ```
        /// #[derive(Event, Default)]
        /// struct ProcessBusinessLogic {
        ///     process_completed: Event,
        /// }
        /// 
        /// impl ProcessBusinessLogic {
        ///     fn doing_a_bunch_of_processing(&self) {
        ///         // processing logic...
        ///         self.process_completed.notify();
        /// 
        ///         // Auto-generated method by the rustvent_macros::Event macro.
        ///         // This is equivalent to notify().
        ///         self.on_process_completed(); 
        ///     }
        /// }
        /// 
        /// struct NewSubscriber {}
        /// impl Subscriber for NewSubscriber {
        ///     fn update(&self) {
        ///         println!("NewSubscriber notified...");
        ///     }
        /// }
        /// 
        /// let mut logic = ProcessBusinessLogic::default();
        /// let rc_new_sub = Rc::new(NewSubscriber {});
        /// 
        /// // NewSubscriber has now subscribed to the process_completed `Event`.
        /// logic.process_completed += rc_new_sub.clone(); 
        /// ```
        pub fn notify(&mut self) {
            match self.config.subscribers_to_notify {
                Notify::All => {
                    self.notify_subscribers();
                    self.notify_subscribers_mut();
                    self.notify_fn_subscribers();
                },
                Notify::OnlySubscribers => self.notify_subscribers(),
                Notify::OnlySubscribersMut => self.notify_subscribers_mut(),
                Notify::OnlyFnSubscribers => self.notify_fn_subscribers(),
            }

            self.try_clear();
        }

        fn contains(&self, subscriber: Rc<dyn Subscriber>) -> Option<usize> {
            if let Some(i) = self.subscribers
            .iter()
            .position(|existing_sub| Rc::ptr_eq(existing_sub, &subscriber))
            {
                Some(i)
            } else {
                None
            }
        }

        fn contains_mut(&self, subscriber: Rc<RefCell<dyn SubscriberMut>>) -> Option<usize> {
            if let Some(i) = self.subscribers_mut
            .iter()
            .position(|existing_sub| Rc::ptr_eq(existing_sub, &subscriber))
            {
                Some(i)
            } else {
                None
            }
        }

        fn notify_subscribers(&mut self) {
            if self.subscribers.is_empty() { return; }

            for sub in self.subscribers.iter() {
                sub.update();
            }
            self.times_subscribers_notified += 1;
        }

        fn notify_subscribers_mut(&mut self) {
            if self.subscribers_mut.is_empty() { return; }

            for sub in self.subscribers_mut.iter_mut() {
                // let sub_mut = Rc::get_mut(sub).expect("The current subscriber is shared and cannot be safely mutated.");
                sub.borrow_mut().update_mut();
            }
            self.times_subscribers_mut_notified += 1;
        }

        fn notify_fn_subscribers(&mut self) {
            if self.fn_subscribers.is_empty() { return; }

            for func in self.fn_subscribers.iter() {
                func();
            }
            self.times_func_subscribers_notified += 1;
        }

        fn try_clear(&mut self) {
            match self.config.clear_subscribers_after_notification {
                Clear::All => self.clear_all_subscribers(),
                Clear::OnlySubscribers => self.clear_subscribers(),
                Clear::OnlySubscribersMut => self.clear_subscribers_mut(),
                Clear::OnlyFuncSubscribers => self.clear_fn_subscribers(),
                Clear::None => return,
            }
        }
        
        fn clear_all_subscribers(&mut self) {
            self.clear_subscribers();
            self.clear_fn_subscribers();
        }
    
        fn clear_subscribers(&mut self) {
            self.subscribers.clear();
        }

        fn clear_subscribers_mut(&mut self) {
            self.subscribers_mut.clear();
        }
        
        fn clear_fn_subscribers(&mut self) {
            self.fn_subscribers.clear();
        }
    }

    impl Default for EventConfig {
        fn default() -> Self {
            Self { 
                subscribers_to_notify: Notify::All, 
                clear_subscribers_after_notification: Clear::All 
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{events::Event, subscriber::SubscriberMut};

    struct SomeSubscriber {}

    #[derive(Debug)]
    struct AnotherSubscriber {}

    struct MutSubscriber {
        mutate_field_int: u8
    }

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

    impl SubscriberMut for MutSubscriber {
        fn update_mut(&mut self) {
            self.mutate_field_int += 10;
        }
    }

    #[test]
    fn event_subscriber_can_subscribe() {
        let subscriber = SomeSubscriber {};
        let mut event = Event::default();
        let sub = into_subscriber!(subscriber);

        event.subscribe(sub.clone());

        assert_eq!(1, event.get_subscribers().len());
    }

    #[test]
    fn event_subscriber_mut_can_subscribe() {
        let subscriber = MutSubscriber { mutate_field_int: 10 };
        let mut event = Event::default();
        let rc_sub = into_mut_subscriber!(subscriber);
        event.subscribe_mut(rc_sub.clone());

        assert_eq!(1, event.get_subscribers_mut().len());
    }

    #[test]
    fn event_subscriber_can_unsubscribe() {
        let some_subscriber = SomeSubscriber {};
        let another_subscriber = AnotherSubscriber {};

        let mut event = Event::default();
        let rc_some_sub: Rc<dyn Subscriber> = into_subscriber!(some_subscriber);
        let rc_another_sub: Rc<dyn Subscriber> = into_subscriber!(another_subscriber);

        event.subscribe(rc_some_sub.clone());
        event.subscribe(rc_another_sub.clone());

        event.unsubscribe(rc_some_sub.clone());
  
        assert_eq!(event.get_subscribers().len(), 1);
        assert!(Rc::ptr_eq(&event.get_subscribers()[0], &rc_another_sub))
    }

    #[test]
    fn event_subscriber_mut_can_unsubscribe() {
        let subscriber = MutSubscriber { mutate_field_int: 10 };

        let mut event = Event::default();
        let rc_sub = into_mut_subscriber!(subscriber);

        event.subscribe_mut(rc_sub.clone());
        event.unsubscribe_mut(rc_sub.clone());
  
        assert!(event.get_subscribers_mut().is_empty());
    }

    #[test]
    fn event_subscriber_is_notified() {
        let subscriber = SomeSubscriber {};
        let mut event = Event::default();
        let rc_sub = into_subscriber!(subscriber);

        event.subscribe(rc_sub.clone());
        event.notify();

        assert_eq!(1, event.times_subscribers_notified);
    }

    #[test]
    fn event_subscriber_mut_is_notified() {
        let subscriber = MutSubscriber { mutate_field_int: 10 };
        let mut event = Event::default();
        let rc_sub = into_mut_subscriber!(subscriber);

        event.subscribe_mut(rc_sub.clone());
        event.notify();

        assert_eq!(1, event.times_subscribers_mut_notified);
        assert_eq!(20, rc_sub.borrow().mutate_field_int);
    }

    #[test]
    fn event_subscriber_mut_multiple_immutable_borrows_of_subscribers_is_valid() {
        let subscriber = MutSubscriber { mutate_field_int: 10 };

        let rc_sub = into_mut_subscriber!(subscriber);
        let rc_sub_borrow = rc_sub.borrow();

        assert_eq!(10, rc_sub.borrow().mutate_field_int);
        assert_eq!(10, rc_sub_borrow.mutate_field_int)
    }

    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn event_subscriber_mut_immutable_borrow_and_borrow_on_single_subscriber_panics() {
        let subscriber = MutSubscriber { mutate_field_int: 10 };
        let sub = into_mut_subscriber!(subscriber);
        let mut sub_borrow = sub.borrow_mut();

        sub_borrow.mutate_field_int += 10;

        assert_eq!(20, sub.borrow().mutate_field_int);
    }

    #[test]
    #[should_panic(expected = "argument could not be found in the list of subscribers")]
    fn event_panics_if_type_did_not_subscribe_before_removing() {
        let mut some_event = Event::default();
        let sub = into_subscriber!(SomeSubscriber {});

        some_event.unsubscribe(sub.clone());
    }

    #[test]
    #[should_panic(expected = "argument could not be found in the list of subscribers")]
    fn event_panics_if_type_did_not_subscribe_mut_before_removing() {
        let subscriber = MutSubscriber { mutate_field_int: 10 };
        let mut event = Event::default();
        let rc_sub = into_mut_subscriber!(subscriber);

        event.unsubscribe_mut(rc_sub.clone());
    }

    #[test]
    fn event_subscribe_as_closure_works() {
        let mut some_event = Event::default();
        some_event.subscribe_as_fn(|| println!("Closure: run some logic..."));

        assert_eq!(some_event.get_fn_subscribers().len(), 1);
    }

    #[test]
    fn event_notify_fn_subscribers_works() {
        let mut some_event = Event::default();
        some_event.subscribe_as_fn(|| println!("Closure: run some logic..."));
        some_event.notify();
    }
}
