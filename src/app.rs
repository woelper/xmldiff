use crate::diff::{Diff, ElementExt};

use eframe::{
    egui::{self, Ui},
    epi,
};
use treexml::{Document, Element};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct DiffUiApp {
    // Example stuff:
    pub our_doc: Element,
    pub their_doc: Element,
    pub diff: Diff,
}

impl Default for DiffUiApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            our_doc: Element::new("foo"),
            their_doc: Element::new("bar"),
            diff: Diff::new(&Element::new("foo"), &Element::new("bar")),
        }
    }
}

impl epi::App for DiffUiApp {
    fn name(&self) -> &str {
        "xmldiff"
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
        let DiffUiApp {
            our_doc,
            their_doc,
            diff,
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("theirs");
            draw_element(their_doc, diff, "theirs", ui);
            // compare(their_doc, our_doc, ui);
        });

        egui::SidePanel::left("side_panel2").show(ctx, |ui| {
            ui.heading("ours");
            draw_element(our_doc, diff, "ours", ui);
        });
    }
}

fn draw_element(element: &mut Element, diff: &Diff, suffix: &str, ui: &mut Ui) {
    for child in &mut element.children {
        let mut d = ("".to_string(), "".to_string());

        // fill up default with first value pair
        for (k, v) in &child.attributes {
            d.0 = k.clone();
            d.1 = v.clone();
        }

        let d_s = "".to_string();

        let name = format!(
            "{} {} {} {} {:?} {}",
            &child.name,
            d.0,
            d.1,
            child.text.as_ref().unwrap_or(&d_s),
            child.children.len(),
            suffix
        );

        // ui.label(format!("is id used? {}", diff.is_id_in_theirs(&child.id())));
        ui.label(format!("path? {:?}", diff.xpath_from_id(&child.id(), "ours")));
        // ui.label(format!("elements? {:?}", diff.elements_from_id(&child.id()).unwrap().len()));

        egui::CollapsingHeader::new(&child.name)
            .id_source(&name)
            .show(ui, |ui| {
                edit_element(child, ui);
                draw_element(child, diff, suffix, ui);
            });
    }
}

fn edit_element(element: &mut Element, ui: &mut Ui) {
    if let Some(text) = &mut element.text {
        ui.text_edit_singleline(text);

    }

    for (k, v) in &mut element.attributes {
        ui.horizontal(|ui| {
            ui.label(k);
            ui.text_edit_singleline(v);
        });
    }
}

// fn compare(theirs: &mut Element, ours: &mut Element, ui: &mut Ui) {
//     draw_element(theirs, ui);

//     if theirs != ours {

//         draw_element(ours, ui);
//     }

// }
