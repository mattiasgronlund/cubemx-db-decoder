use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

use super::specific_parameter::SpecificParameter;
#[derive(Debug)]
pub struct RemapBlock {
    pub name: String,
    pub default_remap: Option<String>,
    pub specific_parameter: Vec<SpecificParameter>,
}

impl Decode for RemapBlock {
    type Object = RemapBlock;
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

        let result = RemapBlock {
            name: attributes.take_required("Name")?,
            default_remap: attributes.take_optional("DefaultRemap"),
            specific_parameter,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
