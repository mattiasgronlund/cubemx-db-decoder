#![allow(unused_variables)]
#![allow(unused_assignments)]
use crate::cubemx_db::Ip;
use crate::cubemx_db::Mcu;
use crate::{decode::Decode, Config, Error};
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
pub enum CubeMxDoc {
    Mcu(Mcu),
    Ip(Ip),
}
impl CubeMxDoc {
    pub fn decode_document(config: Config, doc_path: &String) -> Result<CubeMxDoc, Error> {
        let file = File::open(&doc_path)?;
        let file = BufReader::new(file);

        let mut file = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::UTF_8))
            .bom_override(true)
            .strip_bom(true)
            .build(file);
        let mut text = String::new();
        file.read_to_string(&mut text)
            .map_err(|io_error| Error::InvalidFile(io_error))?;
        CubeMxDoc::decode(config, &text)
    }

    pub fn decode(config: Config, text: &str) -> Result<CubeMxDoc, Error> {
        let document = roxmltree::Document::parse(text)
            .map_err(|roxmltree_error| Error::InvalideXML(roxmltree_error))?;

        match document.root_element().tag_name().name() {
            "Mcu" => Ok(CubeMxDoc::Mcu(Mcu::decode(
                config,
                document.root_element(),
            )?)),
            "IP" => Ok(CubeMxDoc::Ip(Ip::decode(config, document.root_element())?)),
            name => Err(Error::UnknownDocumentType {
                tag_name: name.to_string(),
                pos: document.text_pos_at(document.root_element().range().start),
            }),
        }
    }
}
