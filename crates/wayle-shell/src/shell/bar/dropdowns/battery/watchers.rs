use std::sync::Arc;

use relm4::ComponentSender;
use wayle_config::ConfigService;
use wayle_widgets::watch;

use super::{BatteryDropdown, messages::BatteryDropdownCmd};

pub(super) fn spawn(sender: &ComponentSender<BatteryDropdown>, config: &Arc<ConfigService>) {
    let scale = config.config().styling.scale.clone();
    let max_height = config.config().bar.dropdown_battery_max_height.clone();
    watch!(sender, [scale.watch(), max_height.watch()], |out| {
        let _ = out.send(BatteryDropdownCmd::SizeChanged {
            scale: scale.get().value(),
            max_height: max_height.get(),
        });
    });
}
