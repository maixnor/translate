use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Serialize, Default, PartialEq)]
enum Language {
    #[default]
    Klingon,
    Yoda,
}

impl Language {
    fn as_string(&self) -> &str {
        match self {
            Language::Klingon => "Klingon",
            Language::Yoda => "Yoda Speak",
        }
    }

    fn as_query(&self) -> &str {
        match self {
            Language::Klingon => "klingon",
            Language::Yoda => "yoda",
        }
    }
}

#[derive(Deserialize, Serialize)]
struct TranslationResponse {
    contents: Translation,
}

#[derive(Deserialize, Serialize)]
struct Translation {
    translated: String,
    text: String,
    translation: String,
}

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct App {
    dropped_files: Vec<egui::DroppedFile>,
    input_text: String,
    output_text: String,
    language: Language,
}

impl App {
    /// Called once before the first frame.
    #[allow(dead_code)]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn translate(&mut self) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let res = rt.block_on(async {
            let query = "https://api.funtranslations.com/translate/".to_owned()
                + self.language.as_query()
                + "?text="
                + &self.input_text;

            let returned = reqwest::get(query).await.unwrap().text().await.unwrap();
            println!("{}", returned);
            returned
        });

        let response: TranslationResponse = serde_json::from_str(&res).expect("");
        self.output_text = response.contents.translated;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        side_panel(ctx, &mut self.language);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tranlate something!");

            ui.text_edit_multiline(&mut self.input_text);
            if ui
                .button("Translate to ".to_owned() + self.language.as_string())
                .clicked()
            {
                self.translate();
            }

            ui.separator();
            ui.label(self.output_text.to_string())
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     match dropped_files.first() {
        //         Some(file) => display_image(ui, file),
        //         None => {
        //             ui.label("drag-and-drop image files to convert to ascii art!");
        //         }
        //     };
        // });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Working on File:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                write!(text, "\n{}", path.display()).ok();
            } else if !file.mime.is_empty() {
                write!(text, "\n{}", file.mime).ok();
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

fn side_panel(ctx: &egui::Context, language: &mut Language) {
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Language");

        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.vertical(|ui| {
                ui.radio_value(language, Language::Klingon, "Klingon");
                ui.radio_value(language, Language::Yoda, "Yoda Speak");
            });
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to(
                    "eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });
        });
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            egui::warn_if_debug_build(ui);
        })
    });
}
