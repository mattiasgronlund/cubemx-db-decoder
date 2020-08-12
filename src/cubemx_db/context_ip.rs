use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct ContextIp {
    pub context_name: String,
    pub forced_selection: Option<String>,
    pub default_selection: Option<String>,
    pub initializer_forced: Option<String>,
    pub validated_on_select: Option<String>,
    pub synchronized_contexts: Option<String>,
}

impl Decode for ContextIp {
    type Object = ContextIp;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => Unexpected::element(config, &node, &child)?,
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = ContextIp {
            context_name: attributes.take_required("ContextName")?,
            forced_selection: attributes.take_optional("ForcedSelection"),
            initializer_forced: attributes.take_optional("InitializerForced"),
            validated_on_select: attributes.take_optional("ValidatedOnSelect"),
            synchronized_contexts: attributes.take_optional("SynchronizedContexts"),
            default_selection: attributes.take_optional("DefaultSelection"),
            
            
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
