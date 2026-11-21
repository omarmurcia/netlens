use super::*;
use crate::i18n::Translations;
use crate::telemetry::Telemetry;
use crate::theme::CustomStyles;
use iced::widget::{
    button, column, container, row, scrollable, space, text, tooltip, Column,
};
use iced::{alignment, Alignment, Color, Element, Length, Padding};

impl App {
    /// Modal: Wi-Fi Channel & RF Spectrum Analyzer
    pub(super) fn view_wifi_analyzer_modal(&self) -> Element<'_, Message> {
        let accent = CustomStyles::accent_amber();
        let header_badge = container(text("📶").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::wifi_modal_title(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::wifi_modal_desc(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseWifiAnalyzer),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let scan_btn_label = Translations::wifi_scan_btn(self.is_scanning_wifi, self.language);

        let current_conn_info = if let Some(report) = &self.wifi_analyzer_report {
            Translations::wifi_current_network(
                report.current_ssid.as_deref(),
                report.current_channel,
                self.language,
            )
        } else {
            Translations::wifi_current_network_none(self.language)
        };

        let action_bar = container(
            row![
                text("📡").size(14),
                text(current_conn_info)
                    .size(12)
                    .color(CustomStyles::accent_cyan()),
                space::horizontal(),
                button(text(scan_btn_label).size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([6, 16]))
                    .on_press(Message::ScanWifi),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([8, 12]))
        .width(Length::Fill)
        .style(|_| CustomStyles::card_highlighted());

        let mut modal_body = Column::new().spacing(10).padding(Padding {
            top: 2.0,
            right: 0.0,
            bottom: 6.0,
            left: 0.0,
        });

        if let Some(report) = &self.wifi_analyzer_report {
            // Recommendation Card
            let reco_card = container(
                row![
                    column![
                        text(Translations::wifi_optimal_channels_title(self.language))
                            .size(13)
                            .color(CustomStyles::accent_emerald()),
                        text(Translations::wifi_recommendation_text(
                            report.current_channel,
                            report.recommended_channel_24,
                            report.recommended_channel_5,
                            self.language
                        ))
                        .size(11)
                        .color(CustomStyles::text_secondary()),
                    ]
                    .spacing(3)
                    .width(Length::Fill),
                    container(
                        text(Translations::wifi_channel_24_label(
                            report.recommended_channel_24,
                            self.language
                        ))
                        .size(11)
                        .color(CustomStyles::accent_emerald()),
                    )
                    .padding(Padding::from([5, 10]))
                    .style(|_| CustomStyles::tinted_badge_box(CustomStyles::accent_emerald())),
                    container(
                        text(Translations::wifi_channel_5_label(
                            report.recommended_channel_5,
                            self.language
                        ))
                        .size(11)
                        .color(CustomStyles::accent_cyan()),
                    )
                    .padding(Padding::from([5, 10]))
                    .style(|_| CustomStyles::tinted_badge_box(CustomStyles::accent_cyan())),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .padding(12)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_download());

            modal_body = modal_body.push(reco_card);

            // 2.4 GHz Channel Summary
            let mut ch24_row = row![].spacing(6);
            for ch in &report.channels_24 {
                let is_optimal = ch.channel == report.recommended_channel_24;
                let tile_accent = if is_optimal {
                    CustomStyles::accent_emerald()
                } else if ch.networks_count == 0 {
                    CustomStyles::accent_cyan()
                } else {
                    CustomStyles::accent_amber()
                };

                let ch_card = container(
                    column![
                        text(Translations::wifi_channel_box_title(
                            ch.channel,
                            self.language
                        ))
                        .size(11)
                        .color(tile_accent),
                        text(Translations::wifi_networks_count(
                            ch.networks_count,
                            self.language
                        ))
                        .size(13)
                        .color(CustomStyles::text_primary()),
                        text(Translations::wifi_max_signal(ch.max_signal, self.language))
                            .size(9)
                            .color(CustomStyles::text_muted()),
                    ]
                    .spacing(2),
                )
                .padding(8)
                .width(Length::FillPortion(1))
                .style(move |_| {
                    if is_optimal {
                        CustomStyles::tinted_badge_box(CustomStyles::accent_emerald())
                    } else {
                        CustomStyles::inset_container()
                    }
                });

                ch24_row = ch24_row.push(ch_card);
            }

            modal_body = modal_body.push(
                container(
                    column![
                        text(Translations::wifi_occupancy_24_title(self.language))
                            .size(12)
                            .color(CustomStyles::accent_amber()),
                        ch24_row,
                    ]
                    .spacing(8),
                )
                .padding(12)
                .width(Length::Fill)
                .style(|_| CustomStyles::card_container()),
            );

            // Detected Wi-Fi Networks Table-style Card
            let mut nets_col = column![
                text(Translations::wifi_detected_networks_header(
                    report.networks.len(),
                    self.language
                ))
                .size(12)
                .color(CustomStyles::accent_cyan()),
            ]
            .spacing(6);

            for (idx, net) in report.networks.iter().enumerate() {
                let freq_label = if net.frequency_mhz >= 5000 {
                    "5 GHz"
                } else {
                    "2.4 GHz"
                };
                let is_curr = net.is_current;
                let sig_color = if net.signal_percent >= 70 {
                    CustomStyles::accent_emerald()
                } else if net.signal_percent >= 45 {
                    CustomStyles::accent_amber()
                } else {
                    Color::from_rgb8(239, 68, 68)
                };

                let mut row_items = row![
                    container(text("📶").size(13))
                        .width(28)
                        .height(28)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(move |_| CustomStyles::modal_icon_badge(sig_color)),
                    column![
                        text(&net.ssid).size(12).color(if is_curr {
                            CustomStyles::accent_emerald()
                        } else {
                            CustomStyles::text_primary()
                        }),
                        text(format!("BSSID: {}  •  Seg: {}", net.bssid, net.security))
                            .size(10)
                            .color(CustomStyles::text_muted()),
                    ]
                    .spacing(1)
                    .width(Length::Fill),
                ]
                .align_y(Alignment::Center)
                .spacing(10);

                if is_curr {
                    row_items = row_items.push(
                        container(
                            text(Translations::wifi_badge_current(self.language))
                                .size(9)
                                .color(CustomStyles::accent_emerald())
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink)
                        .padding(Padding::from([2, 7]))
                        .style(|_| {
                            CustomStyles::badge_pill(
                                Color::from_rgb8(15, 45, 30),
                                CustomStyles::accent_emerald(),
                            )
                        }),
                    );
                }

                row_items = row_items
                    .push(
                        container(
                            text(format!("Ch {} ({})", net.channel, freq_label))
                                .size(10)
                                .color(CustomStyles::accent_cyan())
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink)
                        .padding(Padding::from([2, 8]))
                        .style(|_| {
                            CustomStyles::badge_pill(
                                Color::from_rgb8(15, 30, 45),
                                CustomStyles::accent_cyan(),
                            )
                        }),
                    )
                    .push(
                        container(
                            text(format!("{}%", net.signal_percent))
                                .size(11)
                                .color(sig_color),
                        )
                        .width(48)
                        .align_x(Alignment::End),
                    );

                let row_item = container(row_items)
                    .padding(Padding::from([6, 10]))
                    .width(Length::Fill)
                    .style(move |_| CustomStyles::storage_file_row(idx));

                nets_col = nets_col.push(row_item);
            }

            modal_body = modal_body.push(
                container(nets_col)
                    .padding(12)
                    .width(Length::Fill)
                    .style(|_| CustomStyles::card_container()),
            );
        } else {
            modal_body = modal_body.push(
                container(
                    text(Translations::wifi_prompt_scan(self.language))
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(28)
                .align_x(Alignment::Center)
                .width(Length::Fill)
                .style(|_| CustomStyles::inset_container()),
            );
        }

        let scrollable_area = container(
            scrollable(modal_body)
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
            action_bar,
            scrollable_area,
        ]
        .spacing(12);

        container(content)
            .padding(22)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_amber()))
            .into()
    }

    /// Modal: Universal Router UPnP / IGD & Port Forwarding
    pub(super) fn view_upnp_modal(&self) -> Element<'_, Message> {
        let accent = CustomStyles::accent_cyan();
        let header_badge = container(text("🌐").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::upnp_modal_title(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::upnp_modal_desc(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseUpnpModal),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let gw_ip_str = self
            .network
            .gateway_ip
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| Translations::not_detected(self.language).to_string());

        let refresh_btn = button(text(Translations::upnp_query_btn(false, self.language)).size(12))
            .style(|_, status| {
                CustomStyles::storage_action_button(
                    crate::theme::StorageActionTone::Primary,
                    status,
                )
            })
            .padding(Padding::from([5, 14]))
            .on_press(Message::QueryUpnpRouter);

        let action_row = container(
            row![
                text(format!("{}: {}", Translations::gateway_label(self.language), gw_ip_str))
                    .size(12)
                    .color(CustomStyles::text_secondary()),
                space::horizontal(),
                refresh_btn,
            ]
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([8, 12]))
        .width(Length::Fill)
        .style(|_| CustomStyles::card_highlighted());

        let mut modal_body = Column::new().spacing(10).padding(Padding {
            top: 4.0,
            right: 12.0,
            bottom: 8.0,
            left: 4.0,
        });

        if let Some(upnp) = &self.upnp_status {
            let wan_ip = upnp.external_ip.as_deref().unwrap_or(Translations::upnp_not_assigned(self.language));
            let conn_status = upnp.connection_status.as_deref().unwrap_or(Translations::upnp_status_connected(self.language));
            let uptime_str = upnp
                .uptime_secs
                .map(|s| format!("{}h {}m", s / 3600, (s % 3600) / 60))
                .unwrap_or_else(|| "-".to_string());
            let rx_bytes = upnp.total_bytes_received.map(Telemetry::format_bytes).unwrap_or_else(|| "-".to_string());
            let tx_bytes = upnp.total_bytes_sent.map(Telemetry::format_bytes).unwrap_or_else(|| "-".to_string());

            let stat_card_ip = container(
                column![
                    text(Translations::upnp_public_ip_title(self.language)).size(10).color(CustomStyles::accent_cyan()),
                    text(wan_ip).size(16).color(CustomStyles::accent_cyan()),
                    text(Translations::upnp_status_label(conn_status, self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .spacing(3),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::card_container());

            let stat_card_uptime = container(
                column![
                    text(Translations::upnp_uptime_title(self.language)).size(10).color(CustomStyles::accent_emerald()),
                    text(uptime_str).size(16).color(CustomStyles::accent_emerald()),
                    text(Translations::upnp_uptime_sub(self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .spacing(3),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::card_container());

            let stat_card_traffic = container(
                column![
                    text(Translations::upnp_traffic_title(self.language)).size(10).color(CustomStyles::accent_purple()),
                    text(format!("↓ {} | ↑ {}", rx_bytes, tx_bytes)).size(13).color(CustomStyles::text_primary()),
                    text(Translations::upnp_traffic_sub(self.language)).size(10).color(CustomStyles::text_muted()),
                ]
                .spacing(3),
            )
            .padding(10)
            .width(Length::FillPortion(1))
            .style(|_| CustomStyles::card_container());

            let stats_grid = row![stat_card_ip, stat_card_uptime, stat_card_traffic].spacing(8);
            modal_body = modal_body.push(stats_grid);

            // Port Forwarding
            modal_body = modal_body.push(
                text(Translations::upnp_port_mappings_header(upnp.port_mappings.len(), self.language))
                    .size(13)
                    .color(CustomStyles::accent_cyan()),
            );

            if upnp.port_mappings.is_empty() {
                modal_body = modal_body.push(
                    container(
                        text(Translations::upnp_no_mappings(self.language))
                            .size(11)
                            .color(CustomStyles::text_muted()),
                    )
                    .padding(14)
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                );
            } else {
                for m in &upnp.port_mappings {
                    let is_critical = [22, 23, 80, 445, 3389, 5900].contains(&m.external_port);

                    let port_rule_card = container(
                        row![
                            column![
                                row![
                                    text(Translations::upnp_port_label(m.external_port, &m.protocol, self.language))
                                        .size(13)
                                        .color(if is_critical { Color::from_rgb8(239, 68, 68) } else { CustomStyles::accent_cyan() }),
                                    text("➜").size(11).color(CustomStyles::text_muted()),
                                    text(format!("{}:{}", m.internal_client, m.internal_port))
                                        .size(12)
                                        .color(CustomStyles::text_primary()),
                                ]
                                .spacing(6)
                                .align_y(Alignment::Center),
                                text(if m.description.is_empty() { "UPnP Mapped Port" } else { &m.description })
                                    .size(10)
                                    .color(CustomStyles::text_muted()),
                            ]
                            .spacing(2)
                            .width(Length::Fill),
                            if is_critical {
                                container(
                                    text(Translations::upnp_badge_wan_risk(self.language))
                                        .size(9)
                                        .color(Color::WHITE)
                                        .align_x(alignment::Horizontal::Center)
                                        .align_y(alignment::Vertical::Center),
                                )
                                .center(Length::Shrink)
                                .padding(Padding::from([2, 6]))
                                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(185, 28, 28), Color::WHITE))
                            } else {
                                container(
                                    text(if m.enabled { Translations::upnp_badge_active(self.language) } else { Translations::upnp_badge_inactive(self.language) })
                                        .size(9)
                                        .color(CustomStyles::accent_emerald())
                                        .align_x(alignment::Horizontal::Center)
                                        .align_y(alignment::Vertical::Center),
                                )
                                .center(Length::Shrink)
                                .padding(Padding::from([2, 6]))
                                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 45, 30), CustomStyles::accent_emerald()))
                            },
                        ]
                        .align_y(Alignment::Center)
                        .padding(8),
                    )
                    .width(Length::Fill)
                    .style(move |_| {
                        if is_critical {
                            CustomStyles::card_danger()
                        } else {
                            CustomStyles::card_container()
                        }
                    });

                    modal_body = modal_body.push(port_rule_card);
                }
            }
        } else {
            modal_body = modal_body.push(
                container(
                    text(Translations::upnp_prompt(self.language))
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(20)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            );
        }

        let scrollable_area = container(
            scrollable(modal_body)
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
            action_row,
            scrollable_area,
        ]
        .spacing(12);

        container(content)
            .padding(22)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }
}
