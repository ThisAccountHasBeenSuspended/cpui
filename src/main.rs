#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Color32, RichText};

mod cpu;
mod enums;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([340.0, 240.0])
        .with_maximize_button(false)
        .with_resizable(false);
    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    eframe::run_native(
        "CPU Information",
        options,
        Box::new(|__| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[inline]
fn get_family_color(family: &enums::Family) -> (u8, u8, u8) {
    match family {
        enums::Family::Unknown => (0, 0, 0),
        enums::Family::Intel(_) => (0, 113, 197),
        enums::Family::Amd(_) => (237, 28, 36),
    }
}

#[inline]
fn family_label(text: impl Into<String>, color: (u8, u8, u8)) -> RichText {
    RichText::new(text.into())
        .color(
            Color32::from_rgb(
                color.0, 
                color.1, 
                color.2
            )
        )
}

struct MyApp {
    // Information stuff
    cpu_info: cpu::Info,
    cpu_vendor: String,
    cpu_frequencies: [u32; 3],
    cpu_features_name_list: String,

    // Window stuff
    cpu_text_color: (u8, u8, u8),
}

impl Default for MyApp {
    fn default() -> Self {
        use strum::IntoEnumIterator;

        let cpu_family = enums::Family::default();
        let cpu_info = cpu::Info::default();
        let cpu_vendor_vec = cpu::vendor().unwrap_or("Not Supported".into());
        let cpu_vendor = String::from_utf8(cpu_vendor_vec).unwrap();
        let cpu_frequencies = cpu::frequencies(cpu_family.max_lvl()).unwrap_or_default();
        let cpu_features_name_list = enums::Feature::iter()
            .filter(|f| cpu::has_feature(&cpu_info.features, f.clone() as u8))
            .map(|f| f.as_ref().to_string().to_uppercase())
            .collect::<Vec<String>>()
            .join(", ");

        let cpu_text_color = get_family_color(&cpu_family);

        Self {
            cpu_info,
            cpu_vendor,
            cpu_frequencies,
            cpu_features_name_list,

            cpu_text_color,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.cpu_vendor);
            
            egui::Grid::new("unique_id_1").show(ui, |ui| {
                ui.group(|ui| {
                    ui.label("Family:");
                    ui.label(family_label(self.cpu_info.family_id.to_string(), self.cpu_text_color));
                });
                ui.group(|ui| {
                    ui.label("Model:");
                    ui.label(family_label(format!("{:X}", self.cpu_info.model), self.cpu_text_color));
                });
                ui.group(|ui| {
                    ui.label("Stepping:");
                    ui.label(family_label(self.cpu_info.stepping.to_string(), self.cpu_text_color));
                });
                ui.end_row();

                ui.group(|ui| {
                    ui.label("Ext. Family:");
                    ui.label(family_label(self.cpu_info.extended_family_id.to_string(), self.cpu_text_color));
                });
                ui.group(|ui| {
                    ui.label("Ext. Model:");
                    ui.label(family_label(format!("{:X}", self.cpu_info.extended_model_id), self.cpu_text_color));
                });
                ui.group(|ui| {
                    ui.label("Processor Type:");
                    ui.label(family_label(format!("{:X}", self.cpu_info.processor_type), self.cpu_text_color));
                });
                ui.end_row();
            });

            ui.heading("Frequencies");
            egui::Grid::new("unique_id_2").show(ui, |ui| {
                ui.group(|ui| {
                    ui.label("Base:");
                    ui.label(family_label(format!("{:.3}MHz", self.cpu_frequencies[0] as f64 / 1000.0), self.cpu_text_color));
                });
                ui.group(|ui| {
                    ui.label("Max:");
                    ui.label(family_label(format!("{:.3}MHz", self.cpu_frequencies[1] as f64 / 1000.0), self.cpu_text_color));
                });
                ui.group(|ui| {
                    ui.label("Bus:");
                    ui.label(family_label(format!("{:.3}MHz", self.cpu_frequencies[2] as f64 / 1000.0), self.cpu_text_color));
                });
                ui.end_row();
            });

            ui.heading("Features");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(family_label(&self.cpu_features_name_list, self.cpu_text_color));
            });
        });
    }
}
