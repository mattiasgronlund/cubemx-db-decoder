#![allow(unused_variables)]
#![allow(unused_assignments)]
use clap::Clap;
use cubemx_db_decoder::{Config, CubeMxDoc};
use log::{error, info};

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    cubemx_db_path: String,

    /// Some input. Because this isn't an Option<T> it's required to be used
    document: String,
}

fn main() {
    env_logger::init();
    let opts: Opts = Opts::parse();

    let path = opts.cubemx_db_path;
    let mut doc_path = path.to_string();
    doc_path.push_str(&opts.document);

    let config = Config {
        report_unexpected_errors: true,
    };

    let mut documents = vec![doc_path];

    while let Some(doc_path) = documents.pop() {
        let x = CubeMxDoc::decode_document(config, doc_path.clone());
        match x {
            Ok(CubeMxDoc::Mcu(mcu_doc)) => {
                info!("Mcu document {:?}", doc_path);
                documents.extend(
                    mcu_doc
                        .ip
                        .iter()
                        .map(|ip| [&path, "/mcu/IP/", &ip.name, "-", &ip.version, "_Modes.xml"].concat()),
                );
            }
            Ok(CubeMxDoc::Ip(mcu_doc)) => info!("Ip document {:?}", doc_path),
            Err(err) => error!("Failed to process: {}: {}", doc_path, err),
        }
    }

    // path.push_str("/STM32G431K(6-8-B)Ux.xml");
    // println!("Path: {}.", path);
}

// use std::{fs, io::BufReader, path::PathBuf};

// use anyhow::{anyhow, ensure, Context, Error};
// use fs::File;
// use itertools::{Either, Itertools};
// use log::{debug, error, info, warn};
// use walkdir::WalkDir;

// #[derive(Hash, Eq, PartialEq, Debug, Clone)]
// struct IpDocumentId {
//     ip_type: String,
//     name: String,
//     version: String,
// }

// impl IpDocumentId {
//     fn new(ip_type: String, name: String, version: String) -> IpDocumentId {
//         IpDocumentId {
//             ip_type,
//             name,
//             version,
//         }
//     }
// }

// struct IpIndex {
//     index: HashMap<IpDocumentId, PathBuf>,
// }

// impl IpIndex {
//     pub fn init(path: &str) -> IpIndex {
//         let mut me = IpIndex {
//             index: HashMap::new(),
//         };
//         me._init(path);
//         me
//     }

//     fn add_to_index(&mut self, path: PathBuf) -> Result<PathBuf, Error> {
//         let file = File::open(&path)?;
//         let file = BufReader::new(file);
//         let file = encoding_rs_io::DecodeReaderBytesBuilder::new()
//             .strip_bom(true)
//             .build(file);
//         let mut event_reader = EventReader::new(file);
//         loop {
//             match event_reader.next()? {
//                 XmlEvent::StartElement {
//                     name, attributes, ..
//                 } => {
//                     ensure!(
//                         name.local_name == "IP",
//                         format!(
//                             "Unexpected start tag: {} at {}",
//                             name.local_name,
//                             event_reader.position()
//                         )
//                     );

//                     let mut ip_type = String::default();
//                     let mut name = String::default();
//                     let mut version = String::default();
//                     for attribute in attributes {
//                         match attribute.name.local_name.as_str() {
//                             "IPType" => {
//                                 ip_type = attribute.value;
//                             }
//                             "Name" => {
//                                 name = attribute.value;
//                             }
//                             "Version" => {
//                                 version = attribute.value;
//                             }
//                             _ => {}
//                         };
//                     }
//                     info!("Adding {} {} {} to the index.", ip_type, name, version);
//                     let key = IpDocumentId::new(ip_type, name, version);
//                     self.index.insert(key.clone(), path.to_owned());
//                     let mut p2 = "".to_string();
//                     p2.push_str(&key.name);
//                     p2.push_str("-");
//                     p2.push_str(&key.version);
//                     p2.push_str("_Modes.xml");
//                     ensure!(
//                         (path.ends_with(&p2)),
//                         format!(
//                             "Unexpected filename {} does not end with {}",
//                             &path.into_os_string().to_string_lossy(),
//                             &p2
//                         )
//                     );
//                     debug!("Successfully added {:#?} to IP index", key);
//                     // While indexing we only care about the 1st tag named IP
//                     break;
//                 }
//                 // XmlEvent::EndElement{ name } => { },
//                 // XmlEvent::Characters(name) => { },
//                 XmlEvent::EndDocument => break,
//                 _ => {}
//             };
//         }

//         Ok(path)
//     }

//     fn _init(&mut self, path: &str) {
//         debug!("Init Ip cache from: {:?}", path);

//         let (_, locate_errors): (Vec<_>, Vec<_>) = WalkDir::new(path)
//             .follow_links(true)
//             .into_iter()
//             .filter_entry(|e| {
//                 e.file_type().is_dir()
//                     || e.file_name()
//                         .to_string_lossy()
//                         .to_lowercase()
//                         .ends_with(".xml")
//             })
//             .map(|r| match r {
//                 Ok(d) => Ok(d),
//                 Err(e) => Err(e).with_context(|| format!("Failed to traverse {}:", path)),
//             })
//             .filter_map(|r| match r {
//                 Ok(d) => {
//                     if d.file_type().is_dir() {
//                         None
//                     } else {
//                         Some(Ok(d.into_path()))
//                     }
//                 }
//                 Err(e) => Some(Err(e).with_context(|| format!("Failed to traverse {}:", path))),
//             })
//             .map_results(|d| {
//                 self.add_to_index(d.clone())
//                     .with_context(|| format!("When processing file {:?}", d.to_string_lossy()))
//             })
//             .flatten()
//             .partition_map(|r| match r {
//                 Ok(path) => Either::Left(path),
//                 Err(error) => Either::Right(error),
//             });
//         let mut errors: Vec<Error> = Vec::new();
//         errors.extend(locate_errors);

//         for x in errors {
//             warn!("Problem indexing IP - files:\n{:?}", x);
//         }
//     }
// }

// trait Helpers {
//     fn get(self, name: &str) -> Result<String, Error>;
//     fn try_get(self, name: &str) -> Option<String>;
// }

// impl Helpers for Vec<OwnedAttribute> {
//     fn get(self, name: &str) -> Result<String, Error> {
//         self.try_get(name)
//             .ok_or(anyhow!("Missing expected attribute {}", name))
//     }

//     fn try_get(self, name: &str) -> Option<String> {
//         self.into_iter()
//             .find(|a| name == a.name.local_name.to_string())
//             .map(|a| a.value)
//     }
// }

// fn process_mcu_document(ip_index: IpIndex, doc_path: String) -> Result<String, Error> {
//     let file = File::open(&doc_path)?;

//     let file = BufReader::new(file);
//     let file = encoding_rs_io::DecodeReaderBytesBuilder::new()
//         .strip_bom(true)
//         .build(file);

//     let mut event_reader = EventReader::new(file);
//     loop {
//         match event_reader.next()? {
//             XmlEvent::StartElement {
//                 name, attributes, ..
//             } => {
//                 ensure!(
//                     name.local_name == "Mcu",
//                     format!("Unexpected start tag: {}", name.local_name,)
//                 );

//                 let mut family = String::default();
//                 let mut line = String::default();
//                 let mut package = String::default();
//                 let mut ref_name = String::default();

//                 for attribute in attributes {
//                     match attribute.name.local_name.as_str() {
//                         "Family" => {
//                             family = attribute.value;
//                         }
//                         "Line" => {
//                             line = attribute.value;
//                         }
//                         "Package" => {
//                             package = attribute.value;
//                         }
//                         "RefName" => {
//                             ref_name = attribute.value;
//                         }
//                         _ => {}
//                     };
//                 }
//                 info!("Processing MCU: {}", ref_name);
//                 break;
//             }
//             // XmlEvent::EndElement{ name } => { },
//             // XmlEvent::Characters(name) => { },
//             XmlEvent::EndDocument => break,
//             _ => {}
//         };
//     }
//     Ok(doc_path)
// }

// fn main() {
//     env_logger::init();
//     let args: Vec<_> = std::env::args().collect();

//     if args.len() != 2 {
//         println!("Usage:\n\tcargo run -- db-path");
//         std::process::exit(1);
//     }
//     let path = (&args[1]).to_owned();
//     let mut ip_path = path.clone();
//     ip_path.push_str("/IP");
//     let ip_index = IpIndex::init(&ip_path);

//     let mut doc_path = path.clone();
//     doc_path.push_str("/STM32G431K(6-8-B)Ux.xml");
//     let x = process_mcu_document(ip_index, doc_path.clone());
//     match x {
//         Ok(path) => info!("Processed {}", path),
//         Err(err) => error!("Failed to process: {}: {}", doc_path, err),
//     }
//     // path.push_str("/STM32G431K(6-8-B)Ux.xml");
//     // println!("Path: {}.", path);
// }
