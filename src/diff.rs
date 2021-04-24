use xmltree::{Element, ParseError};
use std::{fs::File, path::PathBuf};
use anyhow::{Result, Error};

pub fn load (p: &PathBuf) -> Result<Element, Error>{
    let d = File::open(&p)?;
    Element::parse(d)
    .map_err(|e| Error::from(e))
    
    // Ok(s)
    // println!("{:#?}", root_element);
    // {
    //     // get first `name` element
    //     let name = root_element.get_mut_child("name").expect("Can't find name element");
    //     name.attributes.insert("suffix".to_owned(), "mr".to_owned());
    // }
    // root_element.write(File::create("result.xml").unwrap());
    
}