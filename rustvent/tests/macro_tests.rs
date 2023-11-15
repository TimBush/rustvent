#[cfg(test)]
mod macro_tests {
    use std::rc::Rc;
    use rustvent::subscriber::Subscriber;
    use rustvent_macros::Event;
    use rustvent::events::Event;

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

    struct BusinessSubscriber {}
    impl Subscriber for BusinessSubscriber {
        fn update(&self) {
            println!("Run some logic...");
        }
    }

    #[test]
    fn event_macro_two_events_generates_two_methods() {
        let mut logic = ProcessBusinessLogic::default();
        let first_sub = Rc::new(BusinessSubscriber {});
        let second_sub = Rc::new(BusinessSubscriber {});

        logic.process_completed.subscribe(first_sub.clone());
        logic.process_error.subscribe(second_sub.clone());
        logic.on_process_completed();
        logic.on_process_error();

        assert_eq!(1, logic.process_completed.times_subscribers_notified);
        assert_eq!(1, logic.process_error.times_subscribers_notified);
    }

    #[test]
    fn event_macro_additional_struct_fields_do_not_have_methods_generated() {
        let mut logic = ProcessLogic {
            id: 1,
            process_completed: Event::default(),
            process_error: Event::default()
        };        
    }

    #[test]
    fn event_macro_can_call_on_x_methods_internally() {
        #[derive(Event, Default)]
        struct SomeLogic {
            process_completed: Event
        }

        struct SomeSubscriber {}

        impl SomeLogic {
            fn notify_process_completed_subscribers(&mut self) {
                self.on_process_completed();
            }
        }

        impl Subscriber for SomeSubscriber {
            fn update(&self) {
                println!("Subscriber notified of event");
            }
        }

        let mut logic = SomeLogic::default();
        let subscriber = Rc::new(SomeSubscriber {});
        logic.process_completed.subscribe(subscriber.clone());
        logic.notify_process_completed_subscribers();

        assert_eq!(1, logic.process_completed.times_subscribers_notified);
    }

}