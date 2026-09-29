use relm4::prelude::*;

use super::{DashboardDropdown, messages::DashboardDropdownInit};
use crate::shell::{
    bar::dropdowns::{DropdownFactory, DropdownInstance},
    services::ShellServices,
};

pub(crate) struct Factory;

impl DropdownFactory for Factory {
    fn create(services: &ShellServices) -> Option<DropdownInstance> {
        let init = DashboardDropdownInit {
            config: services.config.clone(),
            sysinfo: services.sysinfo.clone(),
        };

        let controller = DashboardDropdown::builder().launch(init).detach();
        let popover = controller.widget().clone();
        Some(DropdownInstance::new(popover, Box::new(controller)))
    }
}
