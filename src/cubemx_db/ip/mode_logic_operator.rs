use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
use super::Mode;

#[derive(Debug)]
pub struct ModeLogicOperator {
    pub name: String,
    pub mode: Vec<Mode>,
}

impl Decode for ModeLogicOperator {
    type Object = ModeLogicOperator;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut mode = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "Mode" => mode.push(Mode::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = ModeLogicOperator {
            name: attributes.take_required("Name")?,
            mode,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
