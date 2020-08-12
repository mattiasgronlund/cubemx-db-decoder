use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct ContextCondition {
    pub expression: String,
    pub diagnostic: Option<String>,
}

impl Decode for ContextCondition {
    type Object = ContextCondition;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = ContextCondition {
            expression: attributes.take_required("Expression")?,
            diagnostic: attributes.take_optional("Diagnostic"),
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
