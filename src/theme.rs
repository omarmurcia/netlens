use crate::router::storage::StorageFileType;
use iced::border;
use iced::widget::{button, container, scrollable};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageActionTone {
    Primary,
    Success,
    Neutral,
    Danger,
}

pub struct CustomStyles;

impl CustomStyles {
    // Base colors
    #[allow(dead_code)]
    pub fn bg_dark() -> Color {
        Color::from_rgb8(15, 23, 42) // Slate 900
    }

    pub fn card_bg() -> Color {
        Color::from_rgb8(30, 41, 59) // Slate 800
    }

    pub fn card_border() -> Color {
        Color::from_rgb8(51, 65, 85) // Slate 700
    }

    pub fn accent_cyan() -> Color {
        Color::from_rgb8(14, 165, 233) // Sky 500
    }

    pub fn accent_emerald() -> Color {
        Color::from_rgb8(16, 185, 129) // Emerald 500
    }

    pub fn accent_amber() -> Color {
        Color::from_rgb8(245, 158, 11) // Amber 500
    }

    pub fn accent_purple() -> Color {
        Color::from_rgb8(168, 85, 247) // Purple 500
    }

    pub fn accent_red() -> Color {
        Color::from_rgb8(239, 68, 68) // Red 500
    }

    pub fn text_primary() -> Color {
        Color::from_rgb8(248, 250, 252)
    }

    pub fn text_secondary() -> Color {
        Color::from_rgb8(148, 163, 184)
    }

    pub fn text_muted() -> Color {
        Color::from_rgb8(100, 116, 139)
    }

    // Style for primary cards
    pub fn card_container() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Self::card_bg())),
            border: Border {
                color: Self::card_border(),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Interactive button style for clickable device list cards
    pub fn device_row_button(is_selected: bool, status: button::Status) -> button::Style {
        let (bg, border_color, border_width, shadow) = if is_selected {
            (
                Color::from_rgb8(24, 38, 63),
                Color::from_rgb8(14, 165, 233),
                1.5,
                Shadow {
                    color: Color::from_rgba8(14, 165, 233, 0.22),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            )
        } else {
            match status {
                button::Status::Active | button::Status::Disabled => (
                    Self::card_bg(),
                    Self::card_border(),
                    1.0,
                    Shadow::default(),
                ),
                button::Status::Hovered => (
                    Color::from_rgb8(38, 51, 73),
                    Color::from_rgb8(56, 189, 248),
                    1.2,
                    Shadow {
                        color: Color::from_rgba8(14, 165, 233, 0.15),
                        offset: Vector::new(0.0, 2.0),
                        blur_radius: 6.0,
                    },
                ),
                button::Status::Pressed => (
                    Color::from_rgb8(20, 32, 52),
                    Color::from_rgb8(14, 165, 233),
                    1.2,
                    Shadow::default(),
                ),
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: Self::text_primary(),
            border: Border {
                color: border_color,
                width: border_width,
                radius: 10.0.into(),
            },
            shadow,
            snap: false,
        }
    }

    // Style for highlighted card (Gateway or local host)
    pub fn card_highlighted() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(24, 38, 63))),
            border: Border {
                color: Color::from_rgb8(14, 165, 233),
                width: 1.5,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Style for download speed card (Emerald / Green)
    pub fn card_download() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(13, 38, 38))),
            border: Border {
                color: Color::from_rgb8(16, 185, 129),
                width: 1.2,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Style for upload speed card (Purple / Blue)
    pub fn card_upload() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(30, 24, 54))),
            border: Border {
                color: Color::from_rgb8(168, 85, 247),
                width: 1.2,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Style for Wi-Fi signal strength card
    pub fn card_signal() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(38, 30, 18))),
            border: Border {
                color: Color::from_rgb8(245, 158, 11),
                width: 1.2,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Style for intruder / danger alert banner
    pub fn card_danger() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(45, 18, 18))),
            border: Border {
                color: Color::from_rgb8(239, 68, 68),
                width: 1.5,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Style for badges / pill labels
    pub fn badge_pill(bg_color: Color, text_color: Color) -> container::Style {
        container::Style {
            text_color: Some(text_color),
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: bg_color,
                width: 1.0,
                radius: 16.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Style for side inspector panel
    pub fn inspector_panel() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(23, 32, 51))),
            border: Border {
                color: Color::from_rgb8(56, 189, 248),
                width: 1.0,
                radius: 14.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba8(0, 0, 0, 0.25),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 14.0,
            },
            snap: false,
        }
    }

    // Style for settings / preferences panel
    pub fn settings_panel() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(24, 32, 47))),
            border: Border {
                color: Color::from_rgb8(14, 165, 233),
                width: 1.5,
                radius: 14.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba8(0, 0, 0, 0.45),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
            },
            snap: false,
        }
    }

    // Style for advanced modal dialogs (Audit, History, Wi-Fi, UPnP, Storage)
    pub fn modal_panel(accent_color: Color) -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(20, 28, 43))),
            border: Border {
                color: accent_color,
                width: 1.5,
                radius: 14.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba8(0, 0, 0, 0.45),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
            },
            snap: false,
        }
    }

    // Subtle inset container for nested scroll regions (ports, traceroute, DNS requests)
    pub fn inset_container() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(15, 23, 42))),
            border: Border {
                color: Color::from_rgb8(36, 48, 68),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Security audit check card (10px radius matching card hierarchy)
    pub fn audit_card(bg_color: Color, border_color: Color) -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Styled primary action button (Scan)
    pub fn primary_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(14, 165, 233))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgb8(56, 189, 248),
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Subtle / secondary button
    pub fn secondary_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(51, 65, 85))),
            text_color: Self::text_primary(),
            border: Border {
                color: Color::from_rgb8(71, 85, 105),
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Danger / block action button (Red)
    pub fn danger_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(220, 38, 38))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgb8(248, 113, 113),
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Success / allow button (Emerald green)
    pub fn success_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(16, 185, 129))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgb8(52, 211, 153),
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Pill button for active filters
    pub fn filter_pill_active() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(14, 165, 233))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgb8(56, 189, 248),
                width: 1.0,
                radius: 20.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Pill button for inactive filters
    pub fn filter_pill_inactive() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(30, 41, 59))),
            text_color: Self::text_secondary(),
            border: Border {
                color: Color::from_rgb8(51, 65, 85),
                width: 1.0,
                radius: 20.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Floating notification banner (success)
    pub fn notification_success() -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::from_rgb8(6, 78, 59))),
            border: Border {
                color: Color::from_rgb8(16, 185, 129),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Floating notification banner (error)
    pub fn notification_error() -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::from_rgb8(127, 29, 29))),
            border: Border {
                color: Color::from_rgb8(239, 68, 68),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Sleek, non-intrusive scrollbar style (transparent rail when idle to prevent layout noise)
    pub fn scrollbar_style() -> impl Fn(&Theme, scrollable::Status) -> scrollable::Style {
        |theme, status| {
            let (rail_bg, scroller_bg, border_color, border_width) = match status {
                scrollable::Status::Active { .. } => (
                    None,
                    Background::Color(Color::from_rgba8(148, 163, 184, 0.32)),
                    Color::TRANSPARENT,
                    0.0,
                ),
                scrollable::Status::Hovered {
                    is_vertical_scrollbar_hovered,
                    is_horizontal_scrollbar_hovered,
                    ..
                } => {
                    if is_vertical_scrollbar_hovered || is_horizontal_scrollbar_hovered {
                        (
                            Some(Background::Color(Color::from_rgba8(15, 23, 42, 0.28))),
                            Background::Color(Color::from_rgba8(14, 165, 233, 0.85)),
                            Color::from_rgba8(56, 189, 248, 0.5),
                            1.0,
                        )
                    } else {
                        (
                            None,
                            Background::Color(Color::from_rgba8(148, 163, 184, 0.45)),
                            Color::TRANSPARENT,
                            0.0,
                        )
                    }
                }
                scrollable::Status::Dragged {
                    is_vertical_scrollbar_dragged,
                    is_horizontal_scrollbar_dragged,
                    ..
                } => {
                    if is_vertical_scrollbar_dragged || is_horizontal_scrollbar_dragged {
                        (
                            Some(Background::Color(Color::from_rgba8(15, 23, 42, 0.35))),
                            Background::Color(Color::from_rgb8(14, 165, 233)),
                            Color::from_rgb8(56, 189, 248),
                            1.0,
                        )
                    } else {
                        (
                            None,
                            Background::Color(Color::from_rgba8(148, 163, 184, 0.45)),
                            Color::TRANSPARENT,
                            0.0,
                        )
                    }
                }
            };

            let rail = scrollable::Rail {
                background: rail_bg,
                border: Border {
                    radius: 4.0.into(),
                    ..Border::default()
                },
                scroller: scrollable::Scroller {
                    background: scroller_bg,
                    border: Border {
                        radius: 4.0.into(),
                        width: border_width,
                        color: border_color,
                    },
                },
            };

            scrollable::Style {
                container: container::Style::default(),
                vertical_rail: rail,
                horizontal_rail: rail,
                gap: None,
                auto_scroll: scrollable::default(theme, status).auto_scroll,
            }
        }
    }

    /// Discreet, slim horizontal scrollbar that only appears when hovering or dragging over the area
    pub fn discrete_horizontal_scrollbar_style() -> impl Fn(&Theme, scrollable::Status) -> scrollable::Style {
        |theme, status| {
            let scroller_bg = match status {
                scrollable::Status::Active { .. } => Background::Color(Color::TRANSPARENT),
                scrollable::Status::Hovered {
                    is_horizontal_scrollbar_hovered,
                    is_horizontal_scrollbar_disabled,
                    ..
                } => {
                    if is_horizontal_scrollbar_disabled {
                        Background::Color(Color::TRANSPARENT)
                    } else if is_horizontal_scrollbar_hovered {
                        Background::Color(Color::from_rgba8(56, 189, 248, 0.75))
                    } else {
                        Background::Color(Color::from_rgba8(148, 163, 184, 0.35))
                    }
                }
                scrollable::Status::Dragged {
                    is_horizontal_scrollbar_dragged,
                    is_horizontal_scrollbar_disabled,
                    ..
                } => {
                    if is_horizontal_scrollbar_disabled {
                        Background::Color(Color::TRANSPARENT)
                    } else if is_horizontal_scrollbar_dragged {
                        Background::Color(Color::from_rgb8(14, 165, 233))
                    } else {
                        Background::Color(Color::from_rgba8(148, 163, 184, 0.45))
                    }
                }
            };

            let rail = scrollable::Rail {
                background: None,
                border: Border::default(),
                scroller: scrollable::Scroller {
                    background: scroller_bg,
                    border: Border {
                        radius: 3.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                },
            };

            scrollable::Style {
                container: container::Style::default(),
                vertical_rail: rail,
                horizontal_rail: rail,
                gap: None,
                auto_scroll: scrollable::default(theme, status).auto_scroll,
            }
        }
    }

    /// Style for elegant tooltip popup containers
    pub fn tooltip_style() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(15, 23, 42))),
            border: Border {
                color: Color::from_rgb8(51, 65, 85),
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba8(0, 0, 0, 0.4),
                offset: Vector::new(0.0, 3.0),
                blur_radius: 10.0,
            },
            snap: false,
        }
    }

    /// Style for router storage file list area in normal/idle state (bottom corners rounded to join header)
    pub fn drop_zone_idle() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(15, 23, 42))),
            border: Border {
                color: Color::from_rgb8(42, 54, 74),
                width: 1.0,
                radius: border::bottom(8.0),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Style for router storage file list area when files/folders are hovered for drop
    pub fn drop_zone_active() -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::from_rgba8(14, 165, 233, 0.15))),
            border: Border {
                color: Color::from_rgb8(56, 189, 248),
                width: 2.0,
                radius: border::bottom(8.0),
            },
            shadow: Shadow {
                color: Color::from_rgba8(14, 165, 233, 0.35),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 12.0,
            },
            snap: false,
        }
    }

    /// Style for router storage file list rows (borderless zebra striping, inherits table background)
    pub fn storage_file_row(index: usize) -> container::Style {
        let background = if index % 2 == 1 {
            Some(Background::Color(Color::from_rgba8(30, 41, 59, 0.36)))
        } else {
            None
        };

        container::Style {
            text_color: Some(Self::text_primary()),
            background,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Soft tinted icon badge for router storage file types
    pub fn storage_type_badge(file_type: StorageFileType) -> container::Style {
        let (bg, border_col) = match file_type {
            StorageFileType::Folder => (
                Color::from_rgba8(14, 165, 233, 0.14),
                Color::from_rgba8(56, 189, 248, 0.28),
            ),
            StorageFileType::Image => (
                Color::from_rgba8(168, 85, 247, 0.14),
                Color::from_rgba8(192, 132, 252, 0.28),
            ),
            StorageFileType::Video => (
                Color::from_rgba8(244, 63, 94, 0.14),
                Color::from_rgba8(251, 113, 133, 0.28),
            ),
            StorageFileType::Audio => (
                Color::from_rgba8(16, 185, 129, 0.14),
                Color::from_rgba8(52, 211, 153, 0.28),
            ),
            StorageFileType::Document => (
                Color::from_rgba8(59, 130, 246, 0.14),
                Color::from_rgba8(96, 165, 250, 0.28),
            ),
            StorageFileType::Archive => (
                Color::from_rgba8(245, 158, 11, 0.14),
                Color::from_rgba8(251, 191, 36, 0.28),
            ),
            StorageFileType::Executable => (
                Color::from_rgba8(249, 115, 22, 0.14),
                Color::from_rgba8(251, 146, 60, 0.28),
            ),
            StorageFileType::Code => (
                Color::from_rgba8(20, 184, 166, 0.14),
                Color::from_rgba8(45, 212, 191, 0.28),
            ),
            StorageFileType::Unknown => (
                Color::from_rgba8(100, 116, 139, 0.14),
                Color::from_rgba8(148, 163, 184, 0.22),
            ),
        };

        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(bg)),
            border: Border {
                color: border_col,
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Clean interactive link-style button for file/folder names in the storage table
    pub fn storage_name_button(is_dir: bool, status: button::Status) -> button::Style {
        let base_text = if is_dir {
            Color::from_rgb8(56, 189, 248)
        } else {
            Self::text_primary()
        };

        let (background, border_color, border_width, text_color) = match status {
            button::Status::Active | button::Status::Disabled => {
                (None, Color::TRANSPARENT, 0.0, base_text)
            }
            button::Status::Hovered => {
                let hover_bg = if is_dir {
                    Color::from_rgba8(56, 189, 248, 0.12)
                } else {
                    Color::from_rgba8(148, 163, 184, 0.12)
                };
                let hover_border = if is_dir {
                    Color::from_rgba8(56, 189, 248, 0.28)
                } else {
                    Color::from_rgba8(148, 163, 184, 0.22)
                };
                (
                    Some(Background::Color(hover_bg)),
                    hover_border,
                    1.0,
                    if is_dir {
                        Color::from_rgb8(125, 211, 252)
                    } else {
                        Color::WHITE
                    },
                )
            }
            button::Status::Pressed => (
                Some(Background::Color(Color::from_rgba8(56, 189, 248, 0.20))),
                Color::from_rgba8(56, 189, 248, 0.40),
                1.0,
                Color::WHITE,
            ),
        };

        button::Style {
            background,
            text_color,
            border: Border {
                color: border_color,
                width: border_width,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Ghost-pill action buttons for storage table rows
    pub fn storage_action_button(tone: StorageActionTone, status: button::Status) -> button::Style {
        let (idle_bg, idle_border, idle_text, hover_bg, hover_border, hover_text) = match tone {
            StorageActionTone::Primary => (
                Color::from_rgba8(14, 165, 233, 0.14),
                Color::from_rgba8(56, 189, 248, 0.28),
                Color::from_rgb8(56, 189, 248),
                Color::from_rgba8(14, 165, 233, 0.30),
                Color::from_rgb8(56, 189, 248),
                Color::WHITE,
            ),
            StorageActionTone::Success => (
                Color::from_rgba8(16, 185, 129, 0.14),
                Color::from_rgba8(52, 211, 153, 0.28),
                Color::from_rgb8(52, 211, 153),
                Color::from_rgba8(16, 185, 129, 0.28),
                Color::from_rgb8(52, 211, 153),
                Color::WHITE,
            ),
            StorageActionTone::Neutral => (
                Color::from_rgba8(51, 65, 85, 0.42),
                Color::from_rgba8(71, 85, 105, 0.55),
                Color::from_rgb8(203, 213, 225),
                Color::from_rgba8(71, 85, 105, 0.72),
                Color::from_rgb8(148, 163, 184),
                Color::WHITE,
            ),
            StorageActionTone::Danger => (
                Color::from_rgba8(239, 68, 68, 0.12),
                Color::from_rgba8(248, 113, 113, 0.26),
                Color::from_rgb8(248, 113, 113),
                Color::from_rgb8(220, 38, 38),
                Color::from_rgb8(248, 113, 113),
                Color::WHITE,
            ),
        };

        let (bg, border_col, text_col) = match status {
            button::Status::Active | button::Status::Disabled => (idle_bg, idle_border, idle_text),
            button::Status::Hovered | button::Status::Pressed => {
                (hover_bg, hover_border, hover_text)
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border {
                color: border_col,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Style for router storage table header (top corners rounded to cap the table frame)
    pub fn storage_table_header() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(17, 25, 44))),
            border: Border {
                color: Color::from_rgb8(42, 54, 74),
                width: 1.0,
                radius: border::top(8.0),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Accent-tinted icon container for modal headers and KPI badges
    pub fn modal_icon_badge(accent: Color) -> container::Style {
        container::Style {
            text_color: Some(accent),
            background: Some(Background::Color(Color { a: 0.14, ..accent })),
            border: Border {
                color: Color { a: 0.32, ..accent },
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Subtle tinted card/tile for metrics, channels, and status banners
    pub fn tinted_badge_box(accent: Color) -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color { a: 0.09, ..accent })),
            border: Border {
                color: Color { a: 0.26, ..accent },
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Sleek close button ('✕') for modal headers with red hover highlight
    pub fn modal_close_button(status: button::Status) -> button::Style {
        let (bg, border_col, text_col) = match status {
            button::Status::Active | button::Status::Disabled => (
                Color::from_rgba8(51, 65, 85, 0.45),
                Color::from_rgba8(71, 85, 105, 0.55),
                Self::text_secondary(),
            ),
            button::Status::Hovered => (
                Color::from_rgba8(239, 68, 68, 0.22),
                Color::from_rgb8(248, 113, 113),
                Color::WHITE,
            ),
            button::Status::Pressed => (
                Color::from_rgb8(220, 38, 38),
                Color::from_rgb8(248, 113, 113),
                Color::WHITE,
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border {
                color: border_col,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Recessed command-dock container for the top navigation toolbar
    pub fn nav_toolbar_container() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(17, 25, 43))),
            border: Border {
                color: Color::from_rgb8(42, 56, 80),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    /// Interactive navigation pill button with per-tool accent color and active/hover states
    pub fn nav_tool_button(
        is_active: bool,
        accent: Color,
        status: button::Status,
    ) -> button::Style {
        let (bg, border_color, border_width, text_color, shadow) = if is_active {
            (
                Color { a: 0.24, ..accent },
                accent,
                1.2,
                Color::WHITE,
                Shadow {
                    color: Color { a: 0.25, ..accent },
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            )
        } else {
            match status {
                button::Status::Active | button::Status::Disabled => (
                    Color::from_rgba8(30, 41, 59, 0.65),
                    Color::from_rgba8(51, 65, 85, 0.80),
                    1.0,
                    Color::from_rgb8(226, 232, 240),
                    Shadow::default(),
                ),
                button::Status::Hovered => (
                    Color { a: 0.16, ..accent },
                    Color { a: 0.48, ..accent },
                    1.0,
                    Color::WHITE,
                    Shadow::default(),
                ),
                button::Status::Pressed => (
                    Color { a: 0.26, ..accent },
                    accent,
                    1.0,
                    Color::WHITE,
                    Shadow::default(),
                ),
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border {
                color: border_color,
                width: border_width,
                radius: 8.0.into(),
            },
            shadow,
            snap: false,
        }
    }
}
