//! Manual regression check: `cargo run --example body_click` on macOS.
//! Click the notification body; the callback must receive Default, not Expired.
mod common;

fn main() {
    if !common::setup(file!()) {
        return;
    }

    let worker = std::thread::spawn(|| {
        use notify_rust::{Notification, NotificationResponse};

        let mut called = false;
        Notification::new()
            .summary("Body click regression check")
            .body("Click this notification body (there are no action buttons).")
            .show()
            .unwrap()
            .wait_for_response(|response: &NotificationResponse| {
                assert_eq!(*response, NotificationResponse::Default);
                called = true;
            })
            .unwrap();
        assert!(called, "the response handler must run");
    });
    common::run_main_loop_while(worker).unwrap();
}
