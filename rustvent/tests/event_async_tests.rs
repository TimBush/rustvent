#[cfg(test)]
mod event_async_tests {
    use rustvent::{event_async::EventAsync, subscriber::SubscriberAsync};

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
    fn event_async_func_subscribers_are_notified() {
        let mut logic = ProcessBusinessLogic {
            process_completed: EventAsync::default()
        };

        logic.process_completed.subscribe_as_fn(|| println!("First closure notified..."));        
        logic.process_completed.subscribe_as_fn(|| println!("Second closure notified..."));        
        logic.process_completed.notify_fn_subscribers();  

        assert_eq!(1, logic.process_completed.times_func_subscribers_notified); 
    }

}