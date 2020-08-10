mod ip;
mod mcu;

pub use ip::Ip;
pub use mcu::Mcu;

#[macro_export]
macro_rules! text_only_element {
    ($name:ident) => {
        paste::paste! {
            mod [<$name:lower>] {
                use crate::{decode::Decode, error::Unexpected, Config, Error};

                #[derive(Debug)]
                pub struct $name(String);

                impl Decode for $name {
                    type Object = $name;
                    fn decode(config: Config, node: roxmltree::Node) -> Result<Self::Object, Error> {
                        for attribute in node.attributes() {
                            Unexpected::attribute(config, &node, attribute)?
                        }
                        let mut text = String::new();
                        for child in node.children() {
                            match child.node_type() {
                                roxmltree::NodeType::Element => {
                                    Unexpected::element(config, &node, &child)?
                                }
                                roxmltree::NodeType::Text => {
                                    text.push_str(child.text().unwrap_or_default())
                                }
                                _ => {}
                            }
                        }

                        Ok($name(text))
                    }
                }
            }
            use self::[<$name:lower>]::[<$name>];
        }
    };
}
