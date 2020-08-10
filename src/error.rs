use crate::Config;
use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidFile(#[from] std::io::Error),
    #[error(transparent)]
    InvalideXML(#[from] roxmltree::Error),
    #[error("unknown document root tag '{tag_name}' at {pos}")]
    UnknownDocumentType {
        tag_name: String,
        pos: roxmltree::TextPos,
    },
    #[error("unexpected attribute '{attribute_name}' for element '{element_name}' at {pos}")]
    UnexpectedAttribute {
        attribute_name: String,
        element_name: String,
        pos: roxmltree::TextPos,
    },
    #[error("unexpected element '{element_name}' for element '{node_name}' at {pos}")]
    UnexpectedElement {
        node_name: String,
        element_name: String,
        pos: roxmltree::TextPos,
    },
    #[error("text content not expected for element '{node_name}' at {pos}")]
    UnexpectedText {
        node_name: String,
        pos: roxmltree::TextPos,
    },
    #[error("missing attribute '{attribute_name}' for element '{node_name}' at {pos}")]
    MissingAttribute {
        attribute_name: String,
        node_name: String,
        pos: roxmltree::TextPos,
    },
    #[error("Invalid value '{attribute_value}' of element '{node_name}' for '{attribute_name}' at {pos}: {source}")]
    InvalidAttribute {
        attribute_name: String,
        node_name: String,
        attribute_value: String,
        pos: roxmltree::TextPos,
        source: Box<dyn std::error::Error>,
    },
}

impl Error {
    pub(crate) fn invalid_attribute(
        node: &roxmltree::Node,
        attribute: &roxmltree::Attribute,
        source: Box<dyn std::error::Error>,
    ) -> Error {
        Error::InvalidAttribute {
            attribute_name: attribute.name().to_string(),
            node_name: node.tag_name().name().to_string(),
            attribute_value: attribute.value().to_string(),
            pos: node.document().text_pos_at(attribute.value_range().start),
            source,
        }
    }
}

pub(crate) struct Unexpected;

impl Unexpected {
    pub(crate) fn attribute(
        config: Config,
        node: &roxmltree::Node,
        attribute: &roxmltree::Attribute,
    ) -> Result<(), Error> {
        if config.report_unexpected_errors {
            Err(Error::UnexpectedAttribute {
                attribute_name: attribute.name().to_string(),
                element_name: node.tag_name().name().to_string(),
                pos: node.document().text_pos_at(attribute.range().start),
            })
        } else {
            Ok(())
        }
    }
    pub(crate) fn element(
        config: Config,
        node: &roxmltree::Node,
        element: &roxmltree::Node,
    ) -> Result<(), Error> {
        if config.report_unexpected_errors {
            Err(Error::UnexpectedElement {
                element_name: element.tag_name().name().to_string(),
                node_name: node.tag_name().name().to_string(),
                pos: element.document().text_pos_at(element.range().start),
            })
        } else {
            Ok(())
        }
    }

    pub(crate) fn text(
        config: Config,
        node: &roxmltree::Node,
        child: &roxmltree::Node,
    ) -> Result<(), Error> {
        if config.report_unexpected_errors {
            let text = child.text().unwrap_or_default().trim();
            if text.len() > 0 {
                return Err(Error::UnexpectedText {
                    node_name: node.tag_name().name().to_string(),
                    pos: node.document().text_pos_at(child.range().start),
                });
            }
        }
        Ok(())
    }
}
