use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
mod bsp_dependency;

mod context_condition;
mod gpio_pin;
mod gpio_port;
mod mode;
mod mode_logic_operator;
mod parameter;
mod pin_signal;
mod possible_value;
mod ref_mode;
mod ref_parameter;
mod ref_signal;
mod remap_block;
mod signal;
mod signal_logical_op;
mod specific_parameter;
use super::condition::Condition;
use context_condition::ContextCondition;
use remap_block::RemapBlock;

use bsp_dependency::BspDependency;
use gpio_pin::GPIOPin;
use gpio_port::GPIOPort;
use mode::Mode;
use mode_logic_operator::ModeLogicOperator;
use parameter::Parameter;
use pin_signal::PinSignal;
use possible_value::PossibleValue;
use ref_mode::RefMode;
use ref_parameter::RefParameter;
use ref_signal::RefSignal;
use signal::Signal;
use signal_logical_op::SignalLogicalOp;
use specific_parameter::SpecificParameter;

use crate::text_only_element;
text_only_element!(About);
text_only_element!(Description);
text_only_element!(Semaphore);
text_only_element!(ConfigForMode);

#[derive(Debug)]
pub struct Ip {
    pub db_version: String,
    pub ip_type: String,
    pub name: String,
    pub version: String,
    pub ip_group: Option<String>,

    pub about: Vec<About>,
    pub ref_parameter: Vec<RefParameter>,
    pub ref_mode: Vec<RefMode>,
    pub gpio_pin: Vec<GPIOPin>,
    pub gpio_port: Vec<GPIOPort>,
    pub mode_logic_operator: Vec<ModeLogicOperator>,
    pub ref_signal: Vec<RefSignal>,
    pub semaphore: Vec<Semaphore>,
    pub condition: Vec<Condition>,
}

impl Decode for Ip {
    type Object = Ip;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        let mut about = Vec::new();
        let mut ref_parameter = Vec::new();
        let mut ref_mode = Vec::new();
        let mut gpio_pin = Vec::new();
        let mut gpio_port = Vec::new();
        let mut mode_logic_operator = Vec::new();
        let mut ref_signal = Vec::new();
        let mut semaphore = Vec::new();
        let mut condition = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "About" => about.push(About::decode(config, child)?),
                    "RefParameter" => ref_parameter.push(RefParameter::decode(config, child)?),
                    "RefMode" => ref_mode.push(RefMode::decode(config, child)?),
                    "GPIO_Pin" => gpio_pin.push(GPIOPin::decode(config, child)?),
                    "GPIO_Port" => gpio_port.push(GPIOPort::decode(config, child)?),
                    "ModeLogicOperator" => {
                        mode_logic_operator.push(ModeLogicOperator::decode(config, child)?)
                    }
                    "RefSignal" => ref_signal.push(RefSignal::decode(config, child)?),
                    "Semaphore" => semaphore.push(Semaphore::decode(config, child)?),
                    "Condition" => condition.push(Condition::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        attributes.take_optional("schemaLocation");

        let result = Ip {
            db_version: attributes.take_required("DBVersion")?,
            ip_type: attributes.take_required("IPType")?,
            name: attributes.take_required("Name")?,
            version: attributes.take_required("Version")?,
            ip_group: attributes.take_optional("IpGroup"),

            about,
            ref_parameter,
            ref_mode,
            ref_signal,
            gpio_pin,
            gpio_port,
            mode_logic_operator,
            semaphore,
            condition,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}
