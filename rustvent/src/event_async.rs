use std::{sync::Arc, thread::{self, JoinHandle}, ops::{AddAssign, SubAssign}};

use crate::{subscriber::SubscriberAsync, events::{EventConfig, Notify, Clear}};

macro_rules! default {
    () => {
        Default::default()
    };
}

#[derive(Default)]
pub struct EventAsync {
    pub times_subscribers_notified: u32,
    pub times_func_subscribers_notified: u32,
    subscribers: Vec<Arc<(dyn SubscriberAsync + Send + Sync)>>,
    fn_subscribers: Vec<Arc<dyn Fn() -> () + Send + Sync>>,
    config: EventConfig
}

impl EventAsync {
    pub fn new(config: EventConfig) -> EventAsync {
        EventAsync 
        { 
            subscribers: default!(), 
            fn_subscribers: default!(),
            times_subscribers_notified: default!(), 
            times_func_subscribers_notified: default!(),
            config
        }
    }

    pub fn get_subscribers(&self) -> &Vec<Arc<(dyn SubscriberAsync + Send + Sync)>> {
        &self.subscribers
    }

    pub fn get_fn_subscribers(&self) -> &Vec<Arc<dyn Fn() -> () + Send + Sync>> {
        &self.fn_subscribers
    }

    pub fn subscribe(&mut self, subscriber: Arc<(dyn SubscriberAsync + Send + Sync)>) {
        self.subscribers.push(subscriber);
    }

    pub fn subscribe_as_fn<F>(&mut self, subscriber: F) where F: Fn() -> () + Send + Sync + 'static {
        self.fn_subscribers.push(Arc::new(subscriber));
    }

    pub fn unsubscribe(&mut self, rhs: Arc<(dyn SubscriberAsync + Send + Sync)>) {
        let index = self.subscribers
        .iter()
        .position(|sub| Arc::ptr_eq(&rhs, sub))
        .expect("The provided 'rhs' argument could not be found in the list of subscribers.");
        
        self.subscribers.swap_remove(index);
    }

    pub fn notify(&mut self) {
        match self.config.subscribers_to_notify {
            Notify::All => {
                self.notify_subscribers();
                self.notify_fn_subscribers();
            },
            Notify::OnlySubscribers => self.notify_subscribers(),
            Notify::OnlyFnSubscribers => self.notify_fn_subscribers(),
        }

        self.try_clear();
    }

    pub fn notify_subscribers(&mut self) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for sub in self.subscribers.iter() {
            let sub_clone = sub.clone();

            let handle = thread::spawn(move || {
                sub_clone.update();
            });

            handles.push(handle);
        }

        self.times_subscribers_notified += 1;

        handles.into_iter().for_each(|h| h.join().unwrap())
    }

    pub fn notify_fn_subscribers(&mut self) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for closure in self.fn_subscribers.iter() {
            let closure_clone = closure.clone();

            let handle = thread::spawn(move || {
                closure_clone();
            });

            handles.push(handle);
        }

        self.times_func_subscribers_notified += 1;

        handles.into_iter().for_each(|h| h.join().unwrap())
    }

    fn try_clear(&mut self) {
        match self.config.clear_subscribers_after_notification {
            Clear::All => self.clear_all_subscribers(),
            Clear::OnlySubscribers => self.clear_subscribers(),
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
    
    fn clear_fn_subscribers(&mut self) {
        self.fn_subscribers.clear();
    }
}

impl AddAssign<Arc<(dyn SubscriberAsync + Send + Sync)>> for EventAsync {
    fn add_assign(&mut self, rhs: Arc<(dyn SubscriberAsync + Send + Sync)>) {
        self.subscribe(rhs);
    }
}
    
impl SubAssign<Arc<(dyn SubscriberAsync + Send + Sync)>> for EventAsync {
    fn sub_assign(&mut self, rhs: Arc<(dyn SubscriberAsync + Send + Sync)>) {
        self.unsubscribe(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subscriber::SubscriberAsync;

    #[derive(Default)]
    struct TestSubscriber {}

    impl SubscriberAsync for TestSubscriber {
        fn update(&self) {
            println!("SomeSubscriber notified...");
        }
    }

    #[test]
    fn event_async_subscribe_is_successful() {
        let mut event = EventAsync::default();

        let test_sub = TestSubscriber::default().into_arc();
        event.subscribe(test_sub.clone());

        assert_eq!(1, event.get_subscribers().len()); 
    }

    #[test]
    fn event_async_add_assign_adds_one_subscriber() {
        let mut event = EventAsync::default();

        let test_sub = TestSubscriber::default().into_arc();
        event += test_sub;

        assert_eq!(1, event.get_subscribers().len());
    }

    #[test]
    fn event_async_sub_assign_removes_one_subscriber() {
        let mut event = EventAsync::default();

        let test_sub = TestSubscriber::default().into_arc();

        event += test_sub.clone();
        event -= test_sub.clone();

        assert!(event.get_subscribers().is_empty());
    }

    #[test]
    fn event_async_clear_subscribers() {
        let mut event = EventAsync::default();

        let test_sub = TestSubscriber::default().into_arc();

        event += test_sub.clone();
        event.clear_subscribers();

        assert!(event.get_subscribers().is_empty());
    }

    #[test]
    fn event_async_clear_fn_subscribers() {
        let mut event = EventAsync::default();

        event.subscribe_as_fn(|| println!("Closure notified..."));
        event.clear_fn_subscribers();

        assert!(event.get_fn_subscribers().is_empty());
    }

    #[test]
    fn event_async_clear_all_subscribers() {
        let mut event = EventAsync::default();

        let test_sub = TestSubscriber::default().into_arc();

        event.subscribe_as_fn(|| println!("Closure notified..."));
        event.subscribe(test_sub);
        event.clear_all_subscribers();

        assert!(event.get_subscribers().is_empty());
        assert!(event.get_fn_subscribers().is_empty());
    }

}