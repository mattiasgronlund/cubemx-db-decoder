use crate::{Config, decode::{AttributeMap, Decode}, Error, error::Unexpected};

#[derive(Debug)]
pub struct Voltage {
    pub max: f32,
    pub min: f32,
}

impl Decode for Voltage {
    type Object = Voltage;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Voltage {
            max: attributes.take_required_f32("Max")?,
            min: attributes.take_required_f32("Min")?,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
