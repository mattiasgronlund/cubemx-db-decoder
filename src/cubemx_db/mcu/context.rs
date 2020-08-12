use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct Context {
    pub comment: Option<String>,
    pub core: String,
    pub group_name: String,
    pub group_short_name: String,
    pub long_name: String,
    pub short_name: String,
    pub gen_type: String,
    pub name: String,
    pub secure: bool,
    pub semaphore_suffix: String,
}

impl Decode for Context {
    type Object = Context;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Context {
            comment: attributes.take_optional("Comment"),
            core: attributes.take_required("Core")?,
            group_name: attributes.take_required("GroupName")?,
            group_short_name: attributes.take_required("GroupShortName")?,
            long_name: attributes.take_required("LongName")?,
            short_name: attributes.take_required("ShortName")?,
            gen_type: attributes.take_required("GenType")?,
            name: attributes.take_required("Name")?,
            secure: attributes.take_required_bool("Secure")?,
            semaphore_suffix: attributes.take_required("SemaphoreSuffix")?,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
