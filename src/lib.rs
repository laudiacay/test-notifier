#![feature(async_closure)]

use lazy_static::lazy_static;
use nustify::Builder;
use std::env;

use tokio::task;
use tokio::task::spawn_blocking;

lazy_static! {
    static ref KEY: String = env::var("IFTTT_TEST_WEBHOOK_KEY").unwrap();
}

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
        TestNotifier {
            message: Some(message),
        }
    }
}

impl Default for TestNotifier {
    fn default() -> Self {
        Self::new()
    }
}

async fn do_notify(test_name: String, extra_message: Option<String>) {
    println!("test_name: {test_name}");
    println!("message: {:?}", extra_message.clone());
    let extra_message = extra_message.unwrap_or_else(|| "no extra message".to_string());
    let output = format!(
        "test {} is done. also, my caller says: {}",
        test_name,
        extra_message.clone()
    );
    let notification = Builder::new(output.clone())
        .title("Hello from Rust test notifier".to_owned())
        .build();
    nustify::send(&notification, "nustify", &KEY).await.unwrap();
}

impl Drop for TestNotifier {
    fn drop(&mut self) {
        let mut found_my_drop = false; // says "hey i found where i am being dropped" as a bookmark to find which test we were in
        let mut printed_my_message = false; // short-circuits the backtrace after finding which test we were in
        backtrace::trace(|frame| {
            backtrace::resolve_frame(frame, |symbol| {
                if !printed_my_message {
                    let sn = symbol
                        .name()
                        .map_or_else(|| "unknown".to_string(), |sn| format!("{sn:?}"));
                    // println!("symbol: {:?}", sn);
                    if sn.contains("core::ptr::drop_in_place<test_notifier::TestNotifier>") {
                        found_my_drop = true;
                    } else if found_my_drop {
                        let msg = self.message.clone();
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let local = task::LocalSet::new();
                        local.block_on(&rt, async {
                            let join = task::spawn_local(async move {
                                spawn_blocking(move || do_notify(sn, msg))
                                    .await
                                    .unwrap()
                                    .await;
                            });
                            join.await.unwrap();
                        });
                        printed_my_message = true;
                    }
                }
            });
            true
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failing() {
        let mut tn = TestNotifier::new();
        tn.set_message("about to fail".to_string());
        assert!(false);
    }

    #[test]
    fn test_passing() {
        let mut tn = TestNotifier::new();
        tn.set_message("about to pass".to_string());
        assert!(true);
    }
}
