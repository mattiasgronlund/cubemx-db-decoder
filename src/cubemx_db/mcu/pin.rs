use super::Condition;
use super::Signal;
use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
#[derive(Debug)]
pub struct Pin {
    pub name: String,
    pub condition: Vec<Condition>,
    pub position: String,
    pub pin_type: String,
    pub variant: Option<String>,
    pub power_domain: Option<String>,
    pub signal: Vec<Signal>,
}

impl Decode for Pin {
    type Object = Pin;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut condition = Vec::new();
        let mut signal = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "Condition" => condition.push(Condition::decode(config, child)?),
                    "Signal" => signal.push(Signal::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Pin {
            name: attributes.take_required("Name")?,
            position: attributes.take_required("Position")?,
            pin_type: attributes.take_required("Type")?,
            power_domain:  attributes.take_optional("PowerDomain"),
            variant:  attributes.take_optional("Variant"),
            condition,
            signal,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
