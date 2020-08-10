use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

use super::condition::Condition;
use super::possible_value::PossibleValue;
use super::Description;
#[derive(Debug)]
pub struct RefParameter {
    pub name: String,
    pub comment: String,
    pub param_type: String,
    pub group: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub unit: Option<String>,
    pub display: Option<String>,
    pub no_check_opt: Option<bool>,
    pub tab_name: Option<String>,
    pub array_size: Option<String>,
    pub array_type_element: Option<String>,
    pub ip: Option<String>,
    pub separator: Option<String>,

    pub default_value: Option<String>,
    pub possible_value: Vec<PossibleValue>,
    pub description: Vec<Description>,
    pub condition: Vec<Condition>,
}

impl Decode for RefParameter {
    type Object = RefParameter;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut possible_value = Vec::new();
        let mut description = Vec::new();
        let mut condition = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "Condition" => condition.push(Condition::decode(config, child)?),
                    "Description" => description.push(Description::decode(config, child)?),
                    "PossibleValue" => possible_value.push(PossibleValue::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        attributes.take_optional("Visible");
        let result = RefParameter {
            name: attributes.take_required("Name")?,
            comment: attributes.take_required("Comment")?,
            param_type: attributes.take_required("Type")?,
            default_value: attributes.take_optional("DefaultValue"),
            group: attributes.take_optional("Group"),
            max: attributes.take_optional("Max"),
            min: attributes.take_optional("Min"),
            unit: attributes.take_optional("Unit"),
            display: attributes.take_optional("Display"),
            no_check_opt: attributes.take_optional_bool("NoCheckOpt")?,
            tab_name: attributes.take_optional("TabName"),
            array_size: attributes.take_optional("ArraySize"),
            array_type_element: attributes.take_optional("ArrayTypeElement"),
            ip: attributes.take_optional("IP"),
            separator: attributes.take_optional("Separator"),
            condition,
            description,
            possible_value,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
