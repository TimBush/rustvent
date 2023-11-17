use std::{sync::Arc, thread::{self, JoinHandle}, ops::{AddAssign, SubAssign}};

use crate::subscriber::SubscriberAsync;

#[derive(Default)]
pub struct EventAsync {
    pub times_subscribers_notified: u32,
    pub times_func_subscribers_notified: u32,
    subscribers: Vec<Arc<(dyn SubscriberAsync + Send + Sync)>>,
}

impl EventAsync {
    pub fn new() -> EventAsync {
        EventAsync 
        { 
            subscribers: Default::default(), 
            times_subscribers_notified: Default::default(), 
            times_func_subscribers_notified: Default::default() 
        }
    }

    pub fn get_subscribers(&self) -> &Vec<Arc<(dyn SubscriberAsync + Send + Sync)>> {
        &self.subscribers
    }

    pub fn subscribe(&mut self, subscriber: Arc<(dyn SubscriberAsync + Send + Sync)>) {
        self.subscribers.push(subscriber);
    }

    pub fn unsubscribe(&mut self, rhs: Arc<(dyn SubscriberAsync + Send + Sync)>) {
        let index = self.subscribers
        .iter()
        .position(|sub| Arc::ptr_eq(&rhs, sub))
        .expect("The provided 'rhs' argument could not be found in the list of subscribers.");
        
        self.subscribers.swap_remove(index);
    }

    pub fn notify(&mut self) {
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

    #[test]
    fn event_async_subscribe_is_successful() {
        let mut event = EventAsync::default();

        struct SomeSubscriber {}
        impl SubscriberAsync for SomeSubscriber {
            fn update(&self) {
                println!("Just testing");
            }
        }

        let some_sub = SomeSubscriber {}.into_arc();
        event.subscribe(some_sub.clone());

        assert_eq!(1, event.get_subscribers().len()); 
    }

    #[test]
    fn event_async_add_assign_adds_one_subscriber() {
        let mut event = EventAsync::default();

        struct SomeSubscriber {}
        impl SubscriberAsync for SomeSubscriber {
            fn update(&self) {
                println!("SomeSubscriber notified...");
            }
        }

        let some_sub = SomeSubscriber {}.into_arc();

        event += some_sub;

        assert_eq!(1, event.get_subscribers().len());
    }

    #[test]
    fn event_async_sub_assign_removes_one_subscriber() {
        let mut event = EventAsync::default();

        struct SomeSubscriber {}
        impl SubscriberAsync for SomeSubscriber {
            fn update(&self) {
                println!("SomeSubscriber notified...");
            }
        }

        let some_sub = SomeSubscriber {}.into_arc();

        event += some_sub.clone();
        event -= some_sub.clone();

        assert!(event.get_subscribers().is_empty());
    }
}