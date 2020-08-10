use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
use super::PossibleValue;
use super::Condition;
use super::ConfigForMode;

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ref_parameter: Option<String>,
    pub possible_value: Vec<PossibleValue>,
    pub condition: Vec<Condition>,
    pub config_for_mode: Vec<ConfigForMode>,
}

impl Decode for Parameter {
    type Object = Parameter;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut possible_value = Vec::new();
        let mut condition = Vec::new();
        let mut config_for_mode = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "PossibleValue" => possible_value.push(PossibleValue::decode(config, child)?),
                    "Condition" => condition.push(Condition::decode(config, child)?),
                    "ConfigForMode" => config_for_mode.push(ConfigForMode::decode(config, child)?),                        
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Parameter {
            name: attributes.take_required("Name")?,
            ref_parameter: attributes.take_optional("RefParameter"),
            possible_value,
            condition,
            config_for_mode,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
