use super::*;
use crate::i18n::Translations;
use crate::model::Device;
use crate::telemetry::Telemetry;
use crate::theme::CustomStyles;
use crate::widget::CachedSparkline;
use iced::widget::canvas::Canvas;
use iced::widget::{
    button, column, container, pick_list, row, scrollable, space, text,
    text_input, tooltip, Column,
};
use iced::{alignment, Alignment, Color, Element, Length, Padding};

impl App {
    pub(super) fn view_telemetry_dashboard(&self) -> Element<'_, Message> {
        // Card 1: Download Speed (with Sparkline)
        let rx_speed_str = Telemetry::format_speed(self.telemetry.download_speed_bps);
        let rx_session_str = Telemetry::format_bytes(self.telemetry.session_download_bytes());

        let sparkline_dl = Canvas::new(CachedSparkline::new(
            &self.download_history,
            CustomStyles::accent_emerald(),
            &self.download_sparkline_cache,
        ))
        .width(Length::Fill)
        .height(20);

        let card_download = container(
            column![
                row![
                    text(Translations::download(self.language)).size(11).color(CustomStyles::accent_emerald()),
                    space::horizontal(),
                    text(Translations::live(self.language)).size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(rx_speed_str).size(20).color(CustomStyles::accent_emerald()),
                sparkline_dl,
                text(format!("{}: {}", Translations::session(self.language), rx_session_str)).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_download());

        // Card 2: Upload Speed (with Sparkline)
        let tx_speed_str = Telemetry::format_speed(self.telemetry.upload_speed_bps);
        let tx_session_str = Telemetry::format_bytes(self.telemetry.session_upload_bytes());

        let sparkline_ul = Canvas::new(CachedSparkline::new(
            &self.upload_history,
            CustomStyles::accent_purple(),
            &self.upload_sparkline_cache,
        ))
        .width(Length::Fill)
        .height(20);

        let card_upload = container(
            column![
                row![
                    text(Translations::upload(self.language)).size(11).color(CustomStyles::accent_purple()),
                    space::horizontal(),
                    text(Translations::live(self.language)).size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(tx_speed_str).size(20).color(CustomStyles::accent_purple()),
                sparkline_ul,
                text(format!("{}: {}", Translations::session(self.language), tx_session_str)).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_upload());

        // Card 3: Wi-Fi Signal Strength & Quality / Network Connection
        let (signal_title, signal_big_str, signal_sub_str) = if let Some(wifi) = &self.telemetry.wifi {
            let dbm_part = wifi.signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
            let big = format!("{}%{}", wifi.signal_percent, dbm_part);
            let sub = format!(
                "{} • {} • {}",
                wifi.ssid.as_deref().unwrap_or("Wi-Fi"),
                wifi.band_label(),
                Translations::wifi_quality_label(wifi.signal_percent, self.language)
            );
            (Translations::signal_wifi(self.language), big, sub)
        } else if let Some(eth) = &self.telemetry.ethernet {
            let big = eth.display_summary();
            let sub = format!("{} ({})", eth.interface, Translations::ethernet_cable(self.language));
            (Translations::ethernet_cable(self.language), big, sub)
        } else {
            (
                "🌐 LINK",
                format!("{} {}", self.network.interface, Translations::spec_interface(self.language)),
                Translations::link_quality_desc(None, self.language).to_string(),
            )
        };

        let card_signal = container(
            column![
                row![
                    text(signal_title).size(11).color(CustomStyles::accent_amber()),
                    space::horizontal(),
                    text("STATUS").size(9).color(CustomStyles::text_muted()),
                ]
                .align_y(Alignment::Center),
                text(signal_big_str).size(18).color(CustomStyles::accent_amber()),
                text(signal_sub_str).size(11).color(CustomStyles::text_secondary()),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_signal());

        // Card 4: Router / Gateway & Local Network
        let total_devices = self.devices.len();
        let gw_ip_str = self
            .network
            .gateway_ip
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| Translations::not_detected(self.language).to_string());

        let gw_device = self.devices.iter().find(|d| d.is_gateway);
        let gw_name = gw_device
            .map(|d| d.display_name())
            .unwrap_or_else(|| Translations::gateway_label(self.language).to_string());

        let wan_status_text = if let Some(upnp) = &self.upnp_status {
            if let Some(ext_ip) = &upnp.external_ip {
                format!("WAN: {}", ext_ip)
            } else {
                "UPnP Activo".to_string()
            }
        } else {
            format!("Estabilidad: {:.0}%", self.uptime_tracker.uptime_percentage())
        };

        let card_gateway = container(
            column![
                row![
                    text(Translations::network_and_router(self.language)).size(11).color(CustomStyles::accent_cyan()),
                    space::horizontal(),
                    button(text(Translations::upnp_btn(self.language)).size(9))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([1, 6]))
                        .on_press(Message::OpenUpnpModal),
                ]
                .align_y(Alignment::Center),
                row![
                    text(Translations::devices_count(total_devices, self.language)).size(18).color(CustomStyles::text_primary()),
                    text(format!("(GW: {})", gw_ip_str)).size(12).color(CustomStyles::accent_cyan()),
                ]
                .align_y(Alignment::Center)
                .spacing(6),
                row![
                    text(gw_name).size(11).color(CustomStyles::text_secondary()),
                    space::horizontal(),
                    text(wan_status_text).size(10).color(CustomStyles::accent_emerald()),
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(3),
        )
        .padding(12)
        .width(Length::FillPortion(1))
        .style(|_| CustomStyles::card_highlighted());

        row![card_download, card_upload, card_signal, card_gateway]
            .spacing(10)
            .into()
    }

    /// Filter, search, and sorting controls
    pub(super) fn view_controls(&self) -> Element<'_, Message> {
        let search_bar = text_input(
            Translations::search_placeholder(self.language),
            &self.search_query,
        )
        .on_input(Message::SearchChanged)
        .size(13)
        .padding(Padding::from([8, 12]))
        .width(Length::Fixed(300.0));

        // Category compact styled pill buttons in an elegant scrollable horizontal row
        let mut category_pills = row![].spacing(6).align_y(Alignment::Center).padding(Padding {
            top: 4.0,
            right: 6.0,
            bottom: 6.0,
            left: 2.0,
        });

        let is_all_active = self.selected_category.is_none() && self.security_filter == SecurityFilter::All;
        let all_btn = button(
            container(
                text(Translations::all_filter(self.language))
                    .size(12)
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center),
            )
            .center(Length::Shrink),
        )
        .style(move |_, _| {
            if is_all_active {
                CustomStyles::filter_pill_active()
            } else {
                CustomStyles::filter_pill_inactive()
            }
        })
        .padding(Padding::from([4, 10]))
        .on_press(Message::CategorySelected(None));
        category_pills = category_pills.push(
            self.tip(all_btn, Translations::tip_filter_all(self.language), tooltip::Position::Bottom)
        );

        // Only display category pills for device types currently detected in the network (cached)
        for &cat in &self.cached_visible_categories {
            let is_active = self.selected_category == Some(cat);
            let label = Translations::category_short_name(cat, self.language);
            let cat_btn = button(
                container(
                    text(label)
                        .size(12)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink),
            )
            .style(move |_, _| {
                if is_active {
                    CustomStyles::filter_pill_active()
                } else {
                    CustomStyles::filter_pill_inactive()
                }
            })
            .padding(Padding::from([4, 10]))
            .on_press(Message::CategorySelected(Some(cat)));
            category_pills = category_pills.push(
                self.tip(cat_btn, Translations::tip_filter_category(cat, self.language), tooltip::Position::Bottom)
            );
        }

        // Security & List Filters (Whitelist and Blocked) from cached counts
        let whitelist_count = self.cached_whitelist_count;
        let blocked_count = self.cached_blocked_count;

        let is_whitelist_active = self.security_filter == SecurityFilter::Whitelist;
        let is_blocked_active = self.security_filter == SecurityFilter::Blocked;

        if whitelist_count > 0 || is_whitelist_active {
            let wl_btn = button(
                container(
                    text(format!("🛡️ {} ({})", Translations::filter_whitelist(self.language), whitelist_count))
                        .size(11)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink),
            )
            .style(move |_, _| {
                if is_whitelist_active {
                    CustomStyles::filter_pill_active()
                } else {
                    CustomStyles::filter_pill_inactive()
                }
            })
            .padding(Padding::from([4, 10]))
            .on_press(Message::SecurityFilterChanged(
                if is_whitelist_active { SecurityFilter::All } else { SecurityFilter::Whitelist }
            ));
            category_pills = category_pills.push(
                self.tip(wl_btn, Translations::tip_filter_whitelist(self.language), tooltip::Position::Bottom)
            );
        }

        if blocked_count > 0 || is_blocked_active {
            let blk_btn = button(
                container(
                    text(format!("🚫 {} ({})", Translations::filter_blocked(self.language), blocked_count))
                        .size(11)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink),
            )
            .style(move |_, _| {
                if is_blocked_active {
                    CustomStyles::filter_pill_active()
                } else {
                    CustomStyles::filter_pill_inactive()
                }
            })
            .padding(Padding::from([4, 10]))
            .on_press(Message::SecurityFilterChanged(
                if is_blocked_active { SecurityFilter::All } else { SecurityFilter::Blocked }
            ));
            category_pills = category_pills.push(
                self.tip(blk_btn, Translations::tip_filter_blocked(self.language), tooltip::Position::Bottom)
            );
        }

        // Localized reactive sort selector
        let sort_options: Vec<LocalizedSortOption> = SortOption::ALL
            .iter()
            .map(|&s| LocalizedSortOption {
                sort: s,
                lang: self.language,
            })
            .collect();

        let selected_sort = LocalizedSortOption {
            sort: self.sort_by,
            lang: self.language,
        };

        let sort_picker = pick_list(
            sort_options,
            Some(selected_sort),
            |loc| Message::SortChanged(loc.sort),
        );

        let category_scroll = scrollable(category_pills)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new()
                    .width(3.0)
                    .margin(1.0)
                    .scroller_width(3.0)
                    .spacing(4.0),
            ))
            .style(CustomStyles::discrete_horizontal_scrollbar_style())
            .width(Length::Fill);

        row![
            search_bar,
            category_scroll,
            sort_picker,
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
    }

    /// Generates scrollable device cards list
    pub(super) fn view_device_list<'a>(&'a self, devices: &[&'a Device]) -> Element<'a, Message> {
        if devices.is_empty() {
            let empty_text = if self.is_scanning && !self.is_background_scan {
                Translations::empty_scanning(self.language)
            } else if !self.search_query.is_empty() || self.selected_category.is_some() {
                Translations::empty_filtered(self.language)
            } else {
                Translations::empty_initial(self.language)
            };

            return container(
                column![
                    text("🔍").size(40),
                    text(empty_text).size(16).color(CustomStyles::text_muted()),
                ]
                .spacing(12)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into();
        }

        let mut list_col = Column::new().spacing(8).padding(Padding {
            top: 2.0,
            right: 0.0,
            bottom: 10.0,
            left: 0.0,
        });

        for device in devices {
            list_col = list_col.push(self.view_device_card(device));
        }

        scrollable(list_col)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(5.0)
                    .scroller_width(5.0)
                    .margin(2.0)
                    .spacing(6.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fill)
            .into()
    }

    /// Individual device card with responsive 2-level layout (no overflow or overlap)
    pub(super) fn view_device_card<'a>(&'a self, device: &'a Device) -> Element<'a, Message> {
        let is_selected = self.selected_device_ip == Some(device.ip);

        // Category icon
        let icon_widget = text(device.category.icon()).size(24);

        // Device name
        let name_widget = text(device.display_name())
            .size(14)
            .color(if device.is_gateway || device.is_current_host {
                CustomStyles::accent_cyan()
            } else {
                CustomStyles::text_primary()
            });

        // Special role badges
        let mut role_tags = row![].spacing(4).align_y(Alignment::Center);

        if device.is_blocked {
            role_tags = role_tags.push(
                container(
                    text(Translations::badge_blocked(self.language))
                        .size(9)
                        .color(Color::WHITE)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([1, 6]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(185, 28, 28), Color::WHITE)),
            );
        } else if device.is_whitelisted {
            role_tags = role_tags.push(
                container(
                    text(Translations::badge_whitelisted(self.language))
                        .size(9)
                        .color(Color::WHITE)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([1, 6]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(16, 120, 70), Color::WHITE)),
            );
        }

        if device.is_gateway {
            role_tags = role_tags.push(
                container(
                    text(Translations::badge_router(self.language))
                        .size(9)
                        .color(Color::WHITE)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([1, 6]))
                .style(|_| CustomStyles::badge_pill(CustomStyles::accent_cyan(), Color::WHITE)),
            );
        }
        if device.is_current_host {
            role_tags = role_tags.push(
                container(
                    text(Translations::badge_local_host(self.language))
                        .size(9)
                        .color(Color::WHITE)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([1, 6]))
                .style(|_| CustomStyles::badge_pill(CustomStyles::accent_purple(), Color::WHITE)),
            );
        }
        if device.is_randomized_mac {
            role_tags = role_tags.push(
                container(
                    text(Translations::badge_privacy(self.language))
                        .size(9)
                        .color(CustomStyles::accent_amber())
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([1, 6]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(69, 45, 12), CustomStyles::accent_amber())),
            );
        }

        // Latency & Connection Quality
        let (quality_label, quality_color, lat_str) = if let Some(lat) = device.latency_ms {
            if lat < 1.0 {
                (Translations::latency_cable(self.language), CustomStyles::accent_cyan(), format!("{:.1} ms", lat))
            } else if lat < 20.0 {
                (Translations::latency_excellent(self.language), CustomStyles::accent_emerald(), format!("{:.1} ms", lat))
            } else if lat < 70.0 {
                (Translations::latency_good(self.language), CustomStyles::accent_emerald(), format!("{:.1} ms", lat))
            } else if lat < 140.0 {
                (Translations::latency_fair(self.language), CustomStyles::accent_amber(), format!("{:.1} ms", lat))
            } else {
                (Translations::latency_weak(self.language), Color::from_rgb8(239, 68, 68), format!("{:.1} ms", lat))
            }
        } else {
            (Translations::latency_no_response(self.language), CustomStyles::text_muted(), "-".to_string())
        };

        let latency_widget = container(
            row![
                text("🟢").size(8).align_y(alignment::Vertical::Center),
                text(lat_str).size(11).color(quality_color).align_y(alignment::Vertical::Center),
                text(format!("({})", quality_label)).size(10).color(CustomStyles::text_muted()).align_y(alignment::Vertical::Center),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .center(Length::Shrink)
        .padding(Padding::from([2, 6]))
        .style(move |_| CustomStyles::badge_pill(Color::from_rgb8(18, 25, 38), quality_color));

        // Details button
        let btn_details = self.tip(
            button(
                container(
                    text(Translations::details_btn(self.language))
                        .size(11)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink),
            )
            .style(move |_, _| {
                if is_selected {
                    CustomStyles::primary_button()
                } else {
                    CustomStyles::secondary_button()
                }
            })
            .padding(Padding::from([4, 10]))
            .on_press(Message::SelectDevice(device.ip)),
            Translations::tip_device_details(self.language),
            tooltip::Position::Top,
        );

        // Row 1 (Top): Icon + Name + Role badges + Spacer + Latency + Details button
        let top_row = row![
            icon_widget,
            name_widget,
            role_tags,
            space::horizontal(),
            latency_widget,
            btn_details,
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        // Row 2 (Bottom): Metadata in clean pill layout
        // 1. IP with copy button
        let ip_pill = container(
            row![
                text("IP: ").size(10).color(CustomStyles::text_muted()).align_y(alignment::Vertical::Center),
                text(device.ip.to_string()).size(11).color(CustomStyles::text_primary()).align_y(alignment::Vertical::Center),
                self.tip(
                    button(
                        container(
                            text("📋")
                                .size(9)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink),
                    )
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 4]))
                    .on_press(Message::CopyToClipboard(device.ip.to_string(), "IP".to_string())),
                    Translations::tip_copy_ip(self.language),
                    tooltip::Position::Top,
                ),
            ]
            .spacing(3)
            .align_y(Alignment::Center),
        )
        .center(Length::Shrink)
        .padding(Padding::from([2, 7]))
        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 25, 40), CustomStyles::text_primary()));

        // 2. MAC with copy button
        let mac_pill = container(
            row![
                text("MAC: ").size(10).color(CustomStyles::text_muted()).align_y(alignment::Vertical::Center),
                text(&device.mac).size(10).color(CustomStyles::text_secondary()).align_y(alignment::Vertical::Center),
                self.tip(
                    button(
                        container(
                            text("📋")
                                .size(9)
                                .align_x(alignment::Horizontal::Center)
                                .align_y(alignment::Vertical::Center),
                        )
                        .center(Length::Shrink),
                    )
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([1, 4]))
                    .on_press(Message::CopyToClipboard(device.mac.clone(), "MAC".to_string())),
                    Translations::tip_copy_mac(self.language),
                    tooltip::Position::Top,
                ),
            ]
            .spacing(3)
            .align_y(Alignment::Center),
        )
        .center(Length::Shrink)
        .padding(Padding::from([2, 7]))
        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 25, 40), CustomStyles::text_secondary()));

        // 3. Category
        let cat_label = Translations::category_name(device.category, self.language)
            .split('/')
            .next()
            .unwrap_or("")
            .trim();
        let cat_pill = container(
            text(cat_label)
                .size(10)
                .color(CustomStyles::accent_cyan())
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center),
        )
        .center(Length::Shrink)
        .padding(Padding::from([2, 7]))
        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 35, 55), CustomStyles::accent_cyan()));

        // 4. Vendor (UTF-8 char-safe truncation)
        let vendor_str = if !device.vendor.is_empty() && device.vendor != "Desconocido" {
            device.vendor.as_str()
        } else if device.is_randomized_mac {
            Translations::vendor_private(self.language)
        } else {
            Translations::vendor_generic(self.language)
        };
        let vendor_truncated = if vendor_str.chars().count() > 20 {
            let head: String = vendor_str.chars().take(18).collect();
            format!("{}...", head)
        } else {
            vendor_str.to_string()
        };
        let vendor_pill = container(
            text(vendor_truncated)
                .size(10)
                .color(CustomStyles::text_secondary())
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center),
        )
        .center(Length::Shrink)
        .padding(Padding::from([2, 7]))
        .style(|_| CustomStyles::badge_pill(Color::from_rgb8(25, 30, 45), CustomStyles::text_secondary()));

        let mut bottom_row = row![
            ip_pill,
            mac_pill,
            cat_pill,
            vendor_pill,
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        // 5. Router topology (if available)
        if let Some(conn_type) = &device.connection_type {
            let (top_label, top_bg, top_color) = if conn_type.contains("5GHz") || conn_type.contains("5 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                (format!("📶 5GHz{}", sig), Color::from_rgb8(15, 45, 40), CustomStyles::accent_emerald())
            } else if conn_type.contains("2.4GHz") || conn_type.contains("2.4 GHz") {
                let sig = device.wifi_signal_dbm.map(|s| format!(" ({} dBm)", s)).unwrap_or_default();
                (format!("📶 2.4GHz{}", sig), Color::from_rgb8(45, 35, 15), CustomStyles::accent_amber())
            } else {
                let mbps = device.negotiated_mbps.map(|m| format!(" {}M", m)).unwrap_or_default();
                (format!("🔌 LAN{}", mbps), Color::from_rgb8(25, 30, 50), CustomStyles::accent_cyan())
            };

            bottom_row = bottom_row.push(
                container(
                    text(top_label)
                        .size(10)
                        .color(top_color)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([2, 7]))
                .style(move |_| CustomStyles::badge_pill(top_bg, top_color)),
            );
        }

        // Detected ports/services (max 2 on card to avoid clutter)
        for p in device.open_ports.iter().take(2) {
            bottom_row = bottom_row.push(
                container(
                    text(format!("{}: {}", p.port, p.service))
                        .size(9)
                        .color(CustomStyles::accent_cyan())
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([1, 5]))
                .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 40, 60), CustomStyles::accent_cyan())),
            );
        }

        let card_content = column![
            top_row,
            bottom_row,
        ]
        .spacing(8)
        .width(Length::Fill);

        container(card_content)
            .padding(Padding::from([10, 12]))
            .width(Length::Fill)
            .style(move |_| {
                if is_selected {
                    CustomStyles::card_highlighted()
                } else {
                    CustomStyles::card_container()
                }
            })
            .into()
    }

    /// Detail inspector for selected device (fixed-width reactive side panel)

    /// Floating result notification
    pub(super) fn view_notification<'a>(&self, message: &'a str, is_success: bool) -> Element<'a, Message> {
        let style_fn = if is_success {
            CustomStyles::notification_success
        } else {
            CustomStyles::notification_error
        };

        let notif_row = row![
            text(if is_success { "✓" } else { "⚠" }).size(13),
            text(message).size(12),
            space::horizontal(),
            button(text("✕").size(11))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([2, 5]))
                .on_press(Message::DismissNotification),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        container(notif_row)
            .padding(Padding::from([8, 12]))
            .width(Length::Fill)
            .style(move |_| style_fn())
            .into()
    }

}
