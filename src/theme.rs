use iced::widget::{button, container, scrollable};
use iced::{Background, Border, Color, Shadow, Theme};

pub struct CustomStyles;

impl CustomStyles {
    // Colores base
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

    pub fn text_primary() -> Color {
        Color::from_rgb8(248, 250, 252)
    }

    pub fn text_secondary() -> Color {
        Color::from_rgb8(148, 163, 184)
    }

    pub fn text_muted() -> Color {
        Color::from_rgb8(100, 116, 139)
    }

    // Estilo para tarjetas principales
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

    // Estilo para tarjeta destacada (Gateway o equipo actual)
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

    // Estilo para tarjeta de velocidad de descarga (Verde / Esmeralda)
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

    // Estilo para tarjeta de velocidad de subida (Púrpura / Azul)
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

    // Estilo para tarjeta de intensidad de señal Wi-Fi
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

    // Estilo para banner de alerta de intrusos / peligro
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

    // Estilo para badges / etiquetas
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

    // Estilo para el panel lateral / inspector
    pub fn inspector_panel() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(23, 32, 51))),
            border: Border {
                color: Color::from_rgb8(56, 189, 248),
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Estilo para el panel de ajustes / configuración
    pub fn settings_panel() -> container::Style {
        container::Style {
            text_color: Some(Self::text_primary()),
            background: Some(Background::Color(Color::from_rgb8(24, 32, 47))),
            border: Border {
                color: Color::from_rgb8(14, 165, 233),
                width: 1.5,
                radius: 12.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Botón de acción primario estilizado (Escanear)
    pub fn primary_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(14, 165, 233))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgb8(56, 189, 248),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Botón sutil / secundario
    pub fn secondary_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(51, 65, 85))),
            text_color: Self::text_primary(),
            border: Border {
                color: Color::from_rgb8(71, 85, 105),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Botón de peligro / bloqueo (Rojo)
    pub fn danger_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(220, 38, 38))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgb8(248, 113, 113),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Botón de éxito / readmisión (Verde esmeralda)
    pub fn success_button() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(16, 185, 129))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgb8(52, 211, 153),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Botón pill para filtros activos
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

    // Botón pill para filtros inactivos
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

    // Banner de notificación flotante (éxito)
    pub fn notification_success() -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::from_rgb8(6, 78, 59))),
            border: Border {
                color: Color::from_rgb8(16, 185, 129),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Banner de notificación flotante (error)
    pub fn notification_error() -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::from_rgb8(127, 29, 29))),
            border: Border {
                color: Color::from_rgb8(239, 68, 68),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }

    // Estilo elegante para barras de desplazamiento (Scrollables)
    pub fn scrollbar_style() -> impl Fn(&Theme, scrollable::Status) -> scrollable::Style {
        |theme, status| {
            let (scroller_bg, border_color) = match status {
                scrollable::Status::Active { .. } => (
                    Background::Color(Color::from_rgba8(148, 163, 184, 0.4)),
                    Color::TRANSPARENT,
                ),
                scrollable::Status::Hovered {
                    is_vertical_scrollbar_hovered,
                    is_horizontal_scrollbar_hovered,
                    ..
                } => {
                    if is_vertical_scrollbar_hovered || is_horizontal_scrollbar_hovered {
                        (
                            Background::Color(Color::from_rgba8(14, 165, 233, 0.85)),
                            Color::from_rgba8(56, 189, 248, 0.6),
                        )
                    } else {
                        (
                            Background::Color(Color::from_rgba8(148, 163, 184, 0.4)),
                            Color::TRANSPARENT,
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
                            Background::Color(Color::from_rgb8(14, 165, 233)),
                            Color::from_rgb8(56, 189, 248),
                        )
                    } else {
                        (
                            Background::Color(Color::from_rgba8(148, 163, 184, 0.4)),
                            Color::TRANSPARENT,
                        )
                    }
                }
            };

            let rail = scrollable::Rail {
                background: Some(Background::Color(Color::from_rgba8(15, 23, 42, 0.35))),
                border: Border {
                    radius: 4.0.into(),
                    ..Border::default()
                },
                scroller: scrollable::Scroller {
                    background: scroller_bg,
                    border: Border {
                        radius: 4.0.into(),
                        width: 1.0,
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
}
