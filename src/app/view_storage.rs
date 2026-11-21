use super::*;
use crate::i18n::Translations;
use crate::router::{
    format_file_size, StorageFileEntry,
};
use crate::theme::CustomStyles;
use iced::widget::{
    button, column, container, progress_bar, row, scrollable, space, text,
    text_input, tooltip,
};
use iced::{Alignment, Color, Element, Length, Padding};

impl App {
    /// Router USB Storage / NAS Explorer
    pub(super) fn view_storage_explorer_modal(&self) -> Element<'_, Message> {
        let accent = CustomStyles::accent_cyan();
        let header_badge = container(text("💾").size(16))
            .width(34)
            .height(34)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| CustomStyles::modal_icon_badge(accent));

        let title_row = row![
            header_badge,
            column![
                text(strip_title_icon(Translations::storage_modal_title(self.language)))
                    .size(19)
                    .color(CustomStyles::text_primary()),
                text(Translations::storage_modal_desc(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(1),
            space::horizontal(),
            self.tip(
                button(text("✕").size(13))
                    .style(|_, status| CustomStyles::modal_close_button(status))
                    .padding(Padding::from([4, 9]))
                    .on_press(Message::CloseStorageExplorer),
                Translations::tip_close(self.language),
                tooltip::Position::Left,
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Row 1: Connection Control, Target IP, Credentials, and Demo Mode inside a clean top control card
        let ip_input = text_input("192.168.1.1", &self.storage_target_ip)
            .on_input(Message::StorageTargetIpChanged)
            .padding(Padding::from([4, 8]))
            .width(130);

        let refresh_btn = self.tip(
            button(text(Translations::storage_refresh(self.language)).size(11))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([4, 11]))
                .on_press(Message::RefreshStorageShares),
            Translations::tip_storage_refresh(self.language),
            tooltip::Position::Bottom,
        );

        let creds_open = self.is_storage_creds_open;
        let creds_btn = self.tip(
            button(text(Translations::storage_btn_credentials(self.language)).size(11))
                .style(move |_, status| {
                    let tone = if creds_open {
                        crate::theme::StorageActionTone::Primary
                    } else {
                        crate::theme::StorageActionTone::Neutral
                    };
                    CustomStyles::storage_action_button(tone, status)
                })
                .padding(Padding::from([4, 11]))
                .on_press(Message::ToggleStorageCredentialsModal),
            Translations::tip_storage_creds(self.language),
            tooltip::Position::Bottom,
        );

        let is_demo = self.is_storage_demo_mode;
        let demo_btn = self.tip(
            button(text(Translations::storage_demo_mode_btn(self.language)).size(11))
                .style(move |_, status| {
                    let tone = if is_demo {
                        crate::theme::StorageActionTone::Success
                    } else {
                        crate::theme::StorageActionTone::Neutral
                    };
                    CustomStyles::storage_action_button(tone, status)
                })
                .padding(Padding::from([4, 11]))
                .on_press(Message::StorageToggleDemoMode),
            Translations::tip_storage_demo(self.language),
            tooltip::Position::Bottom,
        );

        let mut top_control_col = column![
            row![
                text(Translations::storage_target_label(self.language))
                    .size(11)
                    .color(CustomStyles::text_secondary()),
                ip_input,
                refresh_btn,
                creds_btn,
                space::horizontal(),
                demo_btn,
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(8);

        // Capacity & Disk Space Bar embedded in the top control card
        if let Some(disk) = self.storage_disk_info {
            let used_str = format_file_size(disk.used_bytes());
            let total_str = format_file_size(disk.total_bytes);
            let free_str = format_file_size(disk.free_bytes);
            let percent = disk.used_percent();

            let capacity_strip = container(
                row![
                    text(Translations::storage_space_info(
                        &free_str,
                        &total_str,
                        percent as f64,
                        self.language
                    ))
                    .size(11)
                    .color(CustomStyles::accent_cyan()),
                    container(progress_bar(0.0..=100.0, percent)).width(Length::Fill),
                    text(format!("{} / {}", used_str, total_str))
                        .size(10)
                        .color(CustomStyles::text_secondary()),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(Padding::from([6, 10]))
            .width(Length::Fill)
            .style(|_| CustomStyles::inset_container());

            top_control_col = top_control_col.push(capacity_strip);
        }

        // Share / Resource Selector embedded in the top control card
        if !self.storage_shares.is_empty() {
            let mut share_chips = row![
                text(Translations::storage_share_select(self.language))
                    .size(11)
                    .color(CustomStyles::text_muted()),
            ]
            .spacing(6)
            .align_y(Alignment::Center);

            for share in &self.storage_shares {
                let is_active = self.storage_active_share.as_deref() == Some(&share.name);
                let share_name = share.name.clone();
                let label = if share.comment.is_empty() {
                    format!("📁 {}", share.name)
                } else {
                    format!("📁 {} ({})", share.name, share.comment)
                };

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
                    .padding(Padding::from([4, 10]))
                    .on_press(Message::StorageShareSelected(share_name));

                share_chips = share_chips.push(btn);
            }

            top_control_col = top_control_col.push(share_chips);
        }

        let top_control_card = container(top_control_col)
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

        let mut modal_body = column![top_control_card].spacing(8);

        // SMB / NAS Credentials dialog if open
        if self.is_storage_creds_open {
            let user_in = text_input(
                Translations::storage_creds_user_placeholder(self.language),
                &self.storage_credentials.username,
            )
            .on_input(Message::StorageCredentialsUserChanged)
            .padding(Padding::from([4, 8]))
            .width(180);

            let pass_in = text_input(
                Translations::storage_creds_pass_placeholder(self.language),
                &self.storage_credentials.password,
            )
            .on_input(Message::StorageCredentialsPassChanged)
            .secure(true)
            .padding(Padding::from([4, 8]))
            .width(180);

            let wg_in = text_input("WORKGROUP", &self.storage_credentials.workgroup)
                .on_input(Message::StorageCredentialsWorkgroupChanged)
                .padding(Padding::from([4, 8]))
                .width(140);

            let creds_card = container(
                column![
                    text(Translations::storage_creds_title(self.language))
                        .size(12)
                        .color(CustomStyles::accent_cyan()),
                    text(Translations::storage_creds_desc(self.language))
                        .size(10)
                        .color(CustomStyles::text_muted()),
                    row![
                        text(Translations::storage_creds_user_label(self.language))
                            .size(11)
                            .color(CustomStyles::text_secondary()),
                        user_in,
                        text(Translations::storage_creds_pass_label(self.language))
                            .size(11)
                            .color(CustomStyles::text_secondary()),
                        pass_in,
                        text(Translations::storage_creds_group_label(self.language))
                            .size(11)
                            .color(CustomStyles::text_secondary()),
                        wg_in,
                        button(text(Translations::storage_creds_save_btn(self.language)).size(11))
                            .style(|_, _| CustomStyles::primary_button())
                            .padding(Padding::from([4, 10]))
                            .on_press(Message::StorageSaveCredentials),
                        button(text(Translations::cancel(self.language)).size(11))
                            .style(|_, status| {
                                CustomStyles::storage_action_button(
                                    crate::theme::StorageActionTone::Neutral,
                                    status,
                                )
                            })
                            .padding(Padding::from([4, 10]))
                            .on_press(Message::ToggleStorageCredentialsModal),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(6),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_highlighted());

            modal_body = modal_body.push(creds_card);
        }

        // Navigation & Toolbar (Breadcrumbs inside recessed address bar, Upload, New Folder, Filter)
        let is_root = self.storage_current_path.trim_matches('/').is_empty();
        let up_btn = self.tip(
            button(text(Translations::storage_up_level(self.language)).size(10))
                .style(move |_, status| {
                    let tone = if is_root {
                        crate::theme::StorageActionTone::Neutral
                    } else {
                        crate::theme::StorageActionTone::Primary
                    };
                    CustomStyles::storage_action_button(tone, status)
                })
                .padding(Padding::from([4, 8]))
                .on_press(if is_root {
                    Message::CloseStorageExplorer
                } else {
                    Message::StorageNavigateUp
                }),
            Translations::tip_storage_up(self.language),
            tooltip::Position::Bottom,
        );

        let root_btn = self.tip(
            button(text(Translations::storage_btn_root(self.language)).size(11))
                .style(|_, status| CustomStyles::storage_name_button(true, status))
                .padding(Padding::from([2, 6]))
                .on_press(Message::StorageNavigateTo(String::new())),
            Translations::tip_storage_root(self.language),
            tooltip::Position::Bottom,
        );

        let mut breadcrumbs = row![root_btn].spacing(2).align_y(Alignment::Center);

        if !is_root {
            let parts: Vec<&str> = self
                .storage_current_path
                .trim_matches('/')
                .split('/')
                .collect();
            let mut accum = String::new();
            for part in parts {
                if !accum.is_empty() {
                    accum.push('/');
                }
                accum.push_str(part);
                let path_target = accum.clone();

                breadcrumbs = breadcrumbs
                    .push(text("/").size(11).color(CustomStyles::text_muted()))
                    .push(
                        button(text(part).size(11))
                            .style(|_, status| CustomStyles::storage_name_button(true, status))
                            .padding(Padding::from([2, 6]))
                            .on_press(Message::StorageNavigateTo(path_target)),
                    );
            }
        }

        let breadcrumb_bar = container(breadcrumbs)
            .padding(Padding::from([3, 8]))
            .width(Length::Fill)
            .style(|_| CustomStyles::inset_container());

        let new_folder_btn = self.tip(
            button(text(Translations::storage_new_folder(self.language)).size(10))
                .style(|_, status| {
                    CustomStyles::storage_action_button(
                        crate::theme::StorageActionTone::Neutral,
                        status,
                    )
                })
                .padding(Padding::from([4, 9]))
                .on_press(Message::StorageStartCreateFolder),
            Translations::tip_storage_new_folder(self.language),
            tooltip::Position::Bottom,
        );

        let upload_file_btn = self.tip(
            button(text(Translations::storage_upload_btn(self.language)).size(10))
                .style(|_, status| {
                    CustomStyles::storage_action_button(
                        crate::theme::StorageActionTone::Primary,
                        status,
                    )
                })
                .padding(Padding::from([4, 9]))
                .on_press(Message::StoragePickFileAndUpload),
            Translations::tip_storage_upload_file(self.language),
            tooltip::Position::Bottom,
        );

        let upload_folder_btn = self.tip(
            button(text(Translations::storage_upload_folder_btn(self.language)).size(10))
                .style(|_, status| {
                    CustomStyles::storage_action_button(
                        crate::theme::StorageActionTone::Success,
                        status,
                    )
                })
                .padding(Padding::from([4, 9]))
                .on_press(Message::StoragePickFolderAndUpload),
            Translations::tip_storage_upload_folder(self.language),
            tooltip::Position::Bottom,
        );

        let filter_in = text_input(
            Translations::storage_filter_placeholder(self.language),
            &self.storage_filter_query,
        )
        .on_input(Message::StorageFilterQueryChanged)
        .padding(Padding::from([4, 8]))
        .width(180);

        let toolbar = row![
            up_btn,
            breadcrumb_bar,
            new_folder_btn,
            upload_file_btn,
            upload_folder_btn,
            filter_in,
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        modal_body = modal_body.push(toolbar);

        // Upload Progress Bar & Card (File or Folder)
        if let Some(upload) = &self.storage_upload_progress {
            let percent = upload.percent.clamp(0.0, 100.0);
            let percent_str = format!("{:.1}%", percent);
            let icon = if upload.is_folder { "📁" } else { "📄" };

            let title_color = if upload.is_finished {
                if upload.error.is_some() {
                    Color::from_rgb8(239, 68, 68)
                } else {
                    CustomStyles::accent_emerald()
                }
            } else {
                CustomStyles::accent_cyan()
            };

            let title_row = row![
                text(format!("{} {} | {}", icon, Translations::storage_upload_progress_title(self.language), upload.title))
                    .size(12)
                    .color(title_color)
                    .width(Length::Fill),
                if !upload.is_finished {
                    text(format!("⚡ {}: {}", Translations::storage_upload_progress_speed(self.language), upload.speed_str))
                        .size(11)
                        .color(CustomStyles::accent_emerald())
                } else if upload.error.is_some() {
                    text(Translations::storage_upload_error(self.language))
                        .size(11)
                        .color(Color::from_rgb8(239, 68, 68))
                } else {
                    text(Translations::storage_upload_completed(self.language))
                        .size(11)
                        .color(CustomStyles::accent_emerald())
                },
                button(text(Translations::storage_upload_dismiss_btn(self.language)).size(10))
                    .style(|_, _| CustomStyles::secondary_button())
                    .padding(Padding::from([2, 6]))
                    .on_press(Message::StorageDismissUploadProgress),
            ]
            .spacing(10)
            .align_y(Alignment::Center);

            let pbar = progress_bar(0.0..=100.0, percent);

            let uploaded_str = format_file_size(upload.bytes_uploaded);
            let total_str = format_file_size(upload.total_bytes);

            let metrics_row = if upload.is_folder {
                row![
                    text(Translations::storage_upload_folder_progress(
                        upload.current_index, upload.total_items, &upload.current_item, self.language
                    ))
                    .size(10)
                    .color(CustomStyles::text_primary())
                    .width(Length::Fill),
                    text(format!("{} / {} ({})", uploaded_str, total_str, percent_str))
                        .size(10)
                        .color(CustomStyles::accent_cyan()),
                ]
                .align_y(Alignment::Center)
            } else {
                row![
                    text(Translations::storage_upload_file_progress(&upload.current_item, self.language))
                        .size(10)
                        .color(CustomStyles::text_primary())
                        .width(Length::Fill),
                    text(format!("{} / {} ({})", uploaded_str, total_str, percent_str))
                        .size(10)
                        .color(CustomStyles::accent_cyan()),
                ]
                .align_y(Alignment::Center)
            };

            let dest_share = self.storage_active_share.as_deref().unwrap_or("");
            let dest_dir = if upload.target_dir.is_empty() { Translations::storage_root_label(self.language) } else { &upload.target_dir };
            let dest_row = row![
                text(format!("{} /{}/{}", Translations::storage_upload_progress_target(self.language), dest_share, dest_dir))
                    .size(10)
                    .color(CustomStyles::text_muted()),
            ];

            let card = container(
                column![
                    title_row,
                    container(pbar).width(Length::Fill),
                    metrics_row,
                    dest_row,
                ]
                .spacing(6),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_upload());

            modal_body = modal_body.push(card);
        }

        // Inline form for creating new folder
        if self.is_creating_folder {
            let folder_input = text_input(Translations::storage_new_folder_placeholder(self.language), &self.new_folder_name_input)
                .on_input(Message::StorageCreateFolderNameChanged)
                .padding(Padding::from([3, 8]))
                .width(220);

            let confirm_btn = button(text(Translations::storage_create_confirm(self.language)).size(10))
                .style(|_, _| CustomStyles::primary_button())
                .padding(Padding::from([3, 8]))
                .on_press(Message::StorageConfirmCreateFolder);

            let cancel_btn = button(text(Translations::cancel(self.language)).size(10))
                .style(|_, _| CustomStyles::secondary_button())
                .padding(Padding::from([3, 8]))
                .on_press(Message::StorageCancelCreateFolder);

            let create_row = row![
                text(Translations::storage_create_folder_prompt(self.language)).size(11).color(CustomStyles::accent_cyan()),
                folder_input,
                confirm_btn,
                cancel_btn,
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            modal_body = modal_body.push(container(create_row).padding(8).style(|_| CustomStyles::card_container()));
        }

        // Delete Confirmation Dialog
        if let Some((_, _is_dir, name)) = &self.file_to_delete {
            let msg = format!("{}: '{}'?", Translations::storage_confirm_delete_msg(self.language), name);
            let confirm_del_card = container(
                row![
                    text("⚠️").size(16),
                    column![
                        text(Translations::storage_confirm_delete_title(self.language)).size(12).color(Color::from_rgb8(239, 68, 68)),
                        text(msg).size(11).color(CustomStyles::text_primary()),
                    ].width(Length::Fill),
                    button(text(Translations::cancel(self.language)).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([4, 10]))
                        .on_press(Message::StorageCancelDeleteFile),
                    button(text(Translations::storage_action_delete(self.language)).size(10))
                        .style(|_, _| CustomStyles::danger_button())
                        .padding(Padding::from([4, 10]))
                        .on_press(Message::StorageConfirmDeleteFile),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_danger());

            modal_body = modal_body.push(confirm_del_card);
        }

        // File Properties & Metadata Modal / Card
        if let Some(f) = &self.storage_selected_file {
            let full_path = if self.storage_current_path.is_empty() {
                f.name.clone()
            } else {
                format!("{}/{}", self.storage_current_path.trim_matches('/'), f.name)
            };

            let mut actions_row = row![].spacing(8).align_y(Alignment::Center);
            if !f.is_dir {
                actions_row = actions_row.push(
                    button(text(format!("👁 {}", Translations::storage_action_open(self.language))).size(10))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([3, 10]))
                        .on_press(Message::StorageOpenFile(f.name.clone())),
                );
                actions_row = actions_row.push(
                    button(text(format!("💾 {}", Translations::storage_action_download(self.language))).size(10))
                        .style(|_, _| CustomStyles::secondary_button())
                        .padding(Padding::from([3, 10]))
                        .on_press(Message::StorageDownloadFile(f.name.clone())),
                );
            } else {
                actions_row = actions_row.push(
                    button(text(format!("📁 {}", Translations::storage_action_open(self.language))).size(10))
                        .style(|_, _| CustomStyles::primary_button())
                        .padding(Padding::from([3, 10]))
                        .on_press(Message::StorageNavigateTo(full_path.clone())),
                );
            }

            let props_card = container(
                column![
                    row![
                        text(format!("{} {}", f.file_type.icon(), Translations::storage_props_title(&f.name, self.language))).size(12).color(CustomStyles::accent_cyan()),
                        space::horizontal(),
                        self.tip(
                            button(text("✕").size(10))
                                .style(|_, _| CustomStyles::secondary_button())
                                .padding(Padding::from([2, 6]))
                                .on_press(Message::StorageCloseProperties),
                            Translations::tip_close(self.language),
                            tooltip::Position::Left,
                        ),
                    ].align_y(Alignment::Center),
                    row![
                        text(Translations::storage_prop_type(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(f.file_type.type_name(self.language)).size(11).color(CustomStyles::text_primary()),
                    ],
                    row![
                        text(Translations::storage_prop_full_path(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(full_path.clone()).size(11).color(CustomStyles::accent_cyan()),
                    ],
                    row![
                        text(Translations::storage_prop_size(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(format!("{} ({} bytes)", f.formatted_size(), f.size_bytes)).size(11).color(CustomStyles::text_primary()),
                    ],
                    row![
                        text(Translations::storage_prop_modified(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(&f.modified_str).size(11).color(CustomStyles::text_primary()),
                    ],
                    row![
                        text(Translations::storage_prop_attributes(self.language)).size(11).color(CustomStyles::text_muted()).width(100),
                        text(&f.attributes).size(11).color(CustomStyles::text_primary()),
                    ],
                    actions_row,
                ]
                .spacing(4),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

            modal_body = modal_body.push(props_card);
        }

        // Status or error message
        if let Some(err) = &self.storage_error_msg {
            let err_card = container(
                row![
                    text("⚠️").size(14),
                    text(err).size(11).color(Color::from_rgb8(239, 68, 68)).width(Length::Fill),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(8)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_danger());

            modal_body = modal_body.push(err_card);
        }

        if let Some(status) = &self.storage_status_msg {
            let status_card = container(
                row![
                    text("✓").size(14).color(CustomStyles::accent_emerald()),
                    text(status).size(11).color(CustomStyles::accent_emerald()).width(Length::Fill),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(8)
            .width(Length::Fill)
            .style(|_| CustomStyles::card_container());

            modal_body = modal_body.push(status_card);
        }

        // File table header with strictly aligned widths and reserved right gutter for vertical scrollbar
        let table_header = container(
            row![
                container(text(Translations::storage_col_type(self.language)).size(11).color(CustomStyles::text_muted())).width(45).align_x(Alignment::Center),
                container(text(Translations::storage_col_name(self.language)).size(11).color(CustomStyles::text_muted())).width(Length::Fill).align_x(Alignment::Start),
                container(text(Translations::storage_col_size(self.language)).size(11).color(CustomStyles::text_muted())).width(100).align_x(Alignment::Center),
                container(text(Translations::storage_col_modified(self.language)).size(11).color(CustomStyles::text_muted())).width(160).align_x(Alignment::Center),
                container(text(Translations::storage_col_actions(self.language)).size(11).color(CustomStyles::text_muted())).width(230).align_x(Alignment::Center),
            ]
            .align_y(Alignment::Center)
            .padding(Padding {
                top: 6.0,
                right: 16.0,
                bottom: 6.0,
                left: 10.0,
            }),
        )
        .width(Length::Fill)
        .style(|_| CustomStyles::storage_table_header());

        // Files / folders content
        let query = self.storage_filter_query.trim().to_lowercase();
        let filtered_files: Vec<&StorageFileEntry> = self
            .storage_files
            .iter()
            .filter(|f| query.is_empty() || f.name.to_lowercase().contains(&query))
            .collect();

        let mut files_column = column![].spacing(2).padding(Padding {
            top: 4.0,
            right: 6.0,
            bottom: 4.0,
            left: 4.0,
        });

        if filtered_files.is_empty() {
            let empty_text = if self.is_storage_loading {
                Translations::storage_loading_files(self.language)
            } else if !query.is_empty() {
                Translations::storage_no_filter_match(self.language)
            } else {
                Translations::storage_empty_dir(self.language)
            };

            files_column = files_column.push(
                container(
                    text(empty_text)
                        .size(12)
                        .color(CustomStyles::text_muted()),
                )
                .padding(24)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            );
        } else {
            for (row_idx, file) in filtered_files.into_iter().enumerate() {
                let name_clone = file.name.clone();
                let is_dir = file.is_dir;
                let file_type = file.file_type;
                let target_subpath = if self.storage_current_path.is_empty() {
                    name_clone.clone()
                } else {
                    format!("{}/{}", self.storage_current_path.trim_matches('/'), name_clone)
                };

                let type_badge = container(
                    container(text(file_type.icon()).size(14))
                        .width(28)
                        .height(28)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(move |_| CustomStyles::storage_type_badge(file_type)),
                )
                .width(45)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center);

                let full_name = file.name.clone();
                let target_subpath_btn = target_subpath.clone();
                let name_clone_btn = name_clone.clone();

                let name_responsive = iced::widget::responsive(move |size| {
                    let usable_width = (size.width - 16.0).max(24.0);
                    let max_chars = (usable_width / 6.8).floor() as usize;
                    let char_count = full_name.chars().count();
                    let display_name = if max_chars >= 4 && char_count > max_chars {
                        let truncated: String =
                            full_name.chars().take(max_chars.saturating_sub(3)).collect();
                        format!("{}...", truncated)
                    } else {
                        full_name.clone()
                    };

                    if is_dir {
                        button(text(display_name).size(12))
                            .style(|_, status| CustomStyles::storage_name_button(true, status))
                            .padding(Padding::from([4, 8]))
                            .on_press(Message::StorageNavigateTo(target_subpath_btn.clone()))
                            .into()
                    } else {
                        button(text(display_name).size(12))
                            .style(|_, status| CustomStyles::storage_name_button(false, status))
                            .padding(Padding::from([4, 8]))
                            .on_press(Message::StorageOpenFile(name_clone_btn.clone()))
                            .into()
                    }
                });

                let name_element: Element<'_, Message> = self.tip(
                    container(name_responsive)
                        .width(Length::Fill)
                        .height(Length::Fixed(26.0))
                        .align_y(Alignment::Center),
                    &file.name,
                    tooltip::Position::Top,
                );

                // Centered row action buttons with ghost-pill styling
                let mut row_actions = row![].spacing(5).align_y(Alignment::Center);

                if is_dir {
                    let open_btn = button(text(Translations::storage_action_open(self.language)).size(10))
                        .style(|_, status| {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Primary,
                                status,
                            )
                        })
                        .padding(Padding::from([4, 8]))
                        .on_press(Message::StorageNavigateTo(target_subpath.clone()));
                    row_actions = row_actions.push(
                        self.tip(open_btn, Translations::tip_storage_open_item(self.language), tooltip::Position::Top)
                    );
                } else {
                    let open_name = file.name.clone();
                    let dl_name = file.name.clone();
                    let open_btn = button(text(Translations::storage_action_open(self.language)).size(10))
                        .style(|_, status| {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Primary,
                                status,
                            )
                        })
                        .padding(Padding::from([4, 8]))
                        .on_press(Message::StorageOpenFile(open_name));
                    row_actions = row_actions.push(
                        self.tip(open_btn, Translations::tip_storage_open_item(self.language), tooltip::Position::Top)
                    );

                    let dl_btn = button(text(Translations::storage_action_download(self.language)).size(10))
                        .style(|_, status| {
                            CustomStyles::storage_action_button(
                                crate::theme::StorageActionTone::Success,
                                status,
                            )
                        })
                        .padding(Padding::from([4, 8]))
                        .on_press(Message::StorageDownloadFile(dl_name));
                    row_actions = row_actions.push(
                        self.tip(dl_btn, Translations::tip_storage_download_item(self.language), tooltip::Position::Top)
                    );
                }

                let props_entry = file.clone();
                let details_btn = button(text(Translations::storage_action_details(self.language)).size(10))
                    .style(|_, status| {
                        CustomStyles::storage_action_button(
                            crate::theme::StorageActionTone::Neutral,
                            status,
                        )
                    })
                    .padding(Padding::from([4, 8]))
                    .on_press(Message::StorageShowFileProperties(props_entry));
                row_actions = row_actions.push(
                    self.tip(details_btn, Translations::tip_storage_details_item(self.language), tooltip::Position::Top)
                );

                let del_name = file.name.clone();
                let del_path = target_subpath.clone();
                let del_btn = button(text("🗑").size(10))
                    .style(|_, status| {
                        CustomStyles::storage_action_button(
                            crate::theme::StorageActionTone::Danger,
                            status,
                        )
                    })
                    .padding(Padding::from([4, 8]))
                    .on_press(Message::StorageRequestDeleteFile {
                        path: del_path,
                        is_dir,
                        name: del_name,
                    });
                row_actions = row_actions.push(
                    self.tip(del_btn, Translations::tip_storage_delete_item(self.language), tooltip::Position::Top)
                );

                let size_color = if is_dir {
                    Color::from_rgb8(71, 85, 105)
                } else {
                    CustomStyles::text_secondary()
                };

                let file_row = container(
                    row![
                        type_badge,
                        container(name_element).width(Length::Fill).align_x(Alignment::Start),
                        container(text(file.formatted_size()).size(11).color(size_color)).width(100).align_x(Alignment::Center),
                        container(text(&file.modified_str).size(11).color(Color::from_rgb8(130, 145, 165))).width(160).align_x(Alignment::Center),
                        container(row_actions).width(230).align_x(Alignment::Center),
                    ]
                    .align_y(Alignment::Center)
                    .padding(Padding {
                        top: 5.0,
                        right: 8.0,
                        bottom: 5.0,
                        left: 6.0,
                    }),
                )
                .width(Length::Fill)
                .style(move |_| CustomStyles::storage_file_row(row_idx));

                files_column = files_column.push(file_row);
            }
        }

        let scrollable_files = scrollable(files_column)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(6.0)
                    .scroller_width(6.0)
                    .margin(2.0)
                    .spacing(4.0),
            ))
            .style(CustomStyles::scrollbar_style())
            .height(Length::Fill);

        let files_area: Element<'_, Message> = if self.is_storage_drag_hovered {
            let item_hint = if let Some(name) = &self.hovered_drop_name {
                format!("📄 {}", name)
            } else {
                Translations::storage_drag_drop_hint(self.language).to_string()
            };

            let target_dest = if let Some(share) = &self.storage_active_share {
                if self.storage_current_path.is_empty() {
                    format!("📁 /{}/", share)
                } else {
                    format!("📁 /{}/{}", share, self.storage_current_path.trim_matches('/'))
                }
            } else {
                Translations::storage_drag_drop_select_share(self.language).to_string()
            };

            container(
                column![
                    text("📥").size(48),
                    text(Translations::storage_drag_drop_prompt(self.language))
                        .size(15)
                        .color(CustomStyles::accent_cyan()),
                    text(item_hint)
                        .size(12)
                        .color(CustomStyles::text_primary()),
                    text(format!("Target: {}", target_dest))
                        .size(11)
                        .color(CustomStyles::text_muted()),
                ]
                .spacing(10)
                .align_x(Alignment::Center),
            )
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(|_| CustomStyles::drop_zone_active())
            .into()
        } else {
            container(scrollable_files)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(Padding {
                    top: 0.0,
                    right: 2.0,
                    bottom: 2.0,
                    left: 2.0,
                })
                .style(|_| CustomStyles::drop_zone_idle())
                .into()
        };

        modal_body = modal_body.push(column![table_header, files_area].spacing(0));

        // Bottom bar: item summary and close button
        let total_count = self.storage_files.len();
        let dirs_count = self.storage_files.iter().filter(|f| f.is_dir).count();
        let regular_count = total_count - dirs_count;
        let summary_text = Translations::storage_summary_counts(total_count, dirs_count, regular_count, self.language);

        let footer = row![
            text(summary_text).size(11).color(CustomStyles::text_muted()),
            space::horizontal(),
            text(format!("💡 {}", Translations::storage_drag_drop_hint(self.language)))
                .size(10)
                .color(CustomStyles::accent_cyan()),
        ]
        .align_y(Alignment::Center);

        modal_body = modal_body.push(footer);

        let content = column![
            title_row,
            modal_body,
        ]
        .spacing(10);

        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| CustomStyles::modal_panel(CustomStyles::accent_cyan()))
            .into()
    }
}
