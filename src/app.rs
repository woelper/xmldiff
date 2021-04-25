use std::{alloc::Global, path::PathBuf};

use eframe::{egui::{self, Ui}, epi};
use xmltree::XMLNode;
use treexml::{Document, Element};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    // Example stuff:
    pub theirs: PathBuf,
    pub value: f32,
    // pub our_doc: xmltree::Element,
    pub our_doc: Element,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            theirs: PathBuf::new(),
            value: 2.7,
            // our_doc: xmltree::Element::new("")
            our_doc: Element::new("foo")
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let TemplateApp {
            theirs: label,
            value,
            our_doc
        } = self;


        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Side Panel");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("title");
    
            egui::warn_if_debug_build(ui);

            // for e in &our_doc.children {
            //     ui.collapsing(&e.as_element().unwrap().name, |ui| {

            //     });
            // }

            // node(&our_doc.children, ui);

            draw_element(&our_doc, ui);

            ui.separator();

   
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

// fn node(children: &Element, ui: &mut Ui) {
//     for (i, child) in children.iter().enumerate() {
//         ui.collapsing(&format!("{}##{}", &child.as_element().unwrap().name, i), |ui| {
    
//         });
//     }
// }

fn draw_element(element: &Element, ui: &mut Ui) {
    for (i, child) in element.children.iter().enumerate() {
        ui.collapsing(&format!("{}##{}", &child.name, i), |ui| {
            draw_element(child, ui);
        });
    }
}