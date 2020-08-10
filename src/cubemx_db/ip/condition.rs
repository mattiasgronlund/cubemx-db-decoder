use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct Condition {
    pub diagnostic: String,
    pub expression: String,
}

impl Decode for Condition {
    type Object = Condition;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Condition {
            diagnostic: attributes.take_required("Diagnostic")?,
            expression: attributes.take_required("Expression")?,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
