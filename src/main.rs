#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use eframe::{egui};
use result_generator::Colours::{Green, Grey, Red, Yellow};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use crate::egui::{Context, FontId, RichText, TextStyle, FontFamily, Color32, Ui};

mod result_generator;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bestle",
        options,
        Box::new(|cc| Box::new(Bestle::new(cc))),
    );
}

impl eframe::App for Bestle {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            egui::widgets::global_dark_light_mode_switch(ui);
            Self::add_heading(ui);
            self.add_basic_inputs(ui);

            Self::add_separator(ui);

            self.add_colour_inputs(ui);

            Self::add_separator(ui);

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.params.winning, "Always winning");
            });

            Self::add_bigger_spacing(ui);

            ui.horizontal(|ui| {
                if ui.button("Copy Result To Clipboard").clicked() {
                    self.done = false;
                    generate_result(&mut self.params);
                    self.done = true;
                }
            });

            Self::add_small_spacing(ui);
            if self.done {
                ui.label(RichText::new("Done").text_style(get_style(ERROR_STYLE)).color(Color32::GREEN));
            }
        });
    }
}

fn generate_result(params: &mut Parameters) {
    if !params.red && !params.grey && !params.yellow {
        return;
    }
    let result: String = result_generator::create_block_result(params);
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    clipboard.set_contents(result.to_owned()).unwrap();
}

impl Bestle {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        configure_text_styles(&cc.egui_ctx);
        Bestle::default()
    }

    fn add_heading(ui: &mut Ui) {
        ui.heading("Bestle");
        Self::add_bigger_spacing(ui);
    }

    fn add_bigger_spacing(ui: &mut Ui) {
        ui.add_space(15.0);
    }

    fn add_basic_inputs(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.monospace("Name  ");
            ui.text_edit_singleline(&mut self.params.name);
        });
        Self::add_small_spacing(ui);
        ui.horizontal(|ui| {
            ui.monospace("Width ");
            ui.add(egui::widgets::Slider::new(&mut self.params.width, 1..=12).text("Blocks"));
        });
        Self::add_small_spacing(ui);
        ui.horizontal(|ui| {
            ui.monospace("Height");
            ui.add(egui::widgets::Slider::new(&mut self.params.height, 1..=12).text("Blocks"));
        });
    }

    fn add_separator(ui: &mut Ui) {
        Self::add_small_spacing(ui);
        ui.separator();
        Self::add_small_spacing(ui);
    }

    fn add_colour_inputs(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Colours").text_style(get_style(SUBHEADING_STYLE)));
        Self::add_small_spacing(ui);
        if !self.params.yellow && !self.params.red && !self.params.grey {
            ui.label(RichText::new("Select at least one colour!").text_style(get_style(ERROR_STYLE)).color(Color32::RED));
            Self::add_small_spacing(ui);
        }
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.params.red, "Red");
        });
        Self::add_small_spacing(ui);
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.params.yellow, "Yellow");
        });
        Self::add_small_spacing(ui);
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.params.grey, "Grey");
        });
    }

    fn add_small_spacing(ui: &mut Ui) {
        ui.add_space(7.0);
    }
}

const ERROR_STYLE: &'static str = "Error";
const SUBHEADING_STYLE: &'static str = "Subheading";

fn configure_text_styles(ctx: &Context) {
    use FontFamily::Proportional;
    use FontFamily::Monospace;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (TextStyle::Body, FontId::new(18.0, Monospace)),
        (TextStyle::Monospace, FontId::new(18.0, Monospace)),
        (TextStyle::Button, FontId::new(18.0, Monospace)),
        (TextStyle::Small, FontId::new(12.0, Monospace)),
        (TextStyle::Name(Arc::from(SUBHEADING_STYLE)), FontId::new(22.0, Monospace)),
        (TextStyle::Name(Arc::from(ERROR_STYLE)), FontId::new(18.0, Monospace)),
    ].into();
    ctx.set_style(style);
}

fn get_style(style: &str) -> TextStyle {
    TextStyle::Name(Arc::from(style))
}

pub struct Bestle {
    params: Parameters,
    done: bool
}

#[derive(Debug)]
pub struct Parameters {
    name: String,
    width: u8,
    height: u8,
    winning: bool,
    yellow: bool,
    red: bool,
    grey: bool
}

impl Default for Bestle {
    fn default() -> Self {
        Self {
            done: false,
            params: Parameters {
                name: "Bestle".to_owned(),
                width: 5,
                height: 5,
                winning: true,
                yellow: true,
                red: false,
                grey: true
            }
        }
    }
}
