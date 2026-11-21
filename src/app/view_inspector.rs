use super::*;
use crate::i18n::Translations;
use crate::model::{Device, DeviceCategory};
use crate::theme::CustomStyles;
use crate::widget::CachedSparkline;
use iced::widget::canvas::Canvas;
use iced::widget::{
    button, column, container, pick_list, row, scrollable, space, text,
    text_input, tooltip, Column,
};
use iced::{alignment, Alignment, Color, Element, Length, Padding};

impl App {
    pub(super) fn view_inspector<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        // Fixed top bar of inspector (Title + Close)
        // Kept pinned to top so it is never hidden while scrolling
        let close_btn = self.tip(
            button(text("✕").size(13))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::CloseInspector),
            Translations::tip_close(self.language),
            tooltip::Position::Bottom,
        );

        let top_bar = row![
            text(format!("{} {}", device.category.icon(), Translations::device_details(self.language)))
                .size(15)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            close_btn,
        ]
        .padding(Padding {
            top: 0.0,
            right: 8.0,
            bottom: 0.0,
            left: 2.0,
        })
        .align_y(Alignment::Center);

        let mut insp_body = Column::new().spacing(11).padding(Padding {
            top: 2.0,
            right: 10.0,
            bottom: 10.0,
            left: 2.0,
        });

        // Name and localized subtitle
        insp_body = insp_body.push(
            column![
                text(device.display_name()).size(16).color(CustomStyles::text_primary()),
                text(Translations::device_subtitle(device, self.language)).size(11).color(CustomStyles::text_muted()),
            ]
            .spacing(1),
        );

        // Device type selector (Localized)
        let category_options: Vec<LocalizedCategory> = DeviceCategory::all()
            .iter()
            .map(|&c| LocalizedCategory {
                category: c,
                lang: self.language,
            })
            .collect();

        let selected_category = LocalizedCategory {
            category: device.category,
            lang: self.language,
        };

        let category_selector = row![
            text(Translations::device_type_label(self.language))
                .size(11)
                .color(CustomStyles::text_muted())
                .width(110),
            pick_list(
                category_options,
                Some(selected_category),
                move |loc| Message::ChangeDeviceCategory(device.ip, loc.category),
            ),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        insp_body = insp_body.push(category_selector);

        // Custom alias editor
        let save_alias_btn = self.tip(
            button(text(Translations::save(self.language)).size(11))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::SaveAlias(device.ip)),
            Translations::tip_save_alias(self.language),
            tooltip::Position::Top,
        );

        let alias_row = row![
            text_input(Translations::alias_placeholder(self.language), &self.alias_input)
                .on_input(Message::AliasInputChanged)
                .padding(Padding::from([5, 8]))
                .width(Length::Fill),
            save_alias_btn,
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        insp_body = insp_body.push(alias_row);

        // Technical specs (Fully localized)
        let mut specs_col = Column::new().spacing(5);

        specs_col = specs_col.push(Self::spec_line(Translations::spec_ip(self.language), &device.ip.to_string()));
        specs_col = specs_col.push(Self::spec_line(Translations::spec_mac(self.language), &device.mac));
        specs_col = specs_col.push(Self::spec_line(Translations::spec_vendor(self.language), &device.vendor));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_hostname(self.language),
            device.hostname.as_deref().unwrap_or(Translations::spec_not_assigned(self.language)),
        ));
        if let Some(model) = &device.model_name {
            specs_col = specs_col.push(Self::spec_line("Modelo / Dispositivo", model));
        }
        if !device.discovered_services.is_empty() {
            let services_str = device.discovered_services.join(", ");
            specs_col = specs_col.push(Self::spec_line("Servicios mDNS/SSDP", &services_str));
        }
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_mac_type(self.language),
            if device.is_randomized_mac {
                Translations::mac_type_private(self.language)
            } else {
                Translations::mac_type_global(self.language)
            },
        ));

        // Estimated Wi-Fi link / network quality
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_link_quality(self.language),
            Translations::link_quality_desc(device.latency_ms, self.language),
        ));

        // Linksys Mesh topology info if available
        if let Some(conn_type) = &device.connection_type {
            let label = if conn_type.contains("5GHz") || conn_type.contains("5 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                format!("{} {}", Translations::topology_wifi_5g(self.language), sig)
            } else if conn_type.contains("2.4GHz") || conn_type.contains("2.4 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                format!("{} {}", Translations::topology_wifi_24g(self.language), sig)
            } else {
                let mbps = device.negotiated_mbps.map(|m| format!(" ({} Mbps)", m)).unwrap_or_default();
                format!("{}{}", Translations::topology_ethernet(self.language), mbps)
            };
            specs_col = specs_col.push(Self::spec_line("Topología Router", &label));
        }

        specs_col = specs_col.push(Self::spec_line(Translations::spec_interface(self.language), &device.interface));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_status(self.language),
            Translations::status_label(device.status, self.language),
        ));
        specs_col = specs_col.push(Self::spec_line(
            Translations::spec_last_seen(self.language),
            &device.last_seen.format("%H:%M:%S").to_string(),
        ));

        insp_body = insp_body.push(
            container(specs_col)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        // Quick actions (Copy, Open in browser, WoL)
        let mut quick_actions = row![].spacing(6).align_y(Alignment::Center);

        quick_actions = quick_actions.push(
            self.tip(
                button(text("📋 IP").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::CopyToClipboard(device.ip.to_string(), "IP".to_string())),
                Translations::tip_copy_ip(self.language),
                tooltip::Position::Top,
            ),
        );

        quick_actions = quick_actions.push(
            self.tip(
                button(text("📋 MAC").size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::CopyToClipboard(device.mac.clone(), "MAC".to_string())),
                Translations::tip_copy_mac(self.language),
                tooltip::Position::Top,
            ),
        );

        let has_web = device.open_ports.iter().any(|p| p.port == 80 || p.port == 443 || p.port == 8080);
        if has_web || device.is_gateway {
            quick_actions = quick_actions.push(
                button(text(Translations::action_open_web(self.language)).size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::OpenInBrowser(device.ip)),
            );
        }

        if device.is_gateway || device.open_ports.iter().any(|p| p.port == 445 || p.port == 139) {
            quick_actions = quick_actions.push(
                self.tip(
                    button(text(Translations::storage_badge_available(self.language)).size(11))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 7]))
                        .on_press(Message::OpenStorageExplorer),
                    Translations::tip_router_storage(self.language),
                    tooltip::Position::Top,
                ),
            );
        }

        if !device.is_current_host {
            quick_actions = quick_actions.push(
                self.tip(
                    button(text("⏰ WoL").size(11))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 7]))
                        .on_press(Message::SendWol(device.mac.clone())),
                    Translations::tip_wol(self.language),
                    tooltip::Position::Top,
                ),
            );
        }

        insp_body = insp_body.push(quick_actions);

        // Security & Router Control Actions (Whitelist & Permanent Block)
        let mut security_actions = row![].spacing(6).align_y(Alignment::Center);

        let (wl_label, wl_style) = if device.is_whitelisted {
            (Translations::btn_remove_whitelist(self.language), CustomStyles::secondary_button())
        } else {
            (Translations::btn_add_whitelist(self.language), CustomStyles::primary_button())
        };
        security_actions = security_actions.push(
            self.tip(
                button(text(wl_label).size(11))
                    .style(move |_, _| wl_style.clone())
                    .padding(Padding::from([4, 7]))
                    .on_press(Message::ToggleDeviceWhitelist(device.ip, device.mac.clone())),
                Translations::tip_whitelist(self.language),
                tooltip::Position::Top,
            ),
        );

        if !device.is_current_host && !device.is_gateway {
            let is_this_device_operating = self.is_operating_device == Some(device.ip);
            let (block_label, block_style) = if is_this_device_operating {
                (Translations::btn_applying(self.language), CustomStyles::secondary_button())
            } else if device.is_blocked {
                (Translations::btn_unblock_router(self.language), CustomStyles::success_button())
            } else {
                (Translations::btn_block_router(self.language), CustomStyles::danger_button())
            };
            let mut block_btn = button(text(block_label).size(11))
                .style(move |_, _| block_style.clone())
                .padding(Padding::from([4, 7]));

            if !is_this_device_operating {
                block_btn = block_btn.on_press(Message::ToggleDeviceBlocked(device.ip, device.mac.clone()));
            }

            security_actions = security_actions.push(
                self.tip(block_btn, Translations::tip_block_router(self.language), tooltip::Position::Top),
            );
        }

        insp_body = insp_body.push(security_actions);

        // Tool 1: Real-time Ping Test (Localized)
        let ping_btn_text = if self.is_pinging {
            Translations::btn_measuring(self.language)
        } else {
            Translations::btn_ping_test(self.language)
        };

        let ping_btn = self.tip(
            button(text(ping_btn_text).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::StartPingTest(device.ip)),
            Translations::tip_ping(self.language),
            tooltip::Position::Top,
        );

        let mut ping_section = column![
            row![
                text(Translations::section_latency(self.language)).size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                ping_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if let Some(stats) = &self.inspector_ping_stats {
            let ping_sparkline = Canvas::new(CachedSparkline::new(
                &self.ping_history,
                CustomStyles::accent_cyan(),
                &self.ping_sparkline_cache,
            ))
            .width(Length::Fill)
            .height(20);

            ping_section = ping_section.push(
                column![
                    ping_sparkline,
                    row![
                        text(format!("{} {}", Translations::ping_sent(self.language), stats.transmitted)).size(11).color(CustomStyles::text_secondary()),
                        text(format!("{} {}", Translations::ping_received(self.language), stats.received)).size(11).color(CustomStyles::accent_emerald()),
                        text(format!("{} {:.0}%", Translations::ping_loss(self.language), stats.loss_percent)).size(11).color(if stats.loss_percent > 0.0 { CustomStyles::accent_amber() } else { CustomStyles::accent_emerald() }),
                    ]
                    .spacing(8),
                    row![
                        text(format!("{} {:.2} ms", Translations::ping_min(self.language), stats.min_ms)).size(11).color(CustomStyles::text_secondary()),
                        text(format!("{} {:.2} ms", Translations::ping_avg(self.language), stats.avg_ms)).size(11).color(CustomStyles::accent_cyan()),
                        text(format!("{} {:.2} ms", Translations::ping_max(self.language), stats.max_ms)).size(11).color(CustomStyles::text_secondary()),
                    ]
                    .spacing(8),
                ]
                .spacing(3),
            );
        }

        insp_body = insp_body.push(
            container(ping_section)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        // Tool 2: Port Scanner (Localized)
        let port_btn_text = if self.is_deep_scanning {
            Translations::btn_scanning_ports(self.language)
        } else {
            Translations::btn_scan_ports(self.language)
        };

        let port_btn = self.tip(
            button(text(port_btn_text).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::StartDeepScan(device.ip)),
            Translations::tip_port_scan(self.language),
            tooltip::Position::Top,
        );

        let mut ports_section = column![
            row![
                text(Translations::section_ports(self.language)).size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                port_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if self.deep_ports.is_empty() {
            ports_section = ports_section.push(
                text(if self.is_deep_scanning {
                    Translations::ports_checking(self.language)
                } else {
                    Translations::no_open_ports(self.language)
                })
                .size(11)
                .color(CustomStyles::text_muted()),
            );
        } else {
            let mut port_list = Column::new().spacing(4).padding(Padding {
                top: 2.0,
                right: 8.0,
                bottom: 2.0,
                left: 2.0,
            });
            for p in &self.deep_ports {
                port_list = port_list.push(
                    row![
                        container(
                            text(format!("TCP {}", p.port))
                                .size(9)
                                .color(CustomStyles::accent_cyan())
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink)
                        .padding(Padding::from([1, 5]))
                        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 50), CustomStyles::accent_cyan())),
                        text(&p.service).size(11).color(CustomStyles::text_primary()),
                        text(format!("({})", Translations::port_description(p.port, self.language))).size(10).color(CustomStyles::text_muted()),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                );
            }
            ports_section = ports_section.push(
                container(
                    scrollable(port_list)
                        .direction(scrollable::Direction::Vertical(
                            scrollable::Scrollbar::new()
                                .width(5.0)
                                .scroller_width(5.0)
                                .margin(1.0)
                                .spacing(6.0),
                        ))
                        .style(CustomStyles::scrollbar_style())
                        .height(Length::Shrink),
                )
                .padding(Padding::from([4, 6]))
                .width(Length::Fill)
                .max_height(120.0)
                .style(|_| CustomStyles::inset_container()),
            );
        }

        insp_body = insp_body.push(
            container(ports_section)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        // Tool 3: Route Diagnostics / Visual Traceroute
        let trace_btn_text = if self.is_tracing_route {
            Translations::traceroute_btn_tracing(self.language)
        } else {
            Translations::traceroute_btn_start(self.language)
        };

        let trace_btn = self.tip(
            button(text(trace_btn_text).size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([5, 8]))
                .on_press(Message::StartTraceroute(device.ip)),
            Translations::tip_traceroute(self.language),
            tooltip::Position::Top,
        );

        let mut trace_section = column![
            row![
                text(Translations::traceroute_title(self.language)).size(11).color(CustomStyles::text_muted()),
                space::horizontal(),
                trace_btn,
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(5);

        if let Some(err) = &self.traceroute_error {
            trace_section = trace_section.push(text(err).size(11).color(Color::from_rgb8(239, 68, 68)));
        } else if let Some(hops) = &self.traceroute_hops {
            let hops_header = row![
                text(Translations::traceroute_hop_col(self.language)).size(10).color(CustomStyles::text_muted()).width(36),
                text(Translations::traceroute_ip_col(self.language)).size(10).color(CustomStyles::text_muted()).width(190),
                space::horizontal(),
                text(Translations::traceroute_latency_col(self.language)).size(10).color(CustomStyles::text_muted()),
            ]
            .spacing(6);
            trace_section = trace_section.push(hops_header);

            let mut hops_col = Column::new().spacing(4).padding(Padding {
                top: 2.0,
                right: 8.0,
                bottom: 2.0,
                left: 2.0,
            });
            for h in hops {
                let (ip_str, latency_str, color) = if h.is_timeout {
                    ("---".to_string(), Translations::traceroute_timeout(self.language).to_string(), Color::from_rgb8(239, 68, 68))
                } else {
                    let lat = h.latency_ms.map(|ms| format!("{:.1} ms", ms)).unwrap_or_else(|| "-- ms".to_string());
                    let col = if h.latency_ms.map(|ms| ms > 50.0).unwrap_or(false) {
                        CustomStyles::accent_amber()
                    } else {
                        CustomStyles::accent_emerald()
                    };
                    let ip = h.ip.as_deref().unwrap_or("?");
                    let label = h.label.as_deref().map(|l| format!(" ({})", l)).unwrap_or_default();
                    (format!("{}{}", ip, label), lat, col)
                };

                hops_col = hops_col.push(
                    row![
                        container(
                            text(format!("#{}", h.hop))
                                .size(9)
                                .color(CustomStyles::accent_cyan())
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink)
                        .padding(Padding::from([1, 5]))
                        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 50), CustomStyles::accent_cyan())),
                        text(ip_str).size(10).color(CustomStyles::text_primary()).width(190),
                        space::horizontal(),
                        text(latency_str).size(10).color(color),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                );
            }
            trace_section = trace_section.push(
                container(
                    scrollable(hops_col)
                        .direction(scrollable::Direction::Vertical(
                            scrollable::Scrollbar::new()
                                .width(5.0)
                                .scroller_width(5.0)
                                .margin(1.0)
                                .spacing(6.0),
                        ))
                        .style(CustomStyles::scrollbar_style())
                        .height(Length::Shrink),
                )
                .padding(Padding::from([4, 6]))
                .width(Length::Fill)
                .max_height(130.0)
                .style(|_| CustomStyles::inset_container()),
            );
        }

        insp_body = insp_body.push(
            container(trace_section)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        // Tool 4: Device Requests & Domain History
        let req_count = device.request_history.len();
        let mut req_header = row![
            text(Translations::device_requests_title(self.language)).size(12).color(CustomStyles::accent_cyan()),
            container(
                text(format!("{}", req_count))
                    .size(9)
                    .color(CustomStyles::accent_cyan())
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center),
            )
            .center(Length::Shrink)
            .padding(Padding::from([1, 6]))
            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 30, 50), CustomStyles::accent_cyan())),
            space::horizontal(),
            button(text(Translations::device_requests_simulate_btn(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 6]))
                .on_press(Message::AddDeviceSampleRequest(device.ip)),
        ]
        .align_y(Alignment::Center)
        .spacing(6);

        if req_count > 0 {
            req_header = req_header.push(
                button(text("🗑️").size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 5]))
                    .on_press(Message::ClearDeviceRequestHistory(device.ip)),
            );
        }

        let mut req_section = column![req_header].spacing(6);

        if device.request_history.is_empty() {
            req_section = req_section.push(
                text(Translations::device_requests_empty(self.language))
                    .size(10)
                    .color(CustomStyles::text_muted()),
            );
        } else {
            let mut req_list = Column::new().spacing(5).padding(Padding {
                top: 2.0,
                right: 8.0,
                bottom: 2.0,
                left: 2.0,
            });

            for entry in &device.request_history {
                let time_str = entry.timestamp.format("%H:%M:%S").to_string();
                let cat_badge_color = if entry.category.contains("Streaming") {
                    CustomStyles::accent_purple()
                } else if entry.category.contains("Búsqueda") {
                    CustomStyles::accent_emerald()
                } else if entry.category.contains("Red") || entry.category.contains("DNS") {
                    CustomStyles::accent_cyan()
                } else {
                    CustomStyles::accent_amber()
                };

                let item = container(
                    column![
                        row![
                            text(time_str).size(9).color(CustomStyles::text_muted()),
                            container(
                                text(&entry.protocol)
                                    .size(8)
                                    .color(CustomStyles::accent_cyan())
                                    .align_x(alignment::Horizontal::Center)
                                    .align_y(alignment::Vertical::Center),
                            )
                            .center(Length::Shrink)
                            .padding(Padding::from([1, 5]))
                            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 25, 40), CustomStyles::accent_cyan())),
                            container(
                                text(&entry.category)
                                    .size(8)
                                    .color(cat_badge_color)
                                    .align_x(alignment::Horizontal::Center)
                                    .align_y(alignment::Vertical::Center),
                            )
                            .center(Length::Shrink)
                            .padding(Padding::from([1, 5]))
                            .style(move |_| CustomStyles::badge_pill(Color::from_rgb8(20, 30, 45), cat_badge_color)),
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center),
                        text(&entry.query).size(11).color(CustomStyles::text_primary()),
                    ]
                    .spacing(2),
                )
                .padding(6)
                .width(Length::Fill)
                .style(|_| CustomStyles::card_container());

                req_list = req_list.push(item);
            }

            req_section = req_section.push(
                container(
                    scrollable(req_list)
                        .direction(scrollable::Direction::Vertical(
                            scrollable::Scrollbar::new()
                                .width(5.0)
                                .scroller_width(5.0)
                                .margin(1.0)
                                .spacing(6.0),
                        ))
                        .style(CustomStyles::scrollbar_style())
                        .height(Length::Shrink),
                )
                .padding(Padding::from([4, 6]))
                .width(Length::Fill)
                .max_height(170.0)
                .style(|_| CustomStyles::inset_container()),
            );
        }

        insp_body = insp_body.push(
            container(req_section)
                .padding(10)
                .style(|_| CustomStyles::card_container()),
        );

        let body_scrollable = scrollable(insp_body)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(8.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fill);

        column![
            top_bar,
            body_scrollable,
        ]
        .spacing(12)
        .height(Length::Fill)
        .into()
    }

}
