use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_notification::NotificationService;

pub(crate) struct NotificationDropdownInit {
    pub notification: Arc<NotificationService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum NotificationDropdownMsg {
    DndToggled(bool),
    ClearAll,
    NotificationDismissed,
}

#[derive(Debug)]
pub(crate) enum NotificationDropdownCmd {
    NotificationsChanged,
    DndChanged(bool),
    SizeChanged { scale: f32, max_height: u32 },
    IconSourceChanged,
    TimeTick,
}
