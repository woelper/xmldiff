// use xmltree::{Element};
use std::{fs::File, path::PathBuf};
use anyhow::{Result, Error};
use treexml::{Document, Element};

// pub fn load (p: &PathBuf) -> Result<Element, Error>{
//     let d = File::open(&p)?;
//     Element::parse(d)
//     .map_err(|e| Error::from(e))
    
//     // Ok(s)
//     // println!("{:#?}", root_element);
//     // {
//     //     // get first `name` element
//     //     let name = root_element.get_mut_child("name").expect("Can't find name element");
//     //     name.attributes.insert("suffix".to_owned(), "mr".to_owned());
//     // }
//     // root_element.write(File::create("result.xml").unwrap());
// }

pub fn load (p: &PathBuf) -> Result<Element, Error>{
    let f = File::open(&p)?;
    let d =  Document::parse(f).unwrap();
    Ok(d.root.unwrap())
    
  //t.write(File::create("result.xml").unwrap());
}