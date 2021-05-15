use std::{alloc::Global, path::PathBuf};

use eframe::{
    egui::{self, Ui},
    epi,
};
use treexml::{Document, Element};
use xmltree::XMLNode;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    // Example stuff:
    pub our_doc: Element,
    pub their_doc: Element,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            our_doc: Element::new("foo"),
            their_doc: Element::new("bar"),
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
            our_doc,
            their_doc,
        } = self;

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("theirs");
            draw_element(their_doc, ui);
        });


        egui::SidePanel::left("side_panel2", 200.0).show(ctx, |ui| {
            ui.heading("ours");
            draw_element(our_doc, ui);
        });


    }
}


fn draw_element(element: &mut Element, ui: &mut Ui) {
    for child in &mut element.children {
        let mut d = ("".to_string(), "".to_string());

        // fill up default with first value pair
        for (k, v) in &child.attributes {
            d.0 = k.clone();
            d.1 = v.clone();
        }

        let d_s = "".to_string();

        ui.collapsing(
            &format!(
                "{} {} {} {}",
                &child.name,
                d.0,
                d.1,
                child.text.as_ref().unwrap_or(&d_s)
            ),
            |ui| {
                draw_element(child, ui);
            },
        );
    }
}
