// use xmltree::{Element};
use anyhow::{bail, Error, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, fs::File, path::PathBuf};
use treexml::{Document, Element};

pub fn load(p: &PathBuf) -> Result<Element, Error> {
    let f = File::open(&p)?;
    match Document::parse(f) {
        Ok(doc) => Ok(doc.root.expect("Document has no root element")),
        Err(e) => bail!(format!("{}", e)),
    }
}

pub trait ElementExt {
    fn id(&self) -> String {
        unimplemented!()
    }
}

impl ElementExt for Element {
    fn id(&self) -> String {
        calculate_hash(&format!(
            "{}{:?}{:?}",
            self.name, self.attributes, self.children
        ))
        .to_string()
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// a map of documents to diff
type Documents = HashMap<String, Element>;

#[derive(Debug, Default)]
pub struct Diff {
    // pub theirs: Element,
    // pub ours: Element,
    pub documents: Documents,
    /// xpath to similar elements
    pub xpaths: HashMap<String, HashMap<String, Vec<Element>>>,
    /// id -> xpath map
    pub ids: HashMap<String, HashMap<String, String>>,
}

// #[derive(Debug)]
// pub struct Connection {
//     pub path: String,
//     pub ours: Element,
//     pub tree: HashMap<String, Vec<Element>>,
// }

// type ClashMap = HashMap<String, Clash>;
impl Diff {
    pub fn new(theirs: &Element, ours: &Element) -> Self {
        Self {
            // theirs: theirs.clone(),
            // ours: ours.clone(),
            documents: HashMap::default(),
            xpaths: HashMap::default(),
            ids: HashMap::default(),
        }
    }

    pub fn add_doc(&mut self, name: &str, doc: Element) {
        self.documents.insert(name.to_string(), doc);
    }

    pub fn is_id_in_theirs(&self, id: &str, index: &str) -> bool {
        match self.elements_from_id(id, index) {
            Some(elements) => elements.len() > 1,
            None => false,
        }
    }

    pub fn elements_from_id(&self, id: &str, index: &str) -> Option<&Vec<Element>> {
        self.ids.get(index)?.get(id).map(|id| self.xpaths.get(index)?.get(id)).flatten()
    }

    pub fn xpath_from_id(&self, id: &str, index: &str) -> Option<&String> {
        self.ids.get(index)?.get(id)
    }

    pub fn read(&mut self) -> Option<()>{
        let ours = self.documents.get_mut("ours")?.clone();
        let theirs = self.documents.get_mut("theirs")?.clone();
        self.recurse(&theirs, "", "theirs`");
        self.recurse(&ours, "", "ours");
        // self.recurse(self.documents.get_mut("theirs")?, "", "ours");
        Some(())
    }

    pub fn recurse(&mut self, element: &Element, parent: &str, index: &str) -> Option<()> {
        for child in &element.children {
            let p = format!("{}/{}", parent, element.name);
            let path = format!("{}/{}", p, child.name);
            let e = self.xpaths.get_mut(index)?.entry(path.clone()).or_default();
            self.ids.get_mut(index)?.insert(child.id(), format!("{}[{}]", path, e.len()));
            e.push(child.clone());
            self.recurse(child, &p, index);
        }
        Some(())

    }
}
