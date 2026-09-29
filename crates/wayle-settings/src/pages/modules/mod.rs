//! Per-module settings pages. Each module exports an `entry()` returning a `LeafEntry`.

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
mod notification_module;
mod separator;
mod systray;
mod volume;

use wayle_config::Config;

use super::nav::LeafEntry;

pub(crate) fn factories() -> Vec<fn(&Config) -> LeafEntry> {
    vec![
        battery::entry,
        brightness::entry,
        clock::entry,
        custom::entry,
        dashboard::entry,
        hyprland_workspaces::entry,
        keyboard_input::entry,
        media::entry,
        microphone::entry,
        network::entry,
        notification_module::entry,
        separator::entry,
        systray::entry,
        volume::entry,
    ]
}
