use super::ModeLogicOperator;
use super::Semaphore;
use super::Condition;
use super::SignalLogicalOp;
use super::BspDependency;
use super::ContextCondition;

use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};

#[derive(Debug)]
pub struct Mode {
    pub name: String,
    pub user_name: Option<String>,
    pub shrink_name: Option<String>,
    pub remove_condition: Option<String>,
    pub remove_disable: Option<String>,
    pub mode_logic_operator: Vec<ModeLogicOperator>,
    pub signal_logical_op: Vec<SignalLogicalOp>,
    pub semaphore: Vec<Semaphore>,
    pub condition: Vec<Condition>,
    pub bsp_dependency: Vec<BspDependency>,
    pub context_condition: Vec<ContextCondition>,
}

impl Decode for Mode {
    type Object = Mode;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);
        let mut mode_logic_operator = Vec::new();
        let mut signal_logical_op = Vec::new();
        let mut semaphore = Vec::new();
        let mut condition = Vec::new();
        let mut bsp_dependency = Vec::new();
        let mut context_condition = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "ModeLogicOperator" => {
                        mode_logic_operator.push(ModeLogicOperator::decode(config, child)?)
                    }
                    "SignalLogicalOp" => {
                        signal_logical_op.push(SignalLogicalOp::decode(config, child)?)
                    }
                    "Semaphore" => semaphore.push(Semaphore::decode(config, child)?),
                    "Condition" => condition.push(Condition::decode(config, child)?),
                    "BspDependency" => bsp_dependency.push(BspDependency::decode(config, child)?),
                    "ContextCondition" => context_condition.push(ContextCondition::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Mode {
            name: attributes.take_required("Name")?,
            user_name: attributes.take_optional("UserName"),
            shrink_name: attributes.take_optional("ShrinkName"),
            remove_condition: attributes.take_optional("RemoveCondition"),
            remove_disable: attributes.take_optional("RemoveDisable"),
            context_condition,
            mode_logic_operator,
            signal_logical_op,
            semaphore,
            bsp_dependency,
            condition,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
