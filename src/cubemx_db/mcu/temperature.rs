use crate::{Config, decode::{AttributeMap, Decode}, Error, error::Unexpected};

#[derive(Debug)]
pub struct Temperature {
    pub max: i32,
    pub min: i32,
}

impl Decode for Temperature {
    type Object = Temperature;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Temperature {
            max: attributes.take_required_i32("Max")?,
            min: attributes.take_required_i32("Min")?,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
