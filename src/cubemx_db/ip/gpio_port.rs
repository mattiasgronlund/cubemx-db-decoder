use super::SpecificParameter;
use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct GPIOPort {
    pub name: String,
    pub clock_enable_mode: String,
    pub specific_parameter: Vec<SpecificParameter>,
}

impl Decode for GPIOPort {
    type Object = GPIOPort;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut specific_parameter = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "SpecificParameter" => {
                        specific_parameter.push(SpecificParameter::decode(config, child)?)
                    }
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = GPIOPort {
            name: attributes.take_required("Name")?,
            clock_enable_mode: attributes.take_required("ClockEnableMode")?,
            specific_parameter,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
