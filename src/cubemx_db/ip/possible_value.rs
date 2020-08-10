use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
#[derive(Debug)]
pub struct PossibleValue {
    pub value: String,
    pub comment: Option<String>,
    pub semaphore: Option<String>,
    pub condition: Option<String>,
    pub diagnotic: Option<String>,
    pub action: Option<String>,
}

impl Decode for PossibleValue {
    type Object = PossibleValue;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut text = String::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => {
                    text.push_str(child.text().unwrap_or_default())
                }
                _ => {}
            }
        }

        let value_attribute = attributes.take_optional("Value");

        let value = match value_attribute {
            Some(v) => v,            
            None => text,
        };
        
        let result = PossibleValue {
            value,
            comment: attributes.take_optional("Comment"),
            semaphore: attributes.take_optional("Semaphore"),
            condition: attributes.take_optional("Condition"),
            action: attributes.take_optional("Action"),
            diagnotic: attributes.take_optional("Diagnostic"),
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
