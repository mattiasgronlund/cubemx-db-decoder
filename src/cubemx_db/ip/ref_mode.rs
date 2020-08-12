use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

use super::Condition;
use super::Parameter;
use super::ConfigForMode;
use super::BspDependency;
#[derive(Debug)]
pub struct RefMode {
    pub name: String,
    pub base_mode: Option<String>,
    pub hal_mode: Option<String>,
    pub comment: Option<String>,
    pub is_abstract: Option<bool>,
    pub group: Option<String>,
    pub tab_name: Option<String>,

    pub condition: Vec<Condition>,
    pub config_for_mode: Vec<ConfigForMode>,
    pub parameter: Vec<Parameter>,
    pub bsp_dependency: Vec<BspDependency>,
}

impl Decode for RefMode {
    type Object = RefMode;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut parameter = Vec::new();
        let mut condition = Vec::new();
        let mut config_for_mode = Vec::new();
        let mut bsp_dependency = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "Condition" => condition.push(Condition::decode(config, child)?),
                    "ConfigForMode" => config_for_mode.push(ConfigForMode::decode(config, child)?),                    
                    "Parameter" => parameter.push(Parameter::decode(config, child)?),
                    "BspDependency" => bsp_dependency.push(BspDependency::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = RefMode {
            name: attributes.take_required("Name")?,
            base_mode: attributes.take_optional("BaseMode"),
            hal_mode: attributes.take_optional("HalMode"),
            comment: attributes.take_optional("Comment"),
            is_abstract: attributes.take_optional_bool("Abstract")?,
            group: attributes.take_optional("Group"),
            tab_name: attributes.take_optional("TabName"),
            condition,
            config_for_mode,
            bsp_dependency,
            parameter,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
