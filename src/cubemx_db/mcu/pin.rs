use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct Pin {
    pub name: String,
    pub position: String,
    pub pin_type: String,
    pub signal: Vec<Signal>,
}

impl Decode for Pin {
    type Object = Pin;
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

        let result = Pin {
            name: attributes.take_required("Name")?,
            position: attributes.take_required("Position")?,
            pin_type: attributes.take_required("Type")?,
            signal,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct Signal {
    pub name: String,
    pub io_modes: Option<String>,
}

impl Decode for Signal {
    type Object = Signal;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Signal {
            name: attributes.take_required("Name")?,
            io_modes: attributes.take_optional("IOModes"),
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
