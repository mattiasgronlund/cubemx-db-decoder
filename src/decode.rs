use crate::{error::Unexpected, Config, Error};
use roxmltree::Node;
use std::collections::HashMap;

/// Decode trait allows CubeMX DB objects to be decoded from XML elements.
pub trait Decode {
    /// Object returned by parse method
    type Object;

    /// Parse an XML/CubeMX DB element into it's corresponding `Object`.
    fn decode(config: Config, node: Node) -> Result<Self::Object, Error>;
}

pub(crate) struct AttributeMap<'a> {
    map: HashMap<&'a str, &'a roxmltree::Attribute<'a>>,
    node: roxmltree::Node<'a, 'a>,
    config: Config,
}

impl<'a> AttributeMap<'a> {
    pub(crate) fn from(node: roxmltree::Node<'a, 'a>, config: Config) -> AttributeMap<'a> {
        let mut result = AttributeMap {
            map: HashMap::new(),
            node,
            config,
        };

        for attribute in node.attributes() {
            result.insert(attribute);
        }
        result
    }

    pub fn insert(&mut self, attribute: &'a roxmltree::Attribute<'a>) {
        self.map.insert(attribute.name(), attribute);
    }

    pub(crate) fn take_required(&mut self, name: &str) -> Result<String, Error> {
        Ok(self
            .map
            .remove(name)
            .ok_or(Error::MissingAttribute {
                attribute_name: name.to_string(),
                node_name: self.node.tag_name().name().to_string(),
                pos: self.node.document().text_pos_at(self.node.range().start),
            })?
            .value()
            .to_string())
    }

    // pub(crate) fn take_required_bool(&mut self, name: &str) -> Result<bool, Error> {
    //     let attribute = self.map.remove(name).ok_or(Error::MissingAttribute {
    //         attribute_name: name.to_string(),
    //         node_name: self.node.tag_name().name().to_string(),
    //         pos: self.node.document().text_pos_at(self.node.range().start),
    //     })?;
    //     attribute
    //         .value()
    //         .parse::<bool>()
    //         .map_err(|e| Error::invalid_attribute(&self.node, attribute, Box::from(e)))
    // }

    pub(crate) fn take_required_f32(&mut self, name: &str) -> Result<f32, Error> {
        let attribute = self.map.remove(name).ok_or(Error::MissingAttribute {
            attribute_name: name.to_string(),
            node_name: self.node.tag_name().name().to_string(),
            pos: self.node.document().text_pos_at(self.node.range().start),
        })?;
        attribute
            .value()
            .parse::<f32>()
            .map_err(|e| Error::invalid_attribute(&self.node, attribute, Box::from(e)))
    }

    pub(crate) fn take_required_i32(&mut self, name: &str) -> Result<i32, Error> {
        let attribute = self.map.remove(name).ok_or(Error::MissingAttribute {
            attribute_name: name.to_string(),
            node_name: self.node.tag_name().name().to_string(),
            pos: self.node.document().text_pos_at(self.node.range().start),
        })?;
        attribute
            .value()
            .parse::<i32>()
            .map_err(|e| Error::invalid_attribute(&self.node, attribute, Box::from(e)))
    }

    pub(crate) fn take_optional_bool(&mut self, name: &str) -> Result<Option<bool>, Error> {

        match self.map
            .remove(name) {
                Some(attribute) => {
                    let value = attribute
                        .value()
                        .parse::<bool>()
                        .map_err(|e| Error::invalid_attribute(&self.node, attribute, Box::from(e)))?;
                    Ok(Some(value))
                }
                None => Ok(None)
            }
    }

    // pub(crate) fn take_optional_i64(&mut self, name: &str) -> Result<Option<i64>, Error> {

    //     match self.map
    //         .remove(name) {
    //             Some(attribute) => {
    //                 let value = attribute
    //                     .value()
    //                     .parse::<i64>()
    //                     .map_err(|e| Error::invalid_attribute(&self.node, attribute, Box::from(e)))?;
    //                 Ok(Some(value))
    //             }
    //             None => Ok(None)
    //         }
    // }

    pub(crate) fn take_optional(&mut self, name: &str) -> Option<String> {
        self.map
            .remove(name)
            .map(|attribute| attribute.value().to_string())
    }

    pub(crate) fn report_unexpected_if_not_empty(&self) -> Result<(), Error> {
        for attribute in self.map.values() {
            Unexpected::attribute(self.config, &self.node, attribute)?
        }
        Ok(())
    }
}
