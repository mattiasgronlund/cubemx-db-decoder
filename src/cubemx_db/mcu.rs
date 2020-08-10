use crate::{
    decode::{AttributeMap, Decode},
    error::Unexpected,
    Config, Error,
};
use ip::Ip;
use pin::Pin;
use temperature::Temperature;
use voltage::Voltage;
mod ip;
mod pin;
mod temperature;
mod voltage;
mod signal;
use signal::Signal;
use crate::text_only_element;

text_only_element!(Core);
text_only_element!(Frequency);
text_only_element!(Ram);
text_only_element!(IONb);
text_only_element!(Die);
text_only_element!(Flash);

#[derive(Debug)]
pub struct Mcu {
    pub clock_tree: String,
    pub db_version: String,
    pub family: String,
    pub has_power_pad: String,
    pub io_type: String,
    pub line: String,
    pub package: String,
    pub ref_name: String,

    pub core: Vec<Core>,
    pub frequency: Vec<Frequency>,
    pub ram: Vec<Ram>,
    pub ionb: Vec<IONb>,
    pub die: Vec<Die>,
    pub flash: Vec<Flash>,
    pub voltage: Vec<Voltage>,
    pub temperature: Vec<Temperature>,
    pub ip: Vec<Ip>,
    pub pin: Vec<Pin>,
}

impl Decode for Mcu {
    type Object = Mcu;
    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
        let mut attributes = AttributeMap::from(node, config);

        let mut core = Vec::new();
        let mut frequency = Vec::new();
        let mut ram = Vec::new();
        let mut ionb = Vec::new();
        let mut die = Vec::new();
        let mut flash = Vec::new();
        let mut voltage = Vec::new();
        let mut temperature = Vec::new();
        let mut ip = Vec::new();
        let mut pin = Vec::new();

        for child in node.children() {
            match child.node_type() {
                roxmltree::NodeType::Element => match child.tag_name().name() {
                    "Core" => core.push(Core::decode(config, child)?),
                    "Frequency" => frequency.push(Frequency::decode(config, child)?),
                    "Ram" => ram.push(Ram::decode(config, child)?),
                    "IONb" => ionb.push(IONb::decode(config, child)?),
                    "Die" => die.push(Die::decode(config, child)?),
                    "Flash" => flash.push(Flash::decode(config, child)?),
                    "Voltage" => voltage.push(Voltage::decode(config, child)?),
                    "Temperature" => temperature.push(Temperature::decode(config, child)?),
                    "IP" => ip.push(Ip::decode(config, child)?),
                    "Pin" => pin.push(Pin::decode(config, child)?),
                    _ => Unexpected::element(config, &node, &child)?,
                },
                roxmltree::NodeType::Text => Unexpected::text(config, &node, &child)?,
                _ => {}
            }
        }

        let result = Mcu {
            clock_tree: attributes.take_required("ClockTree")?,
            db_version: attributes.take_required("DBVersion")?,
            family: attributes.take_required("Family")?,
            has_power_pad: attributes.take_required("HasPowerPad")?,
            io_type: attributes.take_required("IOType")?,
            line: attributes.take_required("Line")?,
            package: attributes.take_required("Package")?,
            ref_name: attributes.take_required("RefName")?,

            core,
            frequency,
            ram,
            ionb,
            die,
            flash,
            voltage,
            ip,
            temperature,
            pin,
        };
        attributes.report_unexpected_if_not_empty()?;
        Ok(result)
    }
}

