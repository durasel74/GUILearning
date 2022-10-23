use eframe::{ egui };
use egui::{Response, style::Margin};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, 
        Box::new(|_cc| Box::new(MyApp::new(_cc))));
}

#[derive(PartialEq)]
enum Lessons { 
    Less1,
    Less2,
    Less3,
    Less4,
}

struct MyApp {
    current_lesson: Lessons,
    test_str: String,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn draw_lesson1(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("main_top_panel").resizable(true).show(ctx, |ui| {
            // egui::SidePanel::left("left_top_panel").resizable(true).show(ctx, |ui| {
            //     ui.collapsing("CollapsingHeader", |ui| {
            //         ui.label("Body"); 
            //     }).header_response;
            // });
            
            // egui::SidePanel::right("right_top_panel").resizable(true).show(ctx, |ui| {
            //     ui.collapsing("CollapsingHeader2", |ui| {
            //         ui.label("Body"); 
            //     }).header_response;
            // });
            
            ui.text_edit_multiline(&mut self.test_str);
        });

        egui::TopBottomPanel::bottom("main_bottom_panel").resizable(true).show(ctx, |ui| {
            ui.collapsing("CollapsingHeader3", |ui| {
                ui.label("Body"); 
            }).header_response;

            ui.text_edit_multiline(&mut self.test_str);
        });
    }

    fn draw_lesson2(ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.button("text2");
    }

    fn draw_lesson3(ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.button("text3");
    }

    fn draw_lesson4(ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.button("text4");
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_lesson: Lessons::Less1,
            test_str: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame::none()
            .inner_margin(Margin::same(5.0))
            .fill(egui::Color32::DARK_GRAY);

        egui::SidePanel::left("left_side_bar").resizable(false).frame(frame).show(ctx, |ui| {
            ui.selectable_value(&mut self.current_lesson, Lessons::Less1, "Контейнеры и компоновка");
            ui.selectable_value(&mut self.current_lesson, Lessons::Less2, "Второй урок");
            ui.selectable_value(&mut self.current_lesson, Lessons::Less3, "Третий урок");
            ui.selectable_value(&mut self.current_lesson, Lessons::Less4, "Четвертый урок");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_lesson {
                Lessons::Less1 => self.draw_lesson1(ctx),
                Lessons::Less2 => Self::draw_lesson2(ctx, ui),
                Lessons::Less3 => Self::draw_lesson3(ctx, ui),
                Lessons::Less4 => Self::draw_lesson4(ctx, ui),
            }
        });
    }
}
