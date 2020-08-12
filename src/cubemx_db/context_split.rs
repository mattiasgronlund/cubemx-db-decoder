use super::context_ip::ContextIp;

use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
#[derive(Debug)]
pub struct ContextSplit {
    pub name: String,
    pub context_ip: Vec<ContextIp>,
}

impl Decode for ContextSplit {
    type Object = ContextSplit;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut context_ip = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "ContextIP" => context_ip.push(ContextIp::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = ContextSplit {
            name: attributes.take_required("Name")?,
            context_ip,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
