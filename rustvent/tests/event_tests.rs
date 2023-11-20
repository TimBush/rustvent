#[cfg(test)]
mod event_tests {
    use std::rc::Rc;
    use rustvent::events::{Event, EventConfig, Notify, Clear};
    use rustvent::subscriber::Subscriber;
    use rustvent_macros::Event;

    #[derive(Event, Default)]
    struct ProcessBusinessLogic {
        process_completed: Event,
        process_error: Event
    }

    #[derive(Event, Default)]
    struct ProcessLogic {
        id: u8,
        process_completed: Event,
        process_error: Event
    }

    #[test]
    fn type_can_subscribe_to_event() {
        struct BusinessSubscriber {}
        impl Subscriber for BusinessSubscriber {
            fn update(&self) {
                println!("Run some logic...");
            }
        }

        let subscriber = BusinessSubscriber {};
        let mut logic = ProcessBusinessLogic::default();

        let rc_sub = Rc::new(subscriber);

        logic.process_completed += rc_sub.clone();
        logic.on_process_completed();

        assert_eq!(1, logic.process_completed.times_subscribers_notified);
    }

    #[test]
    fn type_can_call_method_on_self_when_notified() {
        #[derive(Event, Default)]
        struct Logic {
            process_completed: Event,
        }

        impl Logic {
            fn notify_subscribers(&mut self) {
                self.on_process_completed();
            }
        }

        struct BusinessSubscriber {}
        impl Subscriber for BusinessSubscriber {
            fn update(&self) {
                println!("Run some logic...");
            }
        }

        let subscriber = BusinessSubscriber {};
        let mut logic = Logic::default();

        let rc_sub = Rc::new(subscriber);

        logic.process_completed += rc_sub.clone();
        logic.notify_subscribers();

        assert_eq!(1, logic.process_completed.times_subscribers_notified);
    }

    #[test]
    fn multiple_calls_to_event_is_not_valid_if_using_event_defaults() {
        struct BusinessSubscriber {}
        impl Subscriber for BusinessSubscriber {
            fn update(&self) {
                println!("Run some logic...");
            }
        }

        let subscriber = BusinessSubscriber {};
        let mut logic = ProcessBusinessLogic::default();

        let rc_sub = Rc::new(subscriber);

        logic.process_completed.subscribe(rc_sub.clone());
        logic.on_process_completed();
        logic.on_process_completed();

        assert_eq!(1, logic.process_completed.times_subscribers_notified);
    }

    #[test]
    fn closure_can_subscribe_to_an_event() {
        let config = EventConfig { 
            subscribers_to_notify: Notify::OnlyFnSubscribers, 
            clear_subscribers_after_notification: Clear::All 
        };

        let mut logic = ProcessBusinessLogic {
            process_completed: Event::new(config),
            process_error: Event::default()
        };

        logic.process_completed.subscribe_as_fn(|| println!("Closure: run some logic..."));
        logic.process_completed.notify();
        
        assert_eq!(1, logic.process_completed.times_func_subscribers_notified);
    }

    #[test]
    fn closure_is_only_notified_one_time() {
        let config = EventConfig { 
            subscribers_to_notify: Notify::OnlyFnSubscribers, 
            clear_subscribers_after_notification: Clear::All 
        };

        let mut logic = ProcessBusinessLogic {
            process_completed: Event::new(config),
            process_error: Event::default()
        };

        logic.process_completed.subscribe_as_fn(|| println!("Closure: run some logic..."));
        logic.process_completed.notify();
        logic.process_completed.notify();
        
        assert_eq!(1, logic.process_completed.times_func_subscribers_notified);
    }

    #[test]
    fn closure_is_notified_twice() {
        let config = EventConfig { 
            subscribers_to_notify: Notify::OnlyFnSubscribers, 
            clear_subscribers_after_notification: Clear::None 
        };

        let mut logic = ProcessBusinessLogic {
            process_completed: Event::new(config),
            process_error: Event::default()
        };

        logic.process_completed.subscribe_as_fn(|| println!("Closure: run some logic..."));
        logic.process_completed.notify();
        logic.process_completed.notify();
        
        assert_eq!(2, logic.process_completed.times_func_subscribers_notified);
    }

}