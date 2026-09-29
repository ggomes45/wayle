use wayle_config::ConfigService;
use wayle_sysinfo::SysinfoService;

pub(crate) struct DashboardDropdownInit {
    pub config: Arc<ConfigService>,
    pub sysinfo: Arc<SysinfoService>,
}

#[derive(Debug)]
pub(crate) enum DashboardDropdownMsg {
    VisibilityChanged(bool),
    OpenSettings,
}

#[derive(Debug)]
pub(crate) enum DashboardDropdownCmd {
    ScaleChanged(f32),
}
