use crate::{
    cubemx_db::context_split::ContextSplit,
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
    pub context_split: Vec<ContextSplit>,
    pub ip_context_coupling: Option<String>,
    pub power_domain: Option<String>,
}

impl Decode for Ip {
    type Object = Ip;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut context_split = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "ContextSplit" => context_split.push(ContextSplit::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
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
            ip_context_coupling: attributes.take_optional("IPContextCoupling"),
            power_domain: attributes.take_optional("PowerDomain"),

            context_split,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
