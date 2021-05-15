// use xmltree::{Element};
use anyhow::{bail, Error, Result};
use std::{fs::File, path::PathBuf};
use treexml::{Document, Element};

pub fn load(p: &PathBuf) -> Result<Element, Error> {
    let f = File::open(&p)?;
    match Document::parse(f) {
        Ok(doc) => Ok(doc.root.expect("Document has no root element")),
        Err(e) => bail!(format!("{}", e)),
    }
}
