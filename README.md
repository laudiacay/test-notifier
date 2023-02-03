# test-notifier

I had some really long-running tests for my work at Banyan, and I wanted a descriptive notification on my phone whenever they completed.
I also did not want to have to add stupid print statements signifying which test we were in.
This exposes a struct, `TestNotifier`, that you simply initialize at the start of your test.
It will send a notification to your phone when the test completes, and it will also send a notification if the test panics.
Here is how you use it:
```rust
#[test]
fn test_yay_yahoo() {
    // create a test notifier
    let mut tn = TestNotifier::new();
    // set a message (you can do this multiple times... eventually i'll add something that can append to the message)
    // this helps you smuggle more data out to your notification
    tn.set_message("hello".to_string());
    // fail the test (or pass the test. testnotifier doesn't care)
    assert!(false);
    // around here, you'll get an IFTTT notification or whatever you configured.
    // it'll contain "test_yay_yahoo" and the cratepath that you're testing and "hello"
    // maybe you could make your bedroom lights change color depending on whether it passed.
    // have alexa read you the message, including the function's address in memory
    // doing this will deeply impress the women in your life and not scare them. you should explain vtables to her as well, in great detail. srs.
}
```

## how it work?
when the TestNotifier is dropped, it looks at its own backtrace, demangles symbols, and looks for the caller of its own drop to see what test it's in.
then it sends a notification to ifttt which can go to your phone or whatever other device you want to let you know that your tests are done.

it may not work if you have done other vtable/stack-smashing sins in your code. i don't know. i don't care. i'm not your mom
if you did that, you have voided the warranty of this crate... the limitations of the backtrace and rustc-demangle crates are a you problem.

## why?
DRYing things out is good. I don't want to have to add print statements to my tests to signify which test I'm in. I get to find out when they finished anywhere I am. it is like on-call but way worse.
