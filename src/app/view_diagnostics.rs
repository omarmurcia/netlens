use super::*;
use crate::i18n::Translations;
use crate::security_audit::AuditSeverity;
use crate::speedtest::SpeedTestStage;
use crate::theme::CustomStyles;
use iced::widget::{
    button, column, container, progress_bar, row, scrollable, space, text, tooltip, Column,
};
use iced::{alignment, Alignment, Color, Element, Length, Padding};

impl App {
    /// Internet Speed Test Modal (Real-time Ping, Download, and Upload)
    pub(super) fn view_speed_test_modal(&self) -> Element<'_, Message> {
        let is_running = self.speed_test_stage == SpeedTestStage::TestingPing
            || self.speed_test_stage == SpeedTestStage::TestingDownload
            || self.speed_test_stage == SpeedTestStage::TestingUpload;

        let accent = CustomStyles::accent_cyan();
        let header_badge = container(text("🚀").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::speed_test_title(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::speed_test_subtitle(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseSpeedTest),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let (progress, stage_text, stage_color) = match self.speed_test_stage {
            SpeedTestStage::Idle => (
                0.0,
                Translations::speed_stage_idle(self.language),
                CustomStyles::text_secondary(),
            ),
            SpeedTestStage::TestingPing => (
                0.25,
                Translations::speed_stage_ping(self.language),
                CustomStyles::accent_amber(),
            ),
            SpeedTestStage::TestingDownload => (
                0.65,
                Translations::speed_stage_download(self.language),
                CustomStyles::accent_emerald(),
            ),
            SpeedTestStage::TestingUpload => (
                0.90,
                Translations::speed_stage_upload(self.language),
                CustomStyles::accent_purple(),
            ),
            SpeedTestStage::Completed => (
                1.0,
                Translations::speed_stage_complete(self.language),
                CustomStyles::accent_emerald(),
            ),
            SpeedTestStage::Failed => (
                0.0,
                Translations::speed_stage_failed(self.language),
                Color::from_rgb8(239, 68, 68),
            ),
        };

        let server_name = self
            .speed_test_result
            .as_ref()
            .map(|r| r.server.as_str())
            .unwrap_or("Netflix Open Connect / Cloudflare CDN");

        // Top Stage & Server Status Bar
        let status_banner = container(
            column![
                row![
                    text(stage_text).size(12).color(stage_color),
                    space::horizontal(),
                    container(
                        text(format!("🌐 {}", server_name))
                            .size(10)
                            .color(CustomStyles::text_secondary()),
                    )
                    .padding(Padding::from([2, 8]))
                    .style(|_| CustomStyles::inset_container()),
                    container(
                        text(format!("🔌 {}", self.network.interface))
                            .size(10)
                            .color(CustomStyles::accent_cyan()),
                    )
                    .padding(Padding::from([2, 8]))
                    .style(|_| CustomStyles::inset_container()),
                    text(format!("{:.0}%", progress * 100.0))
                        .size(11)
                        .color(stage_color),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                progress_bar(0.0..=1.0, progress),
            ]
            .spacing(8),
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        // Metric Cards (Ping, Download, Upload)
        let ping_str = self
            .speed_test_ping
            .map(|p| format!("{:.1} ms", p))
            .unwrap_or_else(|| "-- ms".to_string());

        let card_ping = container(
            row![
                container(text("⚡").size(16))
                    .width(34)
                    .height(34)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_| CustomStyles::modal_icon_badge(CustomStyles::accent_amber())),
                column![
                    text(Translations::speed_test_ping(self.language))
                        .size(11)
                        .color(CustomStyles::accent_amber()),
                    text(ping_str).size(22).color(CustomStyles::accent_amber()),
                    text("Round-Trip Time")
                        .size(10)
                        .color(CustomStyles::text_muted()),
                ]
                .spacing(2),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(14)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_signal());

        let dl_str = self
            .speed_test_download
            .map(|d| format!("{:.2} Mbps", d))
            .unwrap_or_else(|| "-- Mbps".to_string());
        let dl_sub = self
            .speed_test_download
            .map(|d| format!("{:.2} MB/s", d / 8.0))
            .unwrap_or_else(|| "-- MB/s".to_string());

        let card_dl = container(
            row![
                container(text("⬇").size(16))
                    .width(34)
                    .height(34)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_| CustomStyles::modal_icon_badge(CustomStyles::accent_emerald())),
                column![
                    text(Translations::speed_test_download(self.language))
                        .size(11)
                        .color(CustomStyles::accent_emerald()),
                    text(dl_str).size(22).color(CustomStyles::accent_emerald()),
                    text(dl_sub).size(10).color(CustomStyles::text_secondary()),
                ]
                .spacing(2),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(14)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_download());

        let ul_str = self
            .speed_test_upload
            .map(|u| format!("{:.2} Mbps", u))
            .unwrap_or_else(|| "-- Mbps".to_string());
        let ul_sub = self
            .speed_test_upload
            .map(|u| format!("{:.2} MB/s", u / 8.0))
            .unwrap_or_else(|| "-- MB/s".to_string());

        let card_ul = container(
            row![
                container(text("⬆").size(16))
                    .width(34)
                    .height(34)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_| CustomStyles::modal_icon_badge(CustomStyles::accent_purple())),
                column![
                    text(Translations::speed_test_upload(self.language))
                        .size(11)
                        .color(CustomStyles::accent_purple()),
                    text(ul_str).size(22).color(CustomStyles::accent_purple()),
                    text(ul_sub).size(10).color(CustomStyles::text_secondary()),
                ]
                .spacing(2),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(14)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_upload());

        let metrics_row = row![card_ping, card_dl, card_ul].spacing(10);

        // Action button
        let start_btn = if is_running {
            button(text(Translations::speed_test_running(self.language)).size(12))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([7, 18]))
        } else {
            button(text(Translations::speed_test_start(self.language)).size(12))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([7, 20]))
                .on_press(Message::StartSpeedTest)
        };

        // Section: Persistent Speed Test History using our harmonized table design
        let mut history_header = row![
            text(Translations::speed_test_history_title(self.language))
                .size(13)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
        ]
        .align_y(Alignment::Center);

        if !self.speed_test_history.is_empty() {
            history_header = history_header.push(
                button(text(Translations::speed_test_history_clear(self.language)).size(10))
                    .style(|_, status| {
                        CustomStyles::storage_action_button(
                            crate::theme::StorageActionTone::Danger,
                            status,
                        )
                    })
                    .padding(Padding::from([3, 8]))
                    .on_press(Message::ClearSpeedTestHistory),
            );
        }

        let hist_table_header = container(
            row![
                container(
                    text(Translations::speed_test_col_time_server(self.language))
                        .size(11)
                        .color(CustomStyles::text_muted())
                )
                .width(Length::Fill)
                .align_x(Alignment::Start),
                container(
                    text(Translations::speed_test_ping(self.language))
                        .size(11)
                        .color(CustomStyles::accent_amber())
                )
                .width(120)
                .align_x(Alignment::Center),
                container(
                    text(Translations::speed_test_col_download(self.language))
                        .size(11)
                        .color(CustomStyles::accent_emerald())
                )
                .width(140)
                .align_x(Alignment::Center),
                container(
                    text(Translations::speed_test_col_upload(self.language))
                        .size(11)
                        .color(CustomStyles::accent_purple())
                )
                .width(140)
                .align_x(Alignment::Center),
            ]
            .align_y(Alignment::Center)
            .padding(Padding::from([6, 12])),
        )
        .width(Length::Fill)
        .style(|_| CustomStyles::storage_table_header());

        let mut hist_rows = Column::new().spacing(2).padding(4);

        if self.speed_test_history.is_empty() {
            hist_rows = hist_rows.push(
                container(
                    text(Translations::speed_test_history_empty(self.language))
                        .size(11)
                        .color(CustomStyles::text_muted()),
                )
                .padding(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            );
        } else {
            for (idx, test) in self.speed_test_history.iter().take(100).enumerate() {
                hist_rows = hist_rows.push(
                    container(
                        row![
                            column![
                                text(&test.timestamp)
                                    .size(11)
                                    .color(CustomStyles::text_primary()),
                                text(&test.server)
                                    .size(10)
                                    .color(CustomStyles::text_muted()),
                            ]
                            .width(Length::Fill),
                            container(
                                text(format!("{:.1} ms", test.ping_ms))
                                    .size(11)
                                    .color(CustomStyles::accent_amber())
                            )
                            .width(120)
                            .align_x(Alignment::Center),
                            container(
                                text(format!("{:.1} Mbps", test.download_mbps))
                                    .size(11)
                                    .color(CustomStyles::accent_emerald())
                            )
                            .width(140)
                            .align_x(Alignment::Center),
                            container(
                                text(format!("{:.1} Mbps", test.upload_mbps))
                                    .size(11)
                                    .color(CustomStyles::accent_purple())
                            )
                            .width(140)
                            .align_x(Alignment::Center),
                        ]
                        .align_y(Alignment::Center)
                        .padding(Padding::from([5, 8])),
                    )
                    .width(Length::Fill)
                    .style(move |_| CustomStyles::storage_file_row(idx)),
                );
            }
        }

        let hist_scrollable = scrollable(hist_rows)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(5.0)
                    .scroller_width(5.0)
                    .margin(2.0)
                    .spacing(4.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fill);

        let hist_table_body = container(hist_scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| CustomStyles::drop_zone_idle());

        let history_section = column![
            history_header,
            column![hist_table_header, hist_table_body].spacing(0),
        ]
        .spacing(6);

        let content = column![
            title_row,
            status_banner,
            metrics_row,
            history_section,
            row![space::horizontal(), start_btn]
                .spacing(10)
                .align_y(Alignment::Center),
        ]
        .spacing(12);

        container(content)
            .padding(22)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

    /// Top live telemetry dashboard: Download, upload speed, Wi-Fi signal, and router

    /// Modal: Comprehensive Network Security Audit
    pub(super) fn view_security_audit_modal(&self) -> Element<'_, Message> {
        let accent = CustomStyles::accent_cyan();
        let header_badge = container(text("🛡").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::audit_title(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::audit_subtitle(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseSecurityAudit),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let (score_val, grade_str, issues_cnt, crit_cnt) =
            if let Some(audit) = &self.security_audit_report {
                (
                    audit.score,
                    Translations::audit_grade_label(audit.score, self.language),
                    audit.issues_count,
                    audit.critical_count,
                )
            } else {
                (100, Translations::audit_not_run(self.language), 0, 0)
            };
        let checks_list = self
            .security_audit_report
            .as_ref()
            .map(|a| a.checks.as_slice())
            .unwrap_or(&[]);

        let score_color = if score_val >= 85 {
            CustomStyles::accent_emerald()
        } else if score_val >= 60 {
            CustomStyles::accent_amber()
        } else {
            Color::from_rgb8(239, 68, 68)
        };

        // Top KPI Score Dashboard Card
        let score_badge_box = container(
            column![
                text(format!("{}/100", score_val))
                    .size(28)
                    .color(score_color),
                text(format!(
                    "{} {}",
                    Translations::audit_score_label(self.language),
                    grade_str
                ))
                .size(11)
                .color(score_color),
            ]
            .spacing(2)
            .align_x(Alignment::Center),
        )
        .padding(Padding::from([10, 18]))
        .style(move |_| CustomStyles::tinted_badge_box(score_color));

        let kpi_devices = container(
            column![
                text(
                    Translations::audit_devices_audited(self.devices.len(), self.language)
                )
                .size(11)
                .color(CustomStyles::text_primary()),
                text(format!("{} Checks", checks_list.len()))
                    .size(10)
                    .color(CustomStyles::accent_cyan()),
            ]
            .spacing(2),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::inset_container());

        let obs_color = if issues_cnt > 0 {
            CustomStyles::accent_amber()
        } else {
            CustomStyles::accent_emerald()
        };
        let kpi_issues = container(
            column![
                text(Translations::audit_observations(issues_cnt, self.language))
                    .size(11)
                    .color(obs_color),
                text("⚠ Warnings / Info")
                    .size(10)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(2),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::inset_container());

        let crit_color = if crit_cnt > 0 {
            Color::from_rgb8(239, 68, 68)
        } else {
            CustomStyles::accent_emerald()
        };
        let kpi_crit = container(
            column![
                text(Translations::audit_critical_risks(crit_cnt, self.language))
                    .size(11)
                    .color(crit_color),
                text("🚨 High Priority")
                    .size(10)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(2),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::inset_container());

        let score_card = container(
            row![score_badge_box, kpi_devices, kpi_issues, kpi_crit]
                .spacing(10)
                .align_y(Alignment::Center),
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        let mut checks_col = Column::new().spacing(8).padding(Padding {
            top: 2.0,
            right: 0.0,
            bottom: 4.0,
            left: 0.0,
        });

        for check in checks_list {
            let (icon, sev_label, icon_color, border_bg, border_stroke) = match check.severity {
                AuditSeverity::Pass => (
                    "✅",
                    "PASS",
                    CustomStyles::accent_emerald(),
                    Color::from_rgb8(14, 32, 26),
                    Color::from_rgb8(22, 101, 52),
                ),
                AuditSeverity::Info => (
                    "ℹ️",
                    "INFO",
                    CustomStyles::accent_cyan(),
                    Color::from_rgb8(14, 28, 44),
                    Color::from_rgb8(21, 94, 117),
                ),
                AuditSeverity::Warning => (
                    "⚠️",
                    "WARN",
                    CustomStyles::accent_amber(),
                    Color::from_rgb8(40, 30, 14),
                    Color::from_rgb8(146, 64, 14),
                ),
                AuditSeverity::Critical => (
                    "🚨",
                    "CRIT",
                    Color::from_rgb8(239, 68, 68),
                    Color::from_rgb8(46, 16, 18),
                    Color::from_rgb8(153, 27, 27),
                ),
            };

            let mut check_body = column![
                row![
                    text(icon).size(14),
                    text(check.title_localized(self.language))
                        .size(13)
                        .color(icon_color),
                    space::horizontal(),
                    container(text(sev_label).size(9).color(icon_color))
                        .padding(Padding::from([2, 8]))
                        .style(move |_| CustomStyles::tinted_badge_box(icon_color)),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
                text(check.description_localized(self.language))
                    .size(11)
                    .color(CustomStyles::text_secondary()),
            ]
            .spacing(5);

            if !check.affected_devices.is_empty() {
                let mut dev_col = Column::new().spacing(5);
                let count_label = Translations::audit_affected_devices(
                    check.affected_devices.len(),
                    self.language,
                );
                dev_col = dev_col.push(
                    text(count_label)
                        .size(10)
                        .color(CustomStyles::text_muted()),
                );

                for chunk in check.affected_devices.chunks(3) {
                    let mut chunk_row = row![].spacing(6).align_y(Alignment::Center);
                    for d in chunk {
                        chunk_row = chunk_row.push(
                            container(
                                text(d)
                                    .size(10)
                                    .color(CustomStyles::accent_amber())
                                    .align_x(alignment::Horizontal::Center)
                                    .align_y(alignment::Vertical::Center),
                            )
                            .center(Length::Shrink)
                            .padding(Padding::from([2, 8]))
                            .style(|_| {
                                CustomStyles::badge_pill(
                                    Color::from_rgb8(45, 35, 15),
                                    CustomStyles::accent_amber(),
                                )
                            }),
                        );
                    }
                    dev_col = dev_col.push(chunk_row);
                }
                check_body = check_body.push(
                    container(dev_col)
                        .padding(8)
                        .width(Length::Fill)
                        .style(|_| CustomStyles::inset_container()),
                );
            }

            check_body = check_body.push(
                container(
                    row![
                        text("💡").size(11),
                        text(check.recommendation_localized(self.language))
                            .size(10)
                            .color(CustomStyles::accent_cyan()),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .padding(Padding::from([5, 8]))
                .width(Length::Fill)
                .style(|_| CustomStyles::inset_container()),
            );

            let check_card = container(check_body)
                .padding(12)
                .width(Length::Fill)
                .style(move |_| CustomStyles::audit_card(border_bg, border_stroke));

            checks_col = checks_col.push(check_card);
        }

        let checks_scrollable = container(
            scrollable(checks_col)
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(5.0)
                        .scroller_width(5.0)
                        .margin(2.0)
                        .spacing(6.0),
                ))
                .style(CustomStyles::scrollbar_style())
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let actions_row = row![
            space::horizontal(),
            button(text(Translations::audit_rerun_btn(self.language)).size(12))
                .style(|_, status| {
                    CustomStyles::storage_action_button(
                        crate::theme::StorageActionTone::Primary,
                        status,
                    )
                })
                .padding(Padding::from([6, 16]))
                .on_press(Message::RunSecurityAudit),
        ]
        .align_y(Alignment::Center);

        let content = column![
            title_row,
            score_card,
            checks_scrollable,
            actions_row,
        ]
        .spacing(12);

        container(content)
            .padding(22)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

    /// Modal: Network Event History & Timeline
    pub(super) fn view_timeline_modal(&self) -> Element<'_, Message> {
        let accent = CustomStyles::accent_purple();
        let header_badge = container(text("🕒").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::timeline_modal_title(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::timeline_modal_desc(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseTimeline),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let uptime_pct = self.uptime_tracker.uptime_percentage();
        let online_status = if self.uptime_tracker.is_online {
            (
                Translations::timeline_online(self.language),
                CustomStyles::accent_emerald(),
            )
        } else {
            (
                Translations::timeline_outage(self.language),
                Color::from_rgb8(239, 68, 68),
            )
        };

        let summary_row = container(
            row![
                container(
                    text(Translations::timeline_total_events(
                        self.network_events.len(),
                        self.language
                    ))
                    .size(11)
                    .color(CustomStyles::text_primary()),
                )
                .padding(Padding::from([5, 10]))
                .style(|_| CustomStyles::inset_container()),
                container(
                    text(online_status.0)
                        .size(11)
                        .color(online_status.1),
                )
                .padding(Padding::from([5, 10]))
                .style(move |_| CustomStyles::tinted_badge_box(online_status.1)),
                container(
                    text(Translations::timeline_uptime_pct(
                        uptime_pct as f64,
                        self.language
                    ))
                    .size(11)
                    .color(CustomStyles::accent_cyan()),
                )
                .padding(Padding::from([5, 10]))
                .style(|_| CustomStyles::inset_container()),
                space::horizontal(),
                button(text(Translations::timeline_clear_btn(self.language)).size(11))
                    .style(|_, status| {
                        CustomStyles::storage_action_button(
                            crate::theme::StorageActionTone::Danger,
                            status,
                        )
                    })
                    .padding(Padding::from([5, 12]))
                    .on_press(Message::ClearTimeline),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding(10)
        .width(Length::Fill)
        .style(|_| CustomStyles::card_container());

        let mut events_col = Column::new().spacing(8).padding(Padding {
            top: 2.0,
            right: 0.0,
            bottom: 4.0,
            left: 0.0,
        });

        if self.network_events.is_empty() {
            events_col = events_col.push(
                container(
                    text(Translations::timeline_empty(self.language))
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(28)
                .align_x(Alignment::Center)
                .width(Length::Fill)
                .style(|_| CustomStyles::inset_container()),
            );
        } else {
            for evt in &self.network_events {
                let time_str = evt.timestamp.format("%H:%M:%S • %d/%m").to_string();
                let icon = evt.event_type.icon();
                let type_label = evt.event_type.label_localized(self.language);

                let evt_badge = container(text(icon).size(14))
                    .width(32)
                    .height(32)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_| CustomStyles::modal_icon_badge(CustomStyles::accent_purple()));

                let mut meta_row = row![
                    text(type_label)
                        .size(11)
                        .color(CustomStyles::accent_purple()),
                ]
                .align_y(Alignment::Center)
                .spacing(6);

                if let Some(ip) = evt.ip {
                    meta_row = meta_row.push(
                        container(
                            text(ip.to_string())
                                .size(9)
                                .color(CustomStyles::accent_cyan())
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink)
                        .padding(Padding::from([1, 6]))
                        .style(|_| {
                            CustomStyles::badge_pill(
                                Color::from_rgb8(15, 30, 45),
                                CustomStyles::accent_cyan(),
                            )
                        }),
                    );
                }

                meta_row = meta_row.push(space::horizontal()).push(
                    container(
                        text(time_str)
                            .size(10)
                            .color(CustomStyles::text_muted()),
                    )
                    .padding(Padding::from([2, 7]))
                    .style(|_| CustomStyles::inset_container()),
                );

                let evt_card = container(
                    row![
                        evt_badge,
                        column![
                            meta_row,
                            text(evt.title_localized(self.language))
                                .size(12)
                                .color(CustomStyles::text_primary()),
                            text(evt.details_localized(self.language))
                                .size(11)
                                .color(CustomStyles::text_secondary()),
                        ]
                        .spacing(2)
                        .width(Length::Fill),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                )
                .padding(10)
                .width(Length::Fill)
                .style(|_| CustomStyles::card_container());

                events_col = events_col.push(evt_card);
            }
        }

        let events_scrollable = container(
            scrollable(events_col)
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(5.0)
                        .scroller_width(5.0)
                        .margin(2.0)
                        .spacing(6.0),
                ))
                .style(CustomStyles::scrollbar_style())
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let content = column![
            title_row,
            summary_row,
            events_scrollable,
        ]
        .spacing(12);

        container(content)
            .padding(22)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_purple()))
            .into()
    }
}
