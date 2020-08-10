use crate::{Config, decode::{AttributeMap, Decode}, Error, error::Unexpected};

#[derive(Debug)]
pub struct RefSignal {
    pub name: String,
    pub io_mode: Option<String>,
    pub is_virtual: Option<bool>,
    pub direction: Option<String>,
    pub sharable_group_name: Option<String>,
}

impl Decode for RefSignal {
    type Object = RefSignal;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = RefSignal {
            name: attributes.take_required("Name")?,
            io_mode: attributes.take_optional("IOMode"),
            is_virtual: attributes.take_optional_bool("Virtual")?,
            direction: attributes.take_optional("Direction"),
            sharable_group_name: attributes.take_optional("ShareableGroupName"),
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
