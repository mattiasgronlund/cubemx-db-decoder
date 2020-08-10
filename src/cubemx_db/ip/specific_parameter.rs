use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

use super::possible_value::PossibleValue;

#[derive(Debug)]
pub struct SpecificParameter {
    pub name: String,
    pub possible_value: Vec<PossibleValue>,
}

impl Decode for SpecificParameter {
    type Object = SpecificParameter;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut possible_value = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "PossibleValue" => possible_value.push(PossibleValue::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = SpecificParameter {
            name: attributes.take_required("Name")?,
            possible_value,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
