#[cfg(test)]
mod event_tests {
    use std::rc::Rc;
    use rustvent::Event;
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

        logic.process_completed.subscribe(rc_sub.clone());
        logic.on_process_completed();

    }

    #[test]
    fn iterate_over_type_subscribers_manually() {
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
        for sub in logic.process_completed.subscribers.iter() {
            sub.update();
        }
        
    }

}