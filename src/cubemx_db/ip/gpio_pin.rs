use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
use super::SpecificParameter;
use super::PinSignal;

#[derive(Debug)]
pub struct GPIOPin {
    pub port_name: String,
    pub name: String,
    pub specific_parameter: Vec<SpecificParameter>,
    pub pin_signal: Vec<PinSignal>,
}

impl Decode for GPIOPin {
    type Object = GPIOPin;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut specific_parameter = Vec::new();
        let mut pin_signal = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "PinSignal" => pin_signal.push(PinSignal::decode(config, child)?),
                    "SpecificParameter" => {
                        specific_parameter.push(SpecificParameter::decode(config, child)?)
                    }
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = GPIOPin {
            port_name: attributes.take_required("PortName")?,
            name: attributes.take_required("Name")?,
            pin_signal,
            specific_parameter,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
