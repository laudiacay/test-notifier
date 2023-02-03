use backtrace;

pub struct TestNotifier {
    message: Option<String>,
}

impl TestNotifier {
    pub fn new() -> TestNotifier {
        TestNotifier { message: None }
    }
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }
    pub fn new_with_message(message: String) -> TestNotifier {
        TestNotifier { message: Some(message) }
    }
}

impl Drop for TestNotifier {
    fn drop(&mut self) {
        let mut found_my_drop = false; // says "hey i found where i am being dropped" as a bookmark to find which test we were in
        let mut printed_my_message = false; // short-circuits the backtrace after finding which test we were in
        backtrace::trace(|frame| {
            backtrace::resolve_frame(frame, |symbol| {
                if !printed_my_message {
                    let sn = symbol.name().map_or_else(
                        || { format!("unknown")}, |sn| format!("{:?}", sn));
                    // println!("symbol: {:?}", sn);
                    if sn.contains("core::ptr::drop_in_place<test_notifier::TestNotifier>") {
                        found_my_drop = true;
                    } else if found_my_drop {
                        println!("test {:?} is done", sn);
                        if let Some(msg) = self.message.as_ref() {
                            println!("also... my caller wanted me to tell you: {}", msg);
                        }
                        printed_my_message = true;
                    }
                }
            });
            true
        });
    }
}

#[test]
fn test() {
    let mut tn = TestNotifier::new();
    tn.set_message("hello".to_string());
    assert!(false);
}
