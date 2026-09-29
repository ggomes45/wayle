mod types;

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
/// Notification module configuration and popup types.
pub mod notification;
mod separator;
mod systray;
mod volume;

pub use battery::BatteryConfig;
pub use brightness::BrightnessConfig;
pub use clock::ClockConfig;
pub use custom::{CustomModuleDefinition, ExecutionMode, RestartDelay, RestartPolicy};
pub use dashboard::DashboardConfig;
pub use hyprland_workspaces::{
    ActiveIndicator, DisplayMode, HyprlandWorkspacesConfig, Numbering, UrgentMode, WorkspaceStyle,
};
pub use keyboard_input::KeyboardInputConfig;
pub use media::{BUILTIN_MAPPINGS, MediaConfig, MediaIconType};
pub use microphone::MicrophoneConfig;
pub use network::NetworkConfig;
pub use notification::{
    IconSource, NotificationConfig, PopupCloseBehavior, PopupMonitor, PopupPosition, StackingOrder,
    UrgencyBarThreshold,
};
pub use separator::SeparatorConfig;
pub use systray::{SystrayConfig, TrayItemOverride};
pub use types::TimeFormat;
pub use volume::{AppIconSource, VolumeConfig};
use wayle_derive::wayle_config;

use crate::ConfigProperty;

/// Configuration for all available Wayle modules.
#[wayle_config]
pub struct ModulesConfig {
    /// Battery status module.
    pub battery: BatteryConfig,
    /// Backlight brightness module.
    pub brightness: BrightnessConfig,
    /// Clock display module.
    pub clock: ClockConfig,
    /// Dashboard module.
    pub dashboard: DashboardConfig,
    /// Hyprland workspace switcher module.
    #[serde(rename = "hyprland-workspaces")]
    pub hyprland_workspaces: HyprlandWorkspacesConfig,
    /// Keyboard input module.
    #[serde(rename = "keyboard-input")]
    pub keyboard_input: KeyboardInputConfig,
    /// Media player module.
    pub media: MediaConfig,
    /// Microphone input module.
    pub microphone: MicrophoneConfig,
    /// Network connection module.
    pub network: NetworkConfig,
    /// Notification center module.
    #[serde(rename = "notifications")]
    #[wayle(deprecated_alias = "notification")]
    pub notifications: NotificationConfig,
    /// Separator module.
    pub separator: SeparatorConfig,
    /// System tray module.
    pub systray: SystrayConfig,
    /// Volume control module.
    pub volume: VolumeConfig,
    /// Custom user-defined modules, each backed by a shell command. See
    /// [`CustomModuleDefinition`] for all fields (id, command, interval, click
    /// actions, icons, etc.). Reference them in a layout with `custom-<id>`.
    ///
    /// ## Example
    ///
    /// ```toml
    /// [[modules.custom]]
    /// id = "gpu-temp"
    /// command = "nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader"
    /// interval-ms = 5000
    /// icon-name = "ld-thermometer-symbolic"
    ///
    /// [[modules.custom]]
    /// id = "weather"
    /// command = "curl -s wttr.in/?format=%t"
    /// interval-ms = 600000
    /// ```
    #[default(Vec::new())]
    pub custom: ConfigProperty<Vec<CustomModuleDefinition>>,
}
