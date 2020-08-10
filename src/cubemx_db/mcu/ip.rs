use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct Ip {
    pub instance_name: String,
    pub name: String,
    pub version: String,
    pub config_file: Option<String>,
    pub clock_enable_mode: Option<String>,
}

impl Decode for Ip {
    type Object = Ip;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Ip {
            instance_name: attributes.take_required("InstanceName")?,
            name: attributes.take_required("Name")?,
            version: attributes.take_required("Version")?,
            config_file: attributes.take_optional("ConfigFile"),
            clock_enable_mode: attributes.take_optional("ClockEnableMode"),
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
