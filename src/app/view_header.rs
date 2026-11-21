use super::*;
use crate::i18n::Translations;
use crate::theme::CustomStyles;
use crate::updater::UpdateStatus;
use iced::widget::{
    button, column, container, progress_bar, row, scrollable, space, text, tooltip, Column,
};
use iced::{Alignment, Color, Element, Length, Padding};

impl App {
    pub fn view(&self, _window: iced::window::Id) -> Element<'_, Message> {
        let mut content = Column::new()
            .spacing(12)
            .padding(Padding::from([14, 16]));

        // 1. Main header (Title, Settings button, scan and export buttons)
        content = content.push(self.view_header());

        // Scan progress bar: ONLY shown for manual scan started by button
        if self.is_scanning && !self.is_background_scan {
            content = content.push(
                container(progress_bar(0.0..=1.0, self.scan_progress))
            );
        }

        // Floating notification if active
        if let Some((msg, is_success)) = &self.notification {
            content = content.push(self.view_notification(msg, *is_success));
        }

        // Modal Overlays: When any modal is active, display it as a responsive scrollable overlay
        // so that in low resolution displays (720p, laptops, non-maximized windows) content is NEVER cut off.
        let modal_element: Option<Element<'_, Message>> = if self.is_update_modal_open {
            Some(self.view_update_modal())
        } else if self.is_settings_open {
            Some(self.view_settings_panel())
        } else if self.is_about_open {
            Some(self.view_about_modal())
        } else if self.is_speed_test_open {
            Some(self.view_speed_test_modal())
        } else if self.is_audit_open {
            Some(self.view_security_audit_modal())
        } else if self.is_timeline_open {
            Some(self.view_timeline_modal())
        } else if self.is_wifi_analyzer_open {
            Some(self.view_wifi_analyzer_modal())
        } else if self.is_upnp_open {
            Some(self.view_upnp_modal())
        } else if self.is_storage_explorer_open {
            Some(self.view_storage_explorer_modal())
        } else {
            None
        };

        if let Some(modal) = modal_element {
            // Modals occupy an 80% x 80% envelope (1:8:1 both axes), centered horizontally and vertically.
            // Full-screen tools (Speed Test, Security, UPnP, Storage, Wi-Fi) expand to fill the 80% area.
            // Dialogs (Settings, About, Update) use responsive max_width and max_height so they never
            // look excessively large on big screens, while scaling down safely on smaller screens.
            let (h_margin, h_body, v_margin, v_body) = (1, 8, 1, 8);

            let modal_box = container(modal)
                .width(Length::FillPortion(h_body))
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center);

            let modal_row = row![
                space::horizontal().width(Length::FillPortion(h_margin)),
                modal_box,
                space::horizontal().width(Length::FillPortion(h_margin)),
            ]
            .height(Length::FillPortion(v_body))
            .align_y(Alignment::Center);

            let modal_wrapper = column![
                space::vertical().height(Length::FillPortion(v_margin)),
                modal_row,
                space::vertical().height(Length::FillPortion(v_margin)),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center);

            return container(modal_wrapper)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        // 2. Live Telemetry Dashboard (Download, upload speed, Wi-Fi signal, and network)
        content = content.push(self.view_telemetry_dashboard());

        // Intruder Alert Banner (Rogue Device) if unknown device is detected
        if let Some(alert) = &self.pending_intruder_alert {
            content = content.push(self.view_intruder_alert(alert));
        }

        // 3. Filter bar, categories, search, and sorting
        content = content.push(self.view_controls());

        // 4. Main content area: device list + side inspector (if open)
        let filtered_devices = self.get_filtered_devices();
        let device_list = self.view_device_list(&filtered_devices);

        let main_area: Element<'_, Message> = if let Some(selected_ip) = self.selected_device_ip {
            if let Some(selected_device) = self.devices.iter().find(|d| d.ip == selected_ip) {
                row![
                    container(device_list).width(Length::Fill).height(Length::Fill),
                    space::horizontal().width(12),
                    container(self.view_inspector(selected_device))
                        .width(Length::Fixed(450.0))
                        .height(Length::Fill)
                        .style(|_| CustomStyles::inspector_panel())
                        .padding(Padding {
                            top: 14.0,
                            right: 12.0,
                            bottom: 14.0,
                            left: 14.0,
                        }),
                ]
                .height(Length::Fill)
                .into()
            } else {
                device_list
            }
        } else {
            device_list
        };

        content = content.push(main_area);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Prominent warning banner when an unrecognized device is detected on local network
    pub(super) fn view_intruder_alert(&self, alert: &IntruderAlert) -> Element<'_, Message> {
        let title = text(Translations::intruder_alert_title(self.language))
            .size(12)
            .color(Color::from_rgb8(248, 113, 113));

        let hostname_str = alert.hostname.as_deref().unwrap_or("Sin nombre DNS");
        let info = text(format!(
            "IP: {}   |   MAC: {}   |   {}   |   Fabricante: {}",
            alert.ip,
            alert.mac,
            hostname_str,
            alert.vendor
        ))
        .size(11)
        .color(CustomStyles::text_primary());

        let btn_trust = button(text(Translations::intruder_btn_trust(self.language)).size(11))
            .style(|_, _| CustomStyles::success_button())
            .padding(Padding::from([4, 8]))
            .on_press(Message::TrustDevice(alert.mac.clone()));

        let btn_block = button(text(Translations::intruder_btn_block(self.language)).size(11))
            .style(|_, _| CustomStyles::danger_button())
            .padding(Padding::from([4, 8]))
            .on_press(Message::ToggleDeviceBlocked(alert.ip, alert.mac.clone()));

        let btn_dismiss = button(text(Translations::intruder_btn_dismiss(self.language)).size(11))
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([4, 8]))
            .on_press(Message::DismissIntruderAlert);

        let row_content = row![
            column![title, info].spacing(2),
            space::horizontal(),
            btn_trust,
            btn_block,
            btn_dismiss,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(row_content)
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_danger())
            .into()
    }


    /// Application header and redesigned modular toolbar
    pub(super) fn view_header(&self) -> Element<'_, Message> {
        // 1. Official app icon in top-left corner
        let app_icon = iced::widget::image(self.logo_handle.clone())
            .width(30)
            .height(30);

        let title_row = row![
            app_icon,
            text("NetLens").size(22).color(CustomStyles::accent_cyan()),
            text(format!("| {}", Translations::app_subtitle(self.language)))
                .size(13)
                .color(CustomStyles::text_secondary()),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let status_row = row![
            text(self.status.render(self.language))
                .size(11)
                .color(CustomStyles::text_muted()),
            button(text("🔄").size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([1, 4]))
                .on_press(Message::RefreshNetwork),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let title_col = column![title_row, status_row].spacing(2);

        // Primary top-right actions (Export + Scan)
        let scan_btn = if self.is_scanning && !self.is_background_scan {
            button(
                row![
                    text("⏳").size(12),
                    text(Translations::scanning(self.language)).size(12),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .style(|_, _| CustomStyles::secondary_button())
            .padding(Padding::from([6, 12]))
        } else {
            button(
                row![
                    text("🔍").size(12),
                    text(Translations::scan_network(self.language)).size(12),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .style(|_, _| CustomStyles::primary_button())
            .padding(Padding::from([6, 14]))
            .on_press(Message::StartScan)
        };

        let top_right_actions = row![
            // self.tip(
            //     button(text("📊 CSV").size(11))
            //         .style(|_, status| {
            //             CustomStyles::storage_action_button(
            //                 crate::theme::StorageActionTone::Neutral,
            //                 status,
            //             )
            //         })
            //         .padding(Padding::from([5, 10]))
            //         .on_press(Message::ExportCsv),
            //     Translations::tip_export_csv(self.language),
            //     tooltip::Position::Bottom,
            // ),
            // self.tip(
            //     button(text("📥 JSON").size(11))
            //         .style(|_, status| {
            //             CustomStyles::storage_action_button(
            //                 crate::theme::StorageActionTone::Neutral,
            //                 status,
            //             )
            //         })
            //         .padding(Padding::from([5, 10]))
            //         .on_press(Message::ExportJson),
            //     Translations::tip_export_json(self.language),
            //     tooltip::Position::Bottom,
            // ),
            self.tip(
                scan_btn,
                Translations::tip_scan_network(self.language),
                tooltip::Position::Bottom,
            ),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let mut top_bar = row![
            title_col,
            space::horizontal(),
        ]
        .align_y(Alignment::Center);

        if let UpdateStatus::UpdateAvailable { ref remote_version, remote_pkgrel, .. } = self.update_status {
            top_bar = top_bar.push(
                button(
                    row![
                        text("🚀").size(12),
                        text(format!("{} (v{}-{})", Translations::update_available(self.language), remote_version, remote_pkgrel))
                            .size(11),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center)
                )
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([5, 10]))
                .on_press(Message::OpenUpdateModal)
            ).push(space::horizontal());
        }

        top_bar = top_bar.push(top_right_actions);
        let header_top_bar = top_bar.width(Length::Fill);

        // 2. Modular navigation dock (Tools Bar) with per-tool accent pills, counter badges, and group dividers
        let nav_divider = || {
            container(text(""))
                .width(1)
                .height(18)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba8(71, 85, 105, 0.55))),
                    ..Default::default()
                })
        };

        let settings_open = self.is_settings_open;
        let speed_open = self.is_speed_test_open;
        let audit_open = self.is_audit_open;
        let timeline_open = self.is_timeline_open;
        let wifi_open = self.is_wifi_analyzer_open;
        let storage_open = self.is_storage_explorer_open;
        let about_open = self.is_about_open;

        // Security Audit button with dynamic score pill
        let audit_accent = if let Some(audit) = &self.security_audit_report {
            if audit.score >= 85 {
                CustomStyles::accent_emerald()
            } else if audit.score >= 60 {
                CustomStyles::accent_amber()
            } else {
                CustomStyles::accent_red()
            }
        } else {
            CustomStyles::accent_cyan()
        };

        let audit_btn_content: Element<'_, Message> = if let Some(audit) = &self.security_audit_report {
            row![
                text(Translations::security_audit_btn(self.language)).size(11),
                container(
                    text(format!("{}", audit.score))
                        .size(9)
                        .color(audit_accent),
                )
                .padding(Padding::from([1, 6]))
                .style(move |_| CustomStyles::tinted_badge_box(audit_accent)),
            ]
            .spacing(6)
            .align_y(Alignment::Center)
            .into()
        } else {
            text(Translations::security_audit_btn(self.language))
                .size(11)
                .into()
        };

        // Timeline button with event count pill
        let events_cnt = self.network_events.len();
        let timeline_btn_content: Element<'_, Message> = row![
            text(Translations::timeline_btn(self.language)).size(11),
            container(
                text(format!("{}", events_cnt))
                    .size(9)
                    .color(CustomStyles::accent_purple()),
            )
            .padding(Padding::from([1, 6]))
            .style(|_| CustomStyles::tinted_badge_box(CustomStyles::accent_purple())),
        ]
        .spacing(6)
        .align_y(Alignment::Center)
        .into();

        let nav_tools = row![
            // Settings button
            self.tip(
                button(text(Translations::settings(self.language)).size(11))
                    .style(move |_, status| {
                        CustomStyles::nav_tool_button(
                            settings_open,
                            CustomStyles::accent_cyan(),
                            status,
                        )
                    })
                    .padding(Padding::from([5, 11]))
                    .on_press(if settings_open {
                        Message::CloseSettings
                    } else {
                        Message::OpenSettings
                    }),
                Translations::tip_settings(self.language),
                tooltip::Position::Bottom,
            ),
            nav_divider(),
            // Speed Test button
            self.tip(
                button(text(Translations::speed_test_btn(self.language)).size(11))
                    .style(move |_, status| {
                        CustomStyles::nav_tool_button(
                            speed_open,
                            CustomStyles::accent_emerald(),
                            status,
                        )
                    })
                    .padding(Padding::from([5, 11]))
                    .on_press(if speed_open {
                        Message::CloseSpeedTest
                    } else {
                        Message::OpenSpeedTest
                    }),
                Translations::tip_speed_test(self.language),
                tooltip::Position::Bottom,
            ),
            // Wi-Fi Spectrum Analyzer button
            self.tip(
                button(text(Translations::wifi_analyzer_btn(self.language)).size(11))
                    .style(move |_, status| {
                        CustomStyles::nav_tool_button(
                            wifi_open,
                            CustomStyles::accent_amber(),
                            status,
                        )
                    })
                    .padding(Padding::from([5, 11]))
                    .on_press(if wifi_open {
                        Message::CloseWifiAnalyzer
                    } else {
                        Message::OpenWifiAnalyzer
                    }),
                Translations::tip_wifi_analyzer(self.language),
                tooltip::Position::Bottom,
            ),
            nav_divider(),
            // Security Audit button
            self.tip(
                button(audit_btn_content)
                    .style(move |_, status| {
                        CustomStyles::nav_tool_button(audit_open, audit_accent, status)
                    })
                    .padding(Padding::from([5, 11]))
                    .on_press(if audit_open {
                        Message::CloseSecurityAudit
                    } else {
                        Message::OpenSecurityAudit
                    }),
                Translations::tip_security_audit(self.language),
                tooltip::Position::Bottom,
            ),
            // Event History button (Timeline)
            self.tip(
                button(timeline_btn_content)
                    .style(move |_, status| {
                        CustomStyles::nav_tool_button(
                            timeline_open,
                            CustomStyles::accent_purple(),
                            status,
                        )
                    })
                    .padding(Padding::from([5, 11]))
                    .on_press(if timeline_open {
                        Message::CloseTimeline
                    } else {
                        Message::OpenTimeline
                    }),
                Translations::tip_timeline(self.language),
                tooltip::Position::Bottom,
            ),
            nav_divider(),
            // Router USB Storage Explorer button (NAS)
            self.tip(
                button(text(Translations::storage_btn_header(self.language)).size(11))
                    .style(move |_, status| {
                        CustomStyles::nav_tool_button(
                            storage_open,
                            CustomStyles::accent_cyan(),
                            status,
                        )
                    })
                    .padding(Padding::from([5, 11]))
                    .on_press(if storage_open {
                        Message::CloseStorageExplorer
                    } else {
                        Message::OpenStorageExplorer
                    }),
                Translations::tip_router_storage(self.language),
                tooltip::Position::Bottom,
            ),
            nav_divider(),
            // Classic About NetLens button
            self.tip(
                button(text(Translations::about_btn(self.language)).size(11))
                    .style(move |_, status| {
                        CustomStyles::nav_tool_button(
                            about_open,
                            CustomStyles::accent_cyan(),
                            status,
                        )
                    })
                    .padding(Padding::from([5, 11]))
                    .on_press(if about_open {
                        Message::CloseAbout
                    } else {
                        Message::OpenAbout
                    }),
                Translations::tip_about(self.language),
                tooltip::Position::Bottom,
            ),
        ]
        .spacing(7)
        .align_y(Alignment::Center);

        let tools_scroll = scrollable(nav_tools)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new()
                    .width(3.0)
                    .margin(0.0)
                    .scroller_width(3.0),
            ))
            .style(CustomStyles::discrete_horizontal_scrollbar_style())
            .width(Length::Fill);

        let tools_toolbar = container(tools_scroll)
            .padding(Padding::from([6, 8]))
            .width(Length::Fill)
            .style(|_| CustomStyles::nav_toolbar_container());

        column![
            header_top_bar,
            tools_toolbar,
        ]
        .spacing(8)
        .into()
    }
}
