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
            ui.label("Это просто надпись");
            ui.hyperlink("https://github.com/emilk/egui");
            ui.text_edit_singleline(&mut self.my_string);
            if ui.button("Очистить").clicked() { self.my_string = "".to_string() }
            ui.add(egui::Slider::new(&mut self.my_f32, 0.0..=100.0));
            ui.add(egui::DragValue::new(&mut self.my_f32));

            ui.checkbox(&mut self.my_boolean, "Checkbox");

            
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.my_enum, Enum::First, "First");
                ui.radio_value(&mut self.my_enum, Enum::Second, "Second");
                ui.radio_value(&mut self.my_enum, Enum::Third, "Third");
            });

            ui.separator();

            // ui.image(my_image, [640.0, 480.0]);

            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
            });
        });

        egui::Area::new("my_area")
        .show(ctx, |ui| { 
            egui::Frame::none()
            .fill(egui::Color32::from(self.my_color))
            .inner_margin(Margin::same(5.0))
            .stroke(egui::Stroke::new(3.0, self.my_color2))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
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
    }
}
