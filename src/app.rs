use image::GenericImageView;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    dropped_files: Vec<egui::DroppedFile>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            dropped_files: vec![],
        }
    }
}

impl App {
    /// Called once before the first frame.
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { dropped_files } = self;

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

        // side_panel(ctx, filename);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            match dropped_files.first() {
                Some(file) => display_image_bytes(ui, &file.bytes.as_ref().unwrap().to_vec()),
                None => {
                    ui.label("drag-and-drop image files to convert to ascii art!");
                }
            };

            egui::warn_if_debug_build(ui);
        });

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

fn side_panel(ctx: &egui::Context, filename: &mut String) {
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Side Panel");

        ui.horizontal(|ui| {
            ui.label("Write something: ");
            ui.text_edit_singleline(filename);
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
    });
}

fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn max(a: u32, b: u32) -> u32 {
    match a < b {
        true => b,
        false => a,
    }
}

fn display_image_bytes(ui: &mut egui::Ui, bytes: &[u8]) {
    use std::io::Cursor;

    let reader = image::io::Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap();

    match reader.decode() {
        Ok(img) => ui.monospace(convert_into_ascii(img)),
        Err(_) => ui.label("Not a valid Image!"),
    };
}

fn display_image(ui: &mut egui::Ui, picked_path: &String) {
    match image::open(picked_path) {
        Ok(img) => ui.monospace(convert_into_ascii(img)),
        Err(_) => ui.label("Not a valid Image!"),
    };
}

fn convert_into_ascii(img: image::DynamicImage) -> String {
    let mut ascii = "".to_string();
    let (width, height) = img.dimensions();
    let scale = max(height / 75, width / 150);
    for y in 0..height / scale {
        for x in 0..width / scale {
            if y * scale % (scale * 2) == 0 && x * scale % scale == 0 {
                let pix = img.get_pixel(x * scale, y * scale);
                let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                if pix[3] == 0 {
                    intent = 0;
                }
                ascii.push_str(get_str_ascii(intent));
            }
        }
        if y * scale % (scale * 2) == 0 {
            ascii.push_str("\n");
        }
    }
    ascii
}
