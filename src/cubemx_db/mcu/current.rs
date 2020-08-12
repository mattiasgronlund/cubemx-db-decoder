use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct Current {
    pub lowest: Option<f32>,
    pub run: Option<f32>,
    pub max: Option<f32>,
}

impl Decode for Current {
    type Object = Current;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Current {
            max: attributes.take_optional_f32("Max")?,
            run: attributes.take_optional_f32("Run")?,
            lowest: attributes.take_optional_f32("Lowest")?,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
