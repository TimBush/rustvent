use rustvent::Event;

#[cfg(test)]
mod macro_tests {
    use super::*;
    use rustvent_macros::Event;


    #[derive(Event)]
    struct ProcessBusinessLogic {
        process_completed: Event,
        process_error: Event
    }

    #[derive(Event)]
    struct ProcessLogic {
        id: u8,
        process_completed: Event,
        process_error: Event
    }


    #[test]
    fn event_macro_two_events_generates_two_methods() {
        let mut logic = ProcessBusinessLogic {
            process_completed: Event { subscribers: Vec::new() },
            process_error: Event { subscribers: Vec::new() }

        };

        fn first_subscriber() {
            println!("first_subscriber notified");
        }

        fn second_subscriber() {
            println!("second_subscriber notified");
        }

        logic.process_completed.subscribe(first_subscriber);
        logic.process_error.subscribe(second_subscriber);
        logic.on_process_completed();
        logic.on_process_error();
    }

    #[test]
    fn event_macro_additional_struct_fields_do_not_have_methods_generated() {
        let mut logic = ProcessLogic {
            id: 1,
            process_completed: Event { subscribers: Vec::new() },
            process_error: Event { subscribers: Vec::new() }
        };

        
    }
}