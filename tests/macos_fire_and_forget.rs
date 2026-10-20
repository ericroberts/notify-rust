#![cfg(all(target_os = "macos", not(feature = "preview-macos-un")))]

use notify_rust::Notification;
use std::{sync::mpsc, thread, time::Duration};

#[test]
fn dropping_handles_does_not_wait_for_interaction() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        // Cover both the buttonless case and a notification that would otherwise
        // require a response because it has an action button.
        for with_action in [false, true] {
            let mut notification = Notification::new();
            notification
                .summary("Fire-and-forget regression check")
                .body("No interaction is needed.");
            if with_action {
                notification.action("open", "Open");
            }
            drop(notification.show().unwrap());
        }
        sender.send(()).unwrap();
    });

    // Allow the backend's delivery acknowledgement timeout, but never require a
    // click or a main run loop to release the dropped handles.
    receiver
        .recv_timeout(Duration::from_secs(10))
        .expect("fire-and-forget notifications blocked waiting for interaction");
}
