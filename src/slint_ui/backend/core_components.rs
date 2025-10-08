use std::{collections::BTreeMap, sync::LazyLock};

pub enum NotifcationType {
    Toast,
    Info,
    ParameterState,
    Critical,
    Error,
}

pub struct Notification {
    notification_type: NotifcationType,
    timer: bool, // temp type
}

// TODO: implement notifications based on timers
/*
    Notifications will be able to have their timer reset if
        - the notification is still shown
        - the contents have changed
        - the same id is triggered
*/
static NOTIFCATION_TABLE: LazyLock<BTreeMap<u32, Notification>> =
    LazyLock::new(|| Default::default());

pub fn register_notification(id: u32, notification_type: NotifcationType) {}
