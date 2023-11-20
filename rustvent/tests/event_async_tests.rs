#[cfg(test)]
mod event_async_tests {
    use std::sync::{Arc, Mutex};

    use rustvent::{event_async::EventAsync, subscriber::{SubscriberAsync, SubscriberAsyncMut}};

    struct ProcessBusinessLogic {
        process_completed: EventAsync
    }
    
    impl ProcessBusinessLogic {
        fn on_process_completed(&mut self) {
            self.process_completed.notify_subscribers();
        }
    }

    #[test]
    fn event_async_subscriber_is_notified() {
        let mut logic = ProcessBusinessLogic {
            process_completed: EventAsync::default()
        };

        struct SomeSubscriber {}
        impl SubscriberAsync for SomeSubscriber {
            fn update(&self) {
                println!("SomeSubscriber notified...");
            }
        }

        let some_sub = SomeSubscriber {}.into_arc();
        logic.process_completed.subscribe(some_sub);        
        logic.on_process_completed();  

        assert_eq!(1, logic.process_completed.times_subscribers_notified); 
    }

    #[test]
    fn event_async_multiple_subscribers_are_notified() {
        let mut logic = ProcessBusinessLogic {
            process_completed: EventAsync::default()
        };

        struct FirstSubscriber {}
        impl SubscriberAsync for FirstSubscriber {
            fn update(&self) {
                println!("FirstSubscriber notified...");
            }
        }

        struct SecondSubscriber {}
        impl SubscriberAsync for SecondSubscriber {
            fn update(&self) {
                println!("SecondSubscriber notified...");
            }
        }

        let first_sub = FirstSubscriber {}.into_arc();
        let second_sub = SecondSubscriber {}.into_arc();
        logic.process_completed.subscribe(first_sub);        
        logic.process_completed.subscribe(second_sub);        
        logic.on_process_completed();  

        assert_eq!(1, logic.process_completed.times_subscribers_notified); 
    }

    #[test]
    fn event_async_subscribers_can_call_method_on_self_when_notified() {
        let mut logic = ProcessBusinessLogic {
            process_completed: EventAsync::default()
        };

        struct SomeSubscriber {}

        impl SomeSubscriber {
            fn do_work(&self) {
                println!("SomeSubscriber doing work...");
            }
        }
        impl SubscriberAsync for SomeSubscriber {
            fn update(&self) {
                self.do_work();
            }
        }

        let some_sub = SomeSubscriber {}.into_arc();
        logic.process_completed.subscribe(some_sub);               
        logic.on_process_completed();  

        assert_eq!(1, logic.process_completed.times_subscribers_notified); 
    }

    #[test]
    fn event_async_subscribers_can_mutate_methods_on_self_when_notified() {
        pub struct Logic {
            process_completed: EventAsync
        }
        impl Logic {
            fn on_process_completed(&mut self) {
                self.process_completed.notify_subscribers_mut();
            }
        }

        struct SomeSubscriber {
            pub field_to_mutate: u8
        }

        impl SubscriberAsyncMut for SomeSubscriber {
            fn update_mut(&mut self) {
                self.field_to_mutate += 10
            }
        }

        let mut logic = Logic {
            process_completed: EventAsync::default()
        };

        let some_sub = Arc::new(Mutex::new(SomeSubscriber { field_to_mutate: 10 }));
        logic.process_completed.subscribe_mut(some_sub.clone());               
        logic.on_process_completed();  
    
        assert_eq!(20, some_sub.clone().lock().unwrap().field_to_mutate); 
    }

    #[test]
    fn event_async_fn_subscribers_are_notified() {
        let mut logic = ProcessBusinessLogic {
            process_completed: EventAsync::default()
        };

        logic.process_completed.subscribe_as_fn(|| println!("First closure notified..."));        
        logic.process_completed.subscribe_as_fn(|| println!("Second closure notified..."));        
        logic.process_completed.notify_fn_subscribers();  

        assert_eq!(1, logic.process_completed.times_func_subscribers_notified); 
    }

}