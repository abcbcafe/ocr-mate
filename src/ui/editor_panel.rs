pub struct EditorPanel {
    /// Show line numbers
    show_line_numbers: bool,

    /// Word wrap
    word_wrap: bool,
}

impl EditorPanel {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
            word_wrap: true,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, text: &mut String) {
        ui.vertical(|ui| {
            // Editor toolbar
            ui.horizontal(|ui| {
                ui.label("OCR Results:");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .selectable_label(self.word_wrap, "Word Wrap")
                        .clicked()
                    {
                        self.word_wrap = !self.word_wrap;
                    }

                    if ui
                        .selectable_label(self.show_line_numbers, "Line Numbers")
                        .clicked()
                    {
                        self.show_line_numbers = !self.show_line_numbers;
                    }
                });
            });

            ui.separator();

            // Main text editor
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                    let mut layout_job = egui::text::LayoutJob::default();

                    if self.show_line_numbers {
                        // Add line numbers
                        for (i, line) in text.lines().enumerate() {
                            // Line number
                            layout_job.append(
                                &format!("{:4} ", i + 1),
                                0.0,
                                egui::TextFormat {
                                    color: ui.style().visuals.weak_text_color(),
                                    ..Default::default()
                                },
                            );

                            // Line content
                            layout_job.append(
                                line,
                                0.0,
                                egui::TextFormat {
                                    color: ui.style().visuals.text_color(),
                                    ..Default::default()
                                },
                            );

                            layout_job.append(
                                "\n",
                                0.0,
                                egui::TextFormat::default(),
                            );
                        }
                    } else {
                        layout_job.append(
                            text,
                            0.0,
                            egui::TextFormat {
                                color: ui.style().visuals.text_color(),
                                ..Default::default()
                            },
                        );
                    }

                    ui.fonts(|f| f.layout_job(layout_job))
                };

                let text_edit = egui::TextEdit::multiline(text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(25)
                    .code_editor();

                let text_edit = if self.word_wrap {
                    text_edit
                } else {
                    text_edit.layouter(&mut layouter)
                };

                ui.add(text_edit);
            });

            // Footer with stats
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Characters: {} | Lines: {} | Words: {}",
                    text.len(),
                    text.lines().count(),
                    text.split_whitespace().count()
                ));
            });
        });
    }
}

impl Default for EditorPanel {
    fn default() -> Self {
        Self::new()
    }
}
