use eframe::{ egui, Renderer };
use eframe::egui::{ style::Margin, color::Hsva };

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, 
        Box::new(|_cc| Box::new(MyApp::new(_cc))));
}

struct MyApp {
    name: String,
    age: u32,
    my_string: String,
    my_f32: f32,
    my_boolean: bool,
    my_enum: Enum,
    my_color: Hsva,
    my_color2: Hsva,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            my_string: String::new(),
            my_f32: 0.0,
            my_boolean: false,
            my_enum: Enum::First,
            my_color: Hsva::new(0.5, 0.5, 0.5, 1.0),
            my_color2: Hsva::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(PartialEq)]
enum Enum { First, Second, Third }

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0; // remove spacing between widgets
                // `radio_value` also works for enums, integers, and more.
                ui.radio_value(&mut self.my_boolean, false, "Off");
                ui.radio_value(&mut self.my_boolean, true, "On");
            });

            ui.group(|ui| {
                ui.label("Within a frame");
                ui.set_min_height(200.0);
            });

            // A `scope` creates a temporary [`Ui`] in which you can change settings:
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                ui.style_mut().wrap = Some(false);

                ui.label("This text will be red, monospace, and won't wrap to a new line");
            }); // the temporary settings are reverted here
        });

        egui::Area::new("my_area")
        .show(ctx, |ui| { 
            egui::Frame::none()
            .fill(egui::Color32::from(self.my_color))
            .inner_margin(Margin::same(5.0))
            .stroke(egui::Stroke::new(3.0, self.my_color2))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.collapsing("Heading", |ui| { 
                    ui.add(egui::Slider::new(&mut self.my_f32, 0.0..=100.0));
                    ui.add(egui::DragValue::new(&mut self.my_f32));
                    ui.horizontal(|ui| {
                        egui::color_picker::color_edit_button_hsva(ui, 
                            &mut self.my_color, egui::color_picker::Alpha::BlendOrAdditive);
                        egui::color_picker::color_edit_button_hsva(ui, 
                            &mut self.my_color2, egui::color_picker::Alpha::BlendOrAdditive);
                    });
                });
            });
        });
    }
}
