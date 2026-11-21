use super::*;
use crate::i18n::{Language, Translations};
use crate::theme::CustomStyles;
use crate::updater::{InAppUpdateProgress, UpdateChecker, UpdateStatus};
use iced::widget::{
    button, column, container, pick_list, row, scrollable, space, text, tooltip,
};
use iced::{alignment, Alignment, Color, Element, Length, Padding};

impl App {
    /// Modal: Classic "About NetLens" with author information, repositories, logo and dynamic language switching
    pub(super) fn view_about_modal(&self) -> Element<'_, Message> {
        let accent = CustomStyles::accent_cyan();
        let header_badge = container(text("ℹ️").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::about_modal_title(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::app_subtitle(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseAbout),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // 1. App Header Card with official Logo inside a framed container
        let app_icon = container(
            iced::widget::image(self.logo_handle.clone())
                .width(52)
                .height(52),
        )
        .padding(6)
        .style(|_| CustomStyles::inset_container());

        let update_status_element: Element<'_, Message> = match &self.update_status {
            UpdateStatus::Checking => container(
                text(Translations::checking_updates(self.language))
                    .size(10)
                    .color(CustomStyles::text_muted())
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center),
            )
            .center(Length::Shrink)
            .padding(Padding::from([3, 9]))
            .style(|_| {
                CustomStyles::badge_pill(
                    Color::from_rgb8(25, 30, 45),
                    CustomStyles::text_muted(),
                )
            })
            .into(),
            UpdateStatus::UpdateAvailable {
                remote_version,
                remote_pkgrel,
                ..
            } => row![
                container(
                    row![
                        text("🚀").size(10).align_y(alignment::Vertical::Center),
                        text(format!(
                            "{}: v{}-{}",
                            Translations::update_available(self.language),
                            remote_version,
                            remote_pkgrel
                        ))
                        .size(10)
                        .color(CustomStyles::accent_cyan())
                        .align_y(alignment::Vertical::Center),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([3, 9]))
                .style(|_| {
                    CustomStyles::badge_pill(
                        Color::from_rgb8(15, 35, 55),
                        CustomStyles::accent_cyan(),
                    )
                }),
                button(
                    container(
                        text(Translations::update_btn(self.language))
                            .size(10)
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center),
                    )
                    .center(Length::Shrink),
                )
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([3, 10]))
                .on_press(Message::OpenUpdateModal),
            ]
            .spacing(6)
            .align_y(Alignment::Center)
            .into(),
            UpdateStatus::UpToDate | UpdateStatus::Idle | UpdateStatus::CheckFailed => container(
                row![
                    text("✓")
                        .size(10)
                        .color(CustomStyles::accent_emerald())
                        .align_y(alignment::Vertical::Center),
                    text(Translations::update_latest_version(self.language))
                        .size(10)
                        .color(CustomStyles::accent_emerald())
                        .align_y(alignment::Vertical::Center),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .center(Length::Shrink)
            .padding(Padding::from([3, 9]))
            .style(|_| {
                CustomStyles::badge_pill(
                    Color::from_rgb8(15, 45, 30),
                    CustomStyles::accent_emerald(),
                )
            })
            .into(),
        };

        let check_updates_btn = self.tip(
            button(
                text("🔄")
                    .size(10)
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center),
            )
            .style(|_, status| {
                CustomStyles::storage_action_button(
                    crate::theme::StorageActionTone::Neutral,
                    status,
                )
            })
            .padding(Padding::from([3, 7]))
            .on_press(Message::CheckForUpdates),
            Translations::check_for_updates(self.language),
            tooltip::Position::Top,
        );

        let app_info = column![
            row![
                text("NetLens").size(21).color(CustomStyles::accent_cyan()),
                container(
                    text(format!(
                        "v{}-{}",
                        env!("CARGO_PKG_VERSION"),
                        UpdateChecker::CURRENT_PKGREL
                    ))
                    .size(11)
                    .color(CustomStyles::accent_emerald())
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([3, 9]))
                .style(|_| {
                    CustomStyles::badge_pill(
                        Color::from_rgb8(15, 45, 30),
                        CustomStyles::accent_emerald(),
                    )
                }),
                container(
                    text("CachyOS / Arch Linux")
                        .size(10)
                        .color(CustomStyles::accent_purple())
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center),
                )
                .center(Length::Shrink)
                .padding(Padding::from([3, 9]))
                .style(|_| {
                    CustomStyles::badge_pill(
                        Color::from_rgb8(35, 20, 50),
                        CustomStyles::accent_purple(),
                    )
                }),
                space::horizontal(),
                update_status_element,
                check_updates_btn,
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(Translations::app_subtitle(self.language))
                .size(12)
                .color(CustomStyles::text_primary()),
            text(Translations::about_app_description(self.language))
                .size(11)
                .color(CustomStyles::text_secondary()),
        ]
        .spacing(5)
        .width(Length::Fill);

        let branding_card = container(
            row![app_icon, app_info]
                .spacing(12)
                .align_y(Alignment::Center),
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_| CustomStyles::card_highlighted());

        // 2. Author & Developer Card
        let author_col = column![
            text(Translations::about_author_section(self.language))
                .size(12)
                .color(CustomStyles::accent_cyan()),
            container(
                row![
                    container(text("👨‍💻").size(18))
                        .width(34)
                        .height(34)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(|_| CustomStyles::modal_icon_badge(CustomStyles::accent_cyan())),
                    column![
                        row![
                            text("Omar Murcia")
                                .size(14)
                                .color(CustomStyles::text_primary()),
                            text(format!("• {}", Translations::about_role(self.language)))
                                .size(11)
                                .color(CustomStyles::text_muted()),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                        text(format!(
                            "✉️ {}: ing.omarmurcia@gmail.com",
                            Translations::about_email_label(self.language)
                        ))
                        .size(11)
                        .color(CustomStyles::text_secondary()),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                    row![
                        button(
                            text(format!(
                                "📋 {}",
                                Translations::about_copy_email(self.language)
                            ))
                            .size(10),
                        )
                        .style(|_, status| {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Neutral,
                                status,
                            )
                        })
                        .padding(Padding::from([4, 9]))
                        .on_press(Message::CopyToClipboard(
                            "ing.omarmurcia@gmail.com".to_string(),
                            "Email".to_string()
                        )),
                        button(
                            text(format!(
                                "✉️ {}",
                                Translations::about_send_email(self.language)
                            ))
                            .size(10),
                        )
                        .style(|_, status| {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Primary,
                                status,
                            )
                        })
                        .padding(Padding::from([4, 9]))
                        .on_press(Message::OpenExternalUrl(
                            "mailto:ing.omarmurcia@gmail.com".to_string()
                        )),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .padding(9)
            .width(Length::Fill)
            .style(|_| CustomStyles::inset_container()),
        ]
        .spacing(6);

        let author_card = container(author_col)
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // 3. Repositories & Distribution Card
        let make_repo_row = |icon: &'static str,
                             title: &'static str,
                             url: &'static str,
                             open_label: &'static str,
                             copy_label: &'static str| {
            container(
                row![
                    column![
                        text(format!("{} {}", icon, title))
                            .size(12)
                            .color(CustomStyles::text_primary()),
                        text(url).size(10).color(CustomStyles::accent_cyan()),
                    ]
                    .spacing(1)
                    .width(Length::Fill),
                    row![
                        button(text(format!("🌐 {}", open_label)).size(10))
                            .style(|_, status| {
                                CustomStyles::storage_action_button(
                                    crate::theme::StorageActionTone::Primary,
                                    status,
                                )
                            })
                            .padding(Padding::from([4, 9]))
                            .on_press(Message::OpenExternalUrl(url.to_string())),
                        button(text(format!("📋 {}", copy_label)).size(10))
                            .style(|_, status| {
                                CustomStyles::storage_action_button(
                                    crate::theme::StorageActionTone::Neutral,
                                    status,
                                )
                            })
                            .padding(Padding::from([4, 9]))
                            .on_press(Message::CopyToClipboard(
                                url.to_string(),
                                title.to_string()
                            )),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                ]
                .align_y(Alignment::Center)
                .spacing(8),
            )
            .padding(Padding::from([7, 10]))
            .width(Length::Fill)
            .style(|_| CustomStyles::inset_container())
        };

        let repos_col = column![
            text(Translations::about_repositories_section(self.language))
                .size(12)
                .color(CustomStyles::accent_cyan()),
            make_repo_row(
                "💻",
                Translations::about_source_code(self.language),
                "https://github.com/omarmurcia/netlens",
                Translations::about_open_in_browser(self.language),
                Translations::about_copy_url(self.language)
            ),
            make_repo_row(
                "📦",
                Translations::about_pacman_repo(self.language),
                "https://github.com/omarmurcia/netlens-repo",
                Translations::about_open_in_browser(self.language),
                Translations::about_copy_url(self.language)
            ),
            make_repo_row(
                "🚀",
                Translations::about_web_portal(self.language),
                "https://omarmurcia.github.io/netlens-repo",
                Translations::about_open_in_browser(self.language),
                Translations::about_copy_url(self.language)
            ),
        ]
        .spacing(6);

        let repos_card = container(repos_col)
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // 4. Technical Specs & License Card as 3 clean inset tiles
        let specs_col = column![
            text(Translations::about_tech_specs(self.language))
                .size(12)
                .color(CustomStyles::accent_cyan()),
            row![
                container(
                    column![
                        text(format!("⚖️ {}", Translations::about_license(self.language)))
                            .size(10)
                            .color(CustomStyles::text_muted()),
                        text("MIT License (Free & Open Source)")
                            .size(11)
                            .color(CustomStyles::text_primary()),
                    ]
                    .spacing(2),
                )
                .padding(9)
                .width(Length::FillPortion(1))
                .style(|_| CustomStyles::inset_container()),
                container(
                    column![
                        text("⚡ Target OS")
                            .size(10)
                            .color(CustomStyles::text_muted()),
                        text("Linux x86_64 (CachyOS / Arch)")
                            .size(11)
                            .color(CustomStyles::text_primary()),
                    ]
                    .spacing(2),
                )
                .padding(9)
                .width(Length::FillPortion(1))
                .style(|_| CustomStyles::inset_container()),
                container(
                    column![
                        text("🦀 Core Engine")
                            .size(10)
                            .color(CustomStyles::text_muted()),
                        text("Rust + Iced 0.14 + Tokio")
                            .size(11)
                            .color(CustomStyles::text_primary()),
                    ]
                    .spacing(2),
                )
                .padding(9)
                .width(Length::FillPortion(1))
                .style(|_| CustomStyles::inset_container()),
            ]
            .spacing(8),
        ]
        .spacing(6);

        let specs_card = container(specs_col)
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        // 5. Footer with Language Switcher and Copyright
        let footer_row = row![
            text(format!("🌐 {}", Translations::language_label(self.language)))
                .size(11)
                .color(CustomStyles::text_secondary()),
            pick_list(
                Language::ALL,
                Some(self.language),
                Message::LanguageChanged,
            ),
            space::horizontal(),
            text("© 2026 Omar Murcia. All rights reserved.")
                .size(10)
                .color(CustomStyles::text_muted()),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let body_cards = column![
            branding_card,
            author_card,
            repos_card,
            specs_card,
            footer_row,
        ]
        .spacing(8)
        .padding(Padding {
            top: 2.0,
            right: 2.0,
            bottom: 4.0,
            left: 2.0,
        });

        let scrollable_body = scrollable(body_cards)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(5.0)
                    .scroller_width(5.0)
                    .margin(2.0)
                    .spacing(6.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fill);

        let content = column![
            title_row,
            container(scrollable_body)
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .spacing(8);

        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(760)
            .max_height(620)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }

    /// Modal: Software Update Available Dialog with 1-line installer command and changelog
    pub(super) fn view_update_modal(&self) -> Element<'_, Message> {
        let (ver, rel, changelog, url) = match &self.update_status {
            UpdateStatus::UpdateAvailable {
                remote_version,
                remote_pkgrel,
                changelog,
                url,
            } => (
                remote_version.as_str(),
                *remote_pkgrel,
                changelog.as_deref().unwrap_or(""),
                url.as_str(),
            ),
            _ => (UpdateChecker::CURRENT_VERSION, UpdateChecker::CURRENT_PKGREL, "", "https://omarmurcia.github.io/netlens-repo"),
        };

        let title_row = row![
            text(strip_title_icon(Translations::update_modal_title(self.language)))
                .size(20)
                .color(CustomStyles::accent_cyan()),
            space::horizontal(),
            self.tip(
                button(text("✕").size(14))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 8]))
                    .on_press(Message::CloseUpdateModal),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        let version_banner = container(
            row![
                column![
                    text(format!("NetLens v{}-{}", ver, rel))
                        .size(16)
                        .color(CustomStyles::accent_cyan()),
                    text(if changelog.is_empty() {
                        Translations::app_subtitle(self.language).to_string()
                    } else {
                        changelog.to_string()
                    })
                    .size(11)
                    .color(CustomStyles::text_secondary()),
                ]
                .width(Length::Fill),
                button(text(format!("🌐 {}", Translations::about_web_portal(self.language))).size(11))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([4, 10]))
                    .on_press(Message::OpenExternalUrl(url.to_string())),
            ]
            .align_y(Alignment::Center)
        )
        .padding(14)
        .style(|_| CustomStyles::card_highlighted());

        let in_app_action_box: Element<'_, Message> = match &self.update_progress {
            InAppUpdateProgress::Idle => container(
                row![
                    column![
                        text(Translations::update_now_btn(self.language))
                            .size(13)
                            .color(CustomStyles::accent_cyan()),
                        text(match self.language {
                            Language::Spanish => "Actualiza automáticamente con 1 clic mediante la autenticación del sistema.",
                            Language::English => "Update automatically in 1 click using system authentication.",
                            Language::Chinese => "使用系统身份验证一键自动更新。",
                            Language::Hindi => "सिस्टम प्रमाणीकरण का उपयोग करके 1-क्लिक में स्वचालित रूप से अपडेट करें।",
                            Language::Arabic => "تحديث تلقائي بنقرة واحدة باستخدام مصادقة النظام.",
                            Language::French => "Mise à jour automatique en 1 clic via l'authentification système.",
                            Language::Bengali => "সিস্টেম প্রমাণীকরণ ব্যবহার করে ১-ক্লিকে স্বয়ংক্রিয়ভাবে আপডেট করুন।",
                            Language::Portuguese => "Atualize automaticamente em 1 clique usando a autenticação do sistema.",
                            Language::Russian => "Автоматическое обновление в 1 клик с системной аутентификацией.",
                            Language::Urdu => "سسٹم کی تصدیق کا استعمال کرتے ہوئے 1 کلک میں خودکار طور پر اپ ڈیٹ کریں۔",
                        })
                        .size(11)
                        .color(CustomStyles::text_muted()),
                    ]
                    .width(Length::Fill),
                    row![
                        button(text(Translations::update_in_terminal_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::secondary_button())
                            .padding(Padding::from([6, 12]))
                            .on_press(Message::OpenTerminalUpdater),
                        button(text(Translations::update_now_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::primary_button())
                            .padding(Padding::from([6, 16]))
                            .on_press(Message::StartInAppUpdate),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::card_highlighted())
            .into(),

            InAppUpdateProgress::AuthorizingAndInstalling => container(
                row![
                    text("⏳").size(16),
                    text(Translations::update_authorizing_and_installing(self.language))
                        .size(12)
                        .color(CustomStyles::accent_cyan()),
                ]
                .spacing(10)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::card_container())
            .into(),

            InAppUpdateProgress::Success => container(
                row![
                    column![
                        text(Translations::update_success_banner(self.language))
                            .size(13)
                            .color(CustomStyles::accent_emerald()),
                        text(match self.language {
                            Language::Spanish => "La nueva versión está instalada y lista para iniciarse.",
                            Language::English => "The new version is installed and ready to launch.",
                            Language::Chinese => "新版本已安装并可启动。",
                            Language::Hindi => "नया संस्करण स्थापित हो गया है और लॉन्च के लिए तैयार है।",
                            Language::Arabic => "الإصدار الجديد مثبت وجاهز للتشغيل.",
                            Language::French => "La nouvelle version est installée et prête à être lancée.",
                            Language::Bengali => "নতুন সংস্করণটি ইনস্টল করা হয়েছে এবং চালু করার জন্য প্রস্তুত।",
                            Language::Portuguese => "A nova versão está instalada e pronta para iniciar.",
                            Language::Russian => "Новая версия установлена и готова к запуску.",
                            Language::Urdu => "نیا ورژن انسٹال ہو چکا ہے اور چلانے کے لیے تیار ہے۔",
                        })
                        .size(11)
                        .color(CustomStyles::text_secondary()),
                    ]
                    .width(Length::Fill),
                    button(text(Translations::update_restart_btn(self.language)).size(11))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([6, 16]))
                        .on_press(Message::RestartNetLens),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::badge_pill(Color::from_rgb8(15, 45, 30), CustomStyles::accent_emerald()))
            .into(),

            InAppUpdateProgress::Failed(err) => container(
                row![
                    column![
                        text(format!("{} {}", Translations::update_failed_banner(self.language), err))
                            .size(12)
                            .color(CustomStyles::accent_red()),
                    ]
                    .width(Length::Fill),
                    row![
                        button(text(Translations::update_in_terminal_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::secondary_button())
                            .padding(Padding::from([6, 12]))
                            .on_press(Message::OpenTerminalUpdater),
                        button(text(format!("🔄 {}", Translations::update_retry_btn(self.language))).size(11))
                            .style(|_, _| CustomStyles::primary_button())
                            .padding(Padding::from([6, 14]))
                            .on_press(Message::StartInAppUpdate),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
            )
            .padding(14)
            .style(|_| CustomStyles::card_container())
            .into(),
        };

        let cmd_str = "curl -sSL https://omarmurcia.github.io/netlens-repo/install-repo.sh | bash";
        let cmd_box = container(
            row![
                text(cmd_str).size(12).color(CustomStyles::accent_emerald()),
                space::horizontal(),
                button(text(format!("📋 {}", Translations::update_copy_command(self.language))).size(11))
                    .style(|_, _| CustomStyles::primary_button())
                    .padding(Padding::from([5, 12]))
                    .on_press(Message::CopyToClipboard(
                        cmd_str.to_string(),
                        Translations::update_copy_command(self.language).to_string(),
                    )),
            ]
            .align_y(Alignment::Center)
        )
        .padding(12)
        .style(|_| CustomStyles::card_container());

        let instructions_col = column![
            text(Translations::update_instructions(self.language))
                .size(12)
                .color(CustomStyles::text_primary()),
            cmd_box,
        ]
        .spacing(8);

        let footer_row = row![
            space::horizontal(),
            button(text(Translations::close(self.language)).size(12))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([6, 18]))
                .on_press(Message::CloseUpdateModal),
        ]
        .align_y(Alignment::Center);

        let layout = column![
            title_row,
            version_banner,
            in_app_action_box,
            instructions_col,
            footer_row,
        ]
        .spacing(14);

        container(layout)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(740)
            .max_height(580)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }
}
