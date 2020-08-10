use super::Condition;
use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct BspDependency {
    pub name: String,
    pub comment: String,
    pub bsp_ip_name: String,
    pub bsp_mode_name: Option<String>,
    pub user_name: Option<String>,
    pub api: Option<String>,
    pub condition: Vec<Condition>,    
}

impl Decode for BspDependency {
    type Object = BspDependency;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut condition = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "Condition" => condition.push(Condition::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = BspDependency {
            name: attributes.take_required("Name")?,
            comment: attributes.take_required("Comment")?,
            bsp_ip_name: attributes.take_required("BspIpName")?,
            bsp_mode_name: attributes.take_optional("BspModeName"),
            user_name: attributes.take_optional("UserName"),
            api: attributes.take_optional("Api"),
            condition,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
