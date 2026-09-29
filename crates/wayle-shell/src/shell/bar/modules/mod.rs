mod battery;
mod brightness;
mod clock;
mod custom;
mod dashboard;
mod hyprland_workspaces;
mod keyboard_input;
mod media;
mod microphone;
mod network;
mod notification;
mod separator;
mod systray;
mod volume;
mod registry;

use std::rc::Rc;

use tracing::warn;
use wayle_config::schemas::bar::{BarModule, ModuleRef};
use wayle_widgets::prelude::BarSettings;

pub(crate) use self::registry::{ModuleFactory, ModuleInstance};
use crate::shell::{bar::dropdowns::DropdownRegistry, services::ShellServices};

macro_rules! register_modules {
    ($($variant:ident => $factory:ty),+ $(,)?) => {
        fn create_from_variant(
            module: BarModule,
            settings: &BarSettings,
            services: &ShellServices,
            dropdowns: &Rc<DropdownRegistry>,
            class: Option<String>,
        ) -> Option<ModuleInstance> {
            match module {
                $(BarModule::$variant => <$factory as ModuleFactory>::create(settings, services, dropdowns, class),)+
                _ => {
                    warn!(?module, "module not implemented");
                    None
                }
            }
        }
    };
}

register_modules! {
    Battery => battery::Factory,
    Brightness => brightness::Factory,
    Clock => clock::Factory,
    Dashboard => dashboard::Factory,
    HyprlandWorkspaces => hyprland_workspaces::Factory,
    KeyboardInput => keyboard_input::Factory,
    Media => media::Factory,
    Microphone => microphone::Factory,
    Network => network::Factory,
    Notifications => notification::Factory,
    Separator => separator::Factory,
    Systray => systray::Factory,
    Volume => volume::Factory,
}

pub(crate) fn create_module(
    module_ref: &ModuleRef,
    settings: &BarSettings,
    services: &ShellServices,
    dropdowns: &Rc<DropdownRegistry>,
) -> Option<ModuleInstance> {
    let module = module_ref.module();
    let class = module_ref.class().map(String::from);

    if let Some(id) = module.custom_id() {
        return custom::Factory::create_for_id(id, settings, services, dropdowns, class);
    }

    create_from_variant(module.clone(), settings, services, dropdowns, class)
}
