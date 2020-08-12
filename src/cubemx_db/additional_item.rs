use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct AdditionalItem {
    pub name: String,
    pub value: Option<String>,
}

impl Decode for AdditionalItem {
    type Object = AdditionalItem;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = AdditionalItem {
            value: attributes.take_optional("Value"),
            name: attributes.take_required("Name")?,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
