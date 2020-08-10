use crate::{Config, decode::{AttributeMap, Decode}, Error, error::Unexpected};

#[derive(Debug)]
pub struct Signal {
    pub name: String,
    pub io_mode: Option<String>,
    pub direction: Option<String>,
}

impl Decode for Signal {
    type Object = Signal;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Signal {
            name: attributes.take_required("Name")?,
            io_mode: attributes.take_optional("IOMode"),
            direction: attributes.take_optional("Direction"),
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
