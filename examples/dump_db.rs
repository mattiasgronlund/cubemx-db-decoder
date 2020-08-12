#![allow(unused_variables)]
#![allow(unused_assignments)]
use anyhow::Error;
use clap::Clap;
use cubemx_db_decoder::{Config, CubeMxDoc};
use log::{error, info};
use std::collections::HashSet;
use std::sync::mpsc::channel;

use threadpool::ThreadPool;
use env_logger::Env;

/// Currently only verifies that loading the db is possible
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    /// Path to the CubeMx db folder.
    cubemx_db_path: String,

    /// Document to load, or all Mcu documents if omitted.
    document: Option<String>,
}

enum Doc {
    Ip { name: String, version: String },
    Mcu { name: String },
    Other { path: String },
    Done,
}

fn process_documents(opts: Opts) -> Result<(), Error> {
    let config = Config {
        report_unexpected_elements_and_attributes: true,
    };

    let (tx, rx) = channel();
    match &opts.document {
        Some(doc) => tx.send(Doc::Other {
            path: doc.to_string(),
        })?,
        None => {
            let path = format!("{}/mcu/", &opts.cubemx_db_path);
            for d in std::fs::read_dir(&path)? {
                if let Ok(d) = d {
                    if let Ok(ft) = d.file_type() {
                        if ft.is_file() {
                            let name = d.file_name().to_string_lossy().to_string();
                            if name.starts_with("STM32") {
                                tx.send(Doc::Mcu { name })?;
                            }
                        }
                    }
                }
            }
        }
    };

    let mut processed = HashSet::new();
    let pool = ThreadPool::with_name("worker".into(), 4);

    let mut queued = 0;
    loop {
        let doc = rx.recv().unwrap();
        let doc_path = match doc {
            Doc::Ip { name, version } => format!(
                "{}/mcu/IP/{}-{}_Modes.xml",
                &opts.cubemx_db_path, name, version
            ),
            Doc::Other { path } => path,
            Doc::Mcu { name } => format!("{}/mcu/{}", &opts.cubemx_db_path, name),
            Doc::Done => {
                queued = queued - 1;
                if queued == 0 {
                    break;
                }
                continue;
            }
        };

        if processed.contains(&doc_path) {
            continue;
        }
        queued = queued + 1;

        info!("Queueing: {}", &doc_path);
        processed.insert(doc_path.to_string());

        let thread_tx = tx.clone();
        let thread_doc = doc_path;
        pool.execute(move || {
            let x = CubeMxDoc::decode_document(config, &thread_doc);
            match x {
                Ok(CubeMxDoc::Mcu(mcu_doc)) => {
                    info!("Done processing Mcu document {:?}", thread_doc);
                    for ip in mcu_doc.ip {
                        thread_tx
                            .send(Doc::Ip {
                                name: ip.name.to_string(),
                                version: ip.version.to_string(),
                            })
                            .unwrap();
                    }
                }
                Ok(CubeMxDoc::Ip(mcu_doc)) => info!("Done processing Ip document {:?}", thread_doc),
                Err(err) => error!("Failed to process: {}: {}", thread_doc, err),
            }
            thread_tx.send(Doc::Done).unwrap();
        });
    }
    pool.join();
    Ok(())
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    
    match process_documents(Opts::parse()) {
        Ok(_) => {}
        Err(err) => error!("{}", err),
    }
}
