use super::*;
use crate::i18n::{Language, Translations};
use crate::notification::{NotificationDetailLevel, NotificationUrgency};
use crate::theme::CustomStyles;
use iced::widget::{
    button, column, container, pick_list, row, scrollable, space, text,
    text_input, tooltip,
};
use iced::{alignment, Alignment, Color, Element, Length, Padding};

impl App {
    /// Settings / Configuration Panel (UI Theme and Language among top 10 world languages)
    pub(super) fn view_settings_panel(&self) -> Element<'_, Message> {
        let accent = CustomStyles::accent_cyan();
        let header_badge = container(text("⚙").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::settings(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::theme_desc(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseSettings),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Setting 1 & 2: UI Theme & Application Language grouped in a clean card
        let theme_box = container(
            row![
                column![
                    text(Translations::theme_label(self.language))
                        .size(13)
                        .color(CustomStyles::text_primary()),
                    text(Translations::theme_desc(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                ]
                .spacing(2)
                .width(Length::Fill),
                pick_list(
                    AVAILABLE_THEMES,
                    Some(&self.current_theme),
                    |t| Message::ThemeChanged(t.clone()),
                ),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::inset_container());

        let lang_box = container(
            row![
                column![
                    text(Translations::language_label(self.language))
                        .size(13)
                        .color(CustomStyles::text_primary()),
                    text(Translations::language_desc(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                ]
                .spacing(2)
                .width(Length::Fill),
                pick_list(
                    Language::ALL,
                    Some(self.language),
                    Message::LanguageChanged,
                ),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::inset_container());

        let appearance_card = container(row![theme_box, lang_box].spacing(10))
            .padding(12)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // Setting 3: Periodic Automatic Scan (Auto-Discovery)
        let intervals: &[(u32, &'static str)] = &[
            (0, Translations::auto_scan_disabled(self.language)),
            (30, Translations::auto_scan_30s(self.language)),
            (60, Translations::auto_scan_1m(self.language)),
            (120, Translations::auto_scan_2m(self.language)),
            (300, Translations::auto_scan_5m(self.language)),
        ];

        let mut auto_scan_buttons = row![].spacing(6).align_y(Alignment::Center);
        for &(secs, label) in intervals {
            let is_active = self.auto_scan_interval_secs == secs;
            let btn = button(text(label).size(11))
                .style(move |_, status| {
                    if is_active {
                        CustomStyles::primary_button()
                    } else {
                        CustomStyles::storage_action_button(
                            crate::theme::StorageActionTone::Neutral,
                            status,
                        )
                    }
                })
                .padding(Padding::from([5, 10]))
                .on_press(Message::AutoScanIntervalChanged(secs));
            auto_scan_buttons = auto_scan_buttons.push(btn);
        }

        let auto_scan_section = row![
            column![
                text(Translations::auto_scan_title(self.language))
                    .size(13)
                    .color(CustomStyles::accent_cyan()),
                text(Translations::auto_scan_desc(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(2)
            .width(Length::Fill),
            auto_scan_buttons,
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let auto_scan_card = container(auto_scan_section)
            .padding(12)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // Setting 4: Background & System Autostart (2 side-by-side inset tiles)
        let autostart_active = self.autostart_enabled;
        let autostart_btn = button(
            text(if autostart_active {
                Translations::btn_enabled(self.language)
            } else {
                Translations::btn_disabled(self.language)
            })
            .size(11),
        )
        .style(move |_, status| {
            let tone = if autostart_active {
                crate::theme::StorageActionTone::Success
            } else {
                crate::theme::StorageActionTone::Neutral
            };
            CustomStyles::storage_action_button(tone, status)
        })
        .padding(Padding::from([5, 12]))
        .on_press(Message::ToggleAutostart(!self.autostart_enabled));

        let minimize_active = self.minimize_on_close;
        let minimize_btn = button(
            text(if minimize_active {
                Translations::btn_enabled(self.language)
            } else {
                Translations::btn_disabled(self.language)
            })
            .size(11),
        )
        .style(move |_, status| {
            let tone = if minimize_active {
                crate::theme::StorageActionTone::Success
            } else {
                crate::theme::StorageActionTone::Neutral
            };
            CustomStyles::storage_action_button(tone, status)
        })
        .padding(Padding::from([5, 12]))
        .on_press(Message::ToggleMinimizeOnClose(!self.minimize_on_close));

        let autostart_tile = container(
            row![
                column![
                    text(Translations::autostart_title(self.language))
                        .size(12)
                        .color(CustomStyles::text_primary()),
                    text(Translations::autostart_desc(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                ]
                .spacing(2)
                .width(Length::Fill),
                autostart_btn,
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::inset_container());

        let minimize_tile = container(
            row![
                column![
                    text(Translations::minimize_on_close_title(self.language))
                        .size(12)
                        .color(CustomStyles::text_primary()),
                    text(Translations::minimize_on_close_desc(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                ]
                .spacing(2)
                .width(Length::Fill),
                minimize_btn,
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::inset_container());

        let background_card = container(row![autostart_tile, minimize_tile].spacing(10))
            .padding(12)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // Setting 5: Router Control (Linksys MR73 Mesh / JNAP)
        let router_ip_input = text_input("192.168.1.1", &self.router_ip_input)
            .on_input(Message::RouterIpChanged)
            .padding(Padding::from([5, 8]))
            .width(150);

        let router_pass_input = text_input(
            Translations::router_pass_label(self.language),
            &self.router_password_input,
        )
        .on_input(Message::RouterPasswordChanged)
        .secure(true)
        .padding(Padding::from([5, 8]))
        .width(Length::Fill);

        let test_router_btn = if self.is_testing_router {
            button(text("...").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 12]))
        } else {
            button(text(Translations::router_test_btn(self.language)).size(11))
                .style(|_, status| {
                    CustomStyles::storage_action_button(
                        crate::theme::StorageActionTone::Primary,
                        status,
                    )
                })
                .padding(Padding::from([5, 12]))
                .on_press(Message::TestRouterConnection)
        };

        let mut router_section = column![
            text(Translations::router_settings_title(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            container(
                row![
                    text(Translations::router_ip_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary()),
                    router_ip_input,
                    space::horizontal().width(10),
                    text(Translations::router_pass_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary()),
                    router_pass_input,
                    test_router_btn,
                ]
                .align_y(Alignment::Center)
                .spacing(8),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::inset_container()),
        ]
        .spacing(8);

        if let Some((msg, is_ok)) = &self.router_status_msg {
            let color = if *is_ok {
                CustomStyles::accent_emerald()
            } else {
                Color::from_rgb8(239, 68, 68)
            };
            router_section = router_section.push(text(msg).size(11).color(color));
        }

        let router_card = container(router_section)
            .padding(12)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // Setting 6: System Notifications & Parameterizable Alerts
        let notif_enabled = self.notification_settings.enabled;
        let notif_master_btn = button(
            text(if notif_enabled {
                Translations::notif_master_enabled(self.language)
            } else {
                Translations::notif_master_disabled(self.language)
            })
            .size(11),
        )
        .style(move |_, status| {
            let tone = if notif_enabled {
                crate::theme::StorageActionTone::Success
            } else {
                crate::theme::StorageActionTone::Neutral
            };
            CustomStyles::storage_action_button(tone, status)
        })
        .padding(Padding::from([5, 12]))
        .on_press(Message::ToggleNotificationsMaster(!self.notification_settings.enabled));

        let mut notif_section = column![
            row![
                column![
                    text(Translations::notif_settings_title(self.language))
                        .size(13)
                        .color(CustomStyles::accent_cyan()),
                    text(Translations::notif_settings_desc(self.language))
                        .size(11)
                        .color(CustomStyles::text_muted()),
                ]
                .spacing(2)
                .width(Length::Fill),
                notif_master_btn,
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .spacing(10);

        if self.notification_settings.enabled {
            let trigger_btn = |active: bool, msg: Message| {
                button(
                    text(if active {
                        Translations::btn_enabled(self.language)
                    } else {
                        Translations::btn_disabled(self.language)
                    })
                    .size(10),
                )
                .style(move |_, status| {
                    let tone = if active {
                        crate::theme::StorageActionTone::Success
                    } else {
                        crate::theme::StorageActionTone::Neutral
                    };
                    CustomStyles::storage_action_button(tone, status)
                })
                .padding(Padding::from([3, 10]))
                .on_press(msg)
            };

            let triggers_col = column![
                text(Translations::notif_section_triggers(self.language))
                    .size(11)
                    .color(CustomStyles::accent_cyan()),
                row![
                    column![
                        text(Translations::notif_realtime_label(self.language))
                            .size(11)
                            .color(CustomStyles::accent_cyan()),
                        text(Translations::notif_realtime_desc(self.language))
                            .size(9)
                            .color(CustomStyles::text_muted()),
                    ]
                    .width(Length::Fill),
                    trigger_btn(
                        self.notification_settings.realtime_netlink_monitor,
                        Message::ToggleRealtimeMonitor(
                            !self.notification_settings.realtime_netlink_monitor
                        )
                    ),
                ]
                .align_y(Alignment::Center),
                row![
                    column![
                        text(Translations::notif_heartbeat_label(self.language))
                            .size(11)
                            .color(CustomStyles::accent_cyan()),
                        text(Translations::notif_heartbeat_desc(self.language))
                            .size(9)
                            .color(CustomStyles::text_muted()),
                    ]
                    .width(Length::Fill),
                    trigger_btn(
                        self.notification_settings.fast_disconnection_heartbeat,
                        Message::ToggleHeartbeat(
                            !self.notification_settings.fast_disconnection_heartbeat
                        )
                    ),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_unknown(self.language))
                        .size(11)
                        .color(CustomStyles::text_primary())
                        .width(Length::Fill),
                    trigger_btn(
                        self.notification_settings.notify_unknown_intruders,
                        Message::ToggleNotifyUnknown(
                            !self.notification_settings.notify_unknown_intruders
                        )
                    ),
                ]
                .align_y(Alignment::Center),
                row![
                    row![
                        text(Translations::notif_trigger_unwhitelisted_reconnect(
                            self.language
                        ))
                        .size(11)
                        .color(Color::from_rgb8(245, 158, 11)),
                        container(
                            text(Translations::badge_priority(self.language))
                                .size(9)
                                .color(Color::WHITE)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink)
                        .padding(Padding::from([1, 6]))
                        .style(|_| {
                            CustomStyles::badge_pill(Color::from_rgb8(180, 83, 9), Color::WHITE)
                        }),
                    ]
                    .spacing(6)
                    .width(Length::Fill)
                    .align_y(Alignment::Center),
                    trigger_btn(
                        self.notification_settings.notify_known_unwhitelisted_reconnect,
                        Message::ToggleNotifyKnownUnwhitelisted(
                            !self.notification_settings.notify_known_unwhitelisted_reconnect
                        )
                    ),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_whitelisted_reconnect(
                        self.language
                    ))
                    .size(11)
                    .color(CustomStyles::text_primary())
                    .width(Length::Fill),
                    trigger_btn(
                        self.notification_settings.notify_whitelisted_reconnect,
                        Message::ToggleNotifyWhitelisted(
                            !self.notification_settings.notify_whitelisted_reconnect
                        )
                    ),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_disconnected(self.language))
                        .size(11)
                        .color(CustomStyles::text_primary())
                        .width(Length::Fill),
                    trigger_btn(
                        self.notification_settings.notify_device_disconnected,
                        Message::ToggleNotifyDisconnected(
                            !self.notification_settings.notify_device_disconnected
                        )
                    ),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_outages(self.language))
                        .size(11)
                        .color(CustomStyles::text_primary())
                        .width(Length::Fill),
                    trigger_btn(
                        self.notification_settings.notify_internet_outages,
                        Message::ToggleNotifyOutages(
                            !self.notification_settings.notify_internet_outages
                        )
                    ),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_trigger_security(self.language))
                        .size(11)
                        .color(CustomStyles::text_primary())
                        .width(Length::Fill),
                    trigger_btn(
                        self.notification_settings.notify_security_critical,
                        Message::ToggleNotifySecurity(
                            !self.notification_settings.notify_security_critical
                        )
                    ),
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(6);

            // Subsection B: Behavior (Urgency, Duration, and Sound)
            let urgencies = NotificationUrgency::ALL;
            let mut urgency_buttons = row![].spacing(6).align_y(Alignment::Center);
            for &u in urgencies {
                let is_active = self.notification_settings.urgency == u;
                let btn = button(text(u.label(self.language)).size(10))
                    .style(move |_, status| {
                        if is_active {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Neutral,
                                status,
                            )
                        }
                    })
                    .padding(Padding::from([3, 9]))
                    .on_press(Message::ChangeNotificationUrgency(u));
                urgency_buttons = urgency_buttons.push(btn);
            }

            let timeouts: &[(u32, &'static str)] =
                &[(3, "3s"), (5, "5s"), (7, "7s"), (15, "15s"), (30, "30s")];
            let mut timeout_buttons = row![].spacing(6).align_y(Alignment::Center);
            for &(s, label) in timeouts {
                let is_active = self.notification_settings.timeout_secs == s;
                let btn = button(text(label).size(10))
                    .style(move |_, status| {
                        if is_active {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Neutral,
                                status,
                            )
                        }
                    })
                    .padding(Padding::from([3, 9]))
                    .on_press(Message::ChangeNotificationTimeout(s));
                timeout_buttons = timeout_buttons.push(btn);
            }

            let sound_btn = trigger_btn(
                self.notification_settings.play_sound,
                Message::ToggleNotificationPlaySound(!self.notification_settings.play_sound),
            );

            let behavior_col = column![
                text(Translations::notif_section_behavior(self.language))
                    .size(11)
                    .color(CustomStyles::accent_cyan()),
                row![
                    text(Translations::notif_urgency_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary())
                        .width(140),
                    urgency_buttons,
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_timeout_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary())
                        .width(140),
                    timeout_buttons,
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_sound_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary())
                        .width(Length::Fill),
                    sound_btn,
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(6);

            // Subsection C: Content & Format
            let details = NotificationDetailLevel::ALL;
            let mut detail_buttons = row![].spacing(6).align_y(Alignment::Center);
            for &d in details {
                let is_active = self.notification_settings.detail_level == d;
                let btn = button(text(d.label(self.language)).size(10))
                    .style(move |_, status| {
                        if is_active {
                            CustomStyles::primary_button()
                        } else {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Neutral,
                                status,
                            )
                        }
                    })
                    .padding(Padding::from([3, 9]))
                    .on_press(Message::ChangeNotificationDetail(d));
                detail_buttons = detail_buttons.push(btn);
            }

            let mac_btn = trigger_btn(
                self.notification_settings.show_mac,
                Message::ToggleNotificationShowMac(!self.notification_settings.show_mac),
            );
            let vendor_btn = trigger_btn(
                self.notification_settings.show_vendor,
                Message::ToggleNotificationShowVendor(!self.notification_settings.show_vendor),
            );
            let prefix_input = text_input("NetLens", &self.notification_prefix_input)
                .on_input(Message::NotificationPrefixChanged)
                .padding(Padding::from([4, 8]))
                .width(160);

            let test_btn = button(text(Translations::notif_test_btn(self.language)).size(10))
                .style(|_, status| {
                    CustomStyles::storage_action_button(
                        crate::theme::StorageActionTone::Primary,
                        status,
                    )
                })
                .padding(Padding::from([4, 10]))
                .on_press(Message::SendTestNotification);

            let content_col = column![
                text(Translations::notif_section_content(self.language))
                    .size(11)
                    .color(CustomStyles::accent_cyan()),
                row![
                    text(Translations::notif_detail_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary())
                        .width(140),
                    detail_buttons,
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_show_mac_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary())
                        .width(Length::Fill),
                    mac_btn,
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_show_vendor_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary())
                        .width(Length::Fill),
                    vendor_btn,
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::notif_prefix_label(self.language))
                        .size(11)
                        .color(CustomStyles::text_secondary())
                        .width(140),
                    prefix_input,
                    space::horizontal(),
                    test_btn,
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(6);

            notif_section = notif_section
                .push(
                    container(triggers_col)
                        .padding(10)
                        .width(Length::Fill)
                        .style(|_| CustomStyles::inset_container()),
                )
                .push(
                    container(behavior_col)
                        .padding(10)
                        .width(Length::Fill)
                        .style(|_| CustomStyles::inset_container()),
                )
                .push(
                    container(content_col)
                        .padding(10)
                        .width(Length::Fill)
                        .style(|_| CustomStyles::inset_container()),
                );
        }

        let notifications_card = container(notif_section)
            .padding(12)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // Technical system summary as 3 recessed metric tiles
        let gw_text = self
            .network
            .gateway_ip
            .map(|g| g.to_string())
            .unwrap_or_else(|| Translations::not_detected(self.language).to_string());

        let sys_tiles = row![
            container(
                column![
                    text(Translations::sys_active_iface(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                    text(&self.network.interface)
                        .size(13)
                        .color(CustomStyles::accent_cyan()),
                ]
                .spacing(2),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::inset_container()),
            container(
                column![
                    text(Translations::sys_subnet(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                    text(&self.network.subnet_cidr)
                        .size(13)
                        .color(CustomStyles::text_primary()),
                ]
                .spacing(2),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::inset_container()),
            container(
                column![
                    text(Translations::sys_gateway(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                    text(gw_text)
                        .size(13)
                        .color(CustomStyles::accent_emerald()),
                ]
                .spacing(2),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::inset_container()),
        ]
        .spacing(10);

        let sys_info_card = container(
            column![
                text(Translations::sys_info_title(self.language))
                    .size(13)
                    .color(CustomStyles::accent_cyan()),
                sys_tiles,
            ]
            .spacing(8),
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        // Setting 7: About NetLens & Author Info
        let about_card = container(
            row![
                column![
                    text(Translations::about_modal_title(self.language))
                        .size(13)
                        .color(CustomStyles::accent_cyan()),
                    text(format!(
                        "Omar Murcia • MIT License • v{}",
                        env!("CARGO_PKG_VERSION")
                    ))
                    .size(11)
                    .color(CustomStyles::text_secondary()),
                ]
                .spacing(2)
                .width(Length::Fill),
                button(text(Translations::about_btn(self.language)).size(11))
                    .style(|_, status| {
                        CustomStyles::storage_action_button(
                            crate::theme::StorageActionTone::Primary,
                            status,
                        )
                    })
                    .padding(Padding::from([6, 14]))
                    .on_press(Message::OpenAbout),
            ]
            .align_y(Alignment::Center),
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_| CustomStyles::card_highlighted());

        let body_content = column![
            appearance_card,
            auto_scan_card,
            background_card,
            router_card,
            notifications_card,
            sys_info_card,
            about_card,
        ]
        .spacing(10)
        .padding(Padding {
            top: 2.0,
            right: 0.0,
            bottom: 4.0,
            left: 0.0,
        });

        let scrollable_body = scrollable(body_content)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(5.0)
                    .scroller_width(5.0)
                    .margin(2.0)
                    .spacing(6.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Shrink);

        let panel_layout = column![
            title_row,
            container(scrollable_body)
                .width(Length::Fill)
                .max_height(565.0),
        ]
        .spacing(12);

        container(panel_layout)
            .padding(22)
            .width(860)
            .max_height(685)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

}
