mod factory;
mod messages;
mod system_stats;
mod user_session;
mod watchers;

use std::{
    process::{Command, Stdio},
    thread,
};

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::*;

pub(super) use self::factory::Factory;
use self::{
    messages::{DashboardDropdownCmd, DashboardDropdownInit, DashboardDropdownMsg},
    system_stats::{SystemStatsInit, SystemStatsInput, SystemStatsSection},
    user_session::{UserSessionInit, UserSessionSection},
};
use crate::{i18n::t, shell::bar::dropdowns::scaled_dimension};

const BASE_WIDTH: f32 = 380.0;

fn spawn_settings_app() {
    match Command::new("wayle-settings")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(mut child) => {
            thread::spawn(move || {
                let _ = child.wait();
            });
        }
        Err(err) => {
            tracing::warn!(error = %err, "failed to spawn wayle-settings");
        }
    }
}

pub(crate) struct DashboardDropdown {
    scaled_width: i32,

    system_stats: Controller<SystemStatsSection>,
    user_session: Controller<UserSessionSection>,
}

#[relm4::component(pub(crate))]
impl Component for DashboardDropdown {
    type Init = DashboardDropdownInit;
    type Input = DashboardDropdownMsg;
    type Output = ();
    type CommandOutput = DashboardDropdownCmd;

    view! {
        #[root]
        gtk::Popover {
            set_css_classes: &["dropdown", "dashboard-dropdown"],
            set_has_arrow: false,
            #[watch]
            set_width_request: model.scaled_width,

            #[template]
            #[name = "dashboard_container"]
            Dropdown {
                #[watch]
                set_width_request: model.scaled_width,

                #[template]
                #[name = "header"]
                DropdownHeader {
                    #[template_child]
                    icon {
                        set_visible: true,
                        set_icon_name: Some("ld-layout-dashboard-symbolic"),
                    },
                    #[template_child]
                    label {
                        set_label: &t!("dropdown-dashboard-title"),
                    },
                    #[template_child]
                    actions {
                        #[template]
                        GhostIconButton {
                            add_css_class: "dashboard-settings-btn",
                            set_icon_name: "ld-settings-symbolic",
                            set_tooltip_text: Some(&t!("dropdown-dashboard-open-settings")),
                            connect_clicked => DashboardDropdownMsg::OpenSettings,
                        },
                    },
                },

                #[template]
                #[name = "content"]
                DropdownContent {
                    set_vexpand: true,

                    #[local_ref]
                    system_stats_widget -> gtk::Box {},

                    #[local_ref]
                    user_session_widget -> gtk::Box {},
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let system_stats = SystemStatsSection::builder()
            .launch(SystemStatsInit {
                sysinfo: init.sysinfo.clone(),
            })
            .detach();

        let username = std::env::var("USER").unwrap_or_else(|_| String::from("user"));
        let user_session = UserSessionSection::builder()
            .launch(UserSessionInit {
                username,
                config: init.config.clone(),
            })
            .detach();

        let scale = init.config.config().styling.scale.get().value();

        watchers::spawn(&sender, &init.config);

        let model = Self {
            scaled_width: scaled_dimension(BASE_WIDTH, scale),

            system_stats,
            user_session,
        };

        let input_sender = sender.input_sender().clone();
        root.connect_visible_notify(move |popover| {
            input_sender.emit(DashboardDropdownMsg::VisibilityChanged(
                popover.is_visible(),
            ));
        });

        let is_visible = root.is_visible();

        model
            .system_stats
            .emit(SystemStatsInput::SetActive(is_visible));

        let system_stats_widget = model.system_stats.widget();
        let user_session_widget = model.user_session.widget();

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            DashboardDropdownMsg::VisibilityChanged(visible) => {
                self.system_stats.emit(SystemStatsInput::SetActive(visible));
            }
            DashboardDropdownMsg::OpenSettings => {
                root.popdown();
                spawn_settings_app();
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: DashboardDropdownCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            DashboardDropdownCmd::ScaleChanged(scale) => {
                self.scaled_width = scaled_dimension(BASE_WIDTH, scale);
            }
        }
    }
}
