

use std::fmt::format;

use crate::diff::{Diff, ElementExt};

use eframe::{egui::{self, Ui, Widget}, epi};
use treexml::{Document, Element};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct DiffUiApp {
    // Example stuff:
    pub our_doc: Element,
    pub their_doc: Element,
    // pub our_xpaths: HashMap<String, Vec<Element>>,
    // pub their_xpaths: HashMap<String, Vec<Element>>,
    pub diff: Diff,
}

impl Default for DiffUiApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            our_doc: Element::new("foo"),
            their_doc: Element::new("bar"),
            // our_xpaths: HashMap::default(),
            // their_xpaths: HashMap::default(),
            diff: Diff::default(),
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
            // our_xpaths,
            // their_xpaths,
        } = self;

        egui::SidePanel::left("ours").show(ctx, |ui| {
            ui.heading("ours");

            draw_element(our_doc, diff, "ours", ui);
            // compare(their_doc, our_doc, ui);
        });

        // egui::SidePanel::left("theirs").show(ctx, |ui| {
        //     ui.heading("theirs");
        //     draw_element(their_doc, diff, "theirs", ui);
        // });
    }
}

fn draw_element(element: &mut Element, diff: &mut Diff, suffix: &str, ui: &mut Ui) {
    
    // egui::CollapsingHeader::new(&element.name)
    // .id_source(format!("{:?}{}", element.attributes, element.name))
    // .show(ui, |ui| {

        // ui.label(&element.name);


        for child in &mut element.children {
    
            ui.indent(format!("{:?}{}", child.attributes, child.name), |ui| {

                ui.horizontal(|ui| {
                    ui.label(&child.name);
                    edit_element(child, ui);
                });
                draw_element(child, diff, suffix, ui);
            });
            
            // egui::CollapsingHeader::new(&child.name)
            // .id_source(format!("{:?}{}", child.attributes, child.name))
            // .show(ui, |ui| {
            //     edit_element(child, ui);
            //     // draw_element(child, diff, suffix, ui);
            //     draw_element(child, diff, suffix, ui);
            // });
        }
    // });

}


fn val_edit(s: &mut String, ui: &mut Ui) {
    
    if s.len() > 20 {
        ui.text_edit_multiline(s);
        
    } else {
        if let Ok(f) = s.parse::<f32>() {
            let mut m_f = f;
            if egui::DragValue::new(&mut m_f).ui(ui).changed() {
                *s = m_f.to_string();
            }
        } else {

            ui.text_edit_singleline(s);
        }
    }
}


fn edit_element(element: &mut Element, ui: &mut Ui) {
    // Edit element's text
    if let Some(text) = &mut element.text {
       // ui.text_edit_singleline(text);
       val_edit(text, ui);

    }

    // iterate all attributes
    for (k, v) in &mut element.attributes {
        ui.horizontal(|ui| {
            ui.label(k);

            if let Ok(int) = v.parse::<i32>() {
                let mut m_int = int;
                egui::DragValue::new(&mut m_int).ui(ui);
            }

            if let Ok(f) = v.parse::<f32>() {
                let mut m_f = f;
                egui::DragValue::new(&mut m_f).ui(ui);
            }

   
            //ui.text_edit_singleline(v);
        });
    }
}

// fn compare(theirs: &mut Element, ours: &mut Element, ui: &mut Ui) {
//     draw_element(theirs, ui);

//     if theirs != ours {

//         draw_element(ours, ui);
//     }

// }
