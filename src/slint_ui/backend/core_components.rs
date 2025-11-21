use std::{collections::BTreeMap, sync::LazyLock};

use tokio::sync::mpsc;
use tokio::time::Interval;

pub enum NotifcationType {
    Toast,
    Info,
    ParameterState,
    Critical,
    Error,
}

pub struct Notification {
    notification_type: NotifcationType,
    timer: Interval, // temp type
}

// TODO: implement notifications based on timers
/*
    Notifications will be able to have their timer reset if
        - the notification is still shown
        - the contents have changed
        - the same id is triggered

    Switching from using a static btreemap to instead returning the tx from an mpsc
    for updating notifications, the caller should maintain the channel
    for oneshots, they will be triggered and dropped

    The UI needs to handle the shown notifications on its own later
*/

pub fn register_notification(notification: Notification) {
    // let (tx, mut rx) = mpsc::channel(0);
    // tokio::spawn(async move {});
}

pub fn notify_oneshot() {}
