use super::Signal;
use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct SignalLogicalOp {
    pub name: String,
    pub signal: Vec<Signal>,
}

impl Decode for SignalLogicalOp {
    type Object = SignalLogicalOp;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut signal = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "Signal" => signal.push(Signal::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = SignalLogicalOp {
            name: attributes.take_required("Name")?,
            signal,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
