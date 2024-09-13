// module api

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

/// rust-container-tool cli struct
#[derive(Parser, Debug)]
#[command(name = "rust-serverless-interface-cli")]
#[command(author = "Luigi Mario Zuccarelli <luzuccar@redhat.com>")]
#[command(version = "0.1.1")]
#[command(about = "A cli to build and create serverless unikernels", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// set the loglevel
    #[arg(
        value_enum,
        short,
        long,
        value_name = "loglevel",
        default_value = "info",
        help = "Set the log level [possible values: info, debug, trace]"
    )]
    pub loglevel: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate subcommand (build and creates a unikernel)
    Generate {
        #[arg(
            short,
            long,
            value_name = "config-file",
            help = "config file (serverless config) used to geenerate the unikernel (required))"
        )]
        config_file: String,

        #[arg(
            short,
            long,
            value_name = "working-dir",
            help = "The directory used to stage all metadata, needed to create a unikernel (required))"
        )]
        working_dir: String,

        #[arg(
            short,
            long,
            value_name = "no-cleanup",
            help = "If set will keep the generated serverless code directories"
        )]
        no_cleanup: bool,

        #[arg(
            short,
            long,
            value_name = "force-rebuild",
            help = "If set will force a rebuild of the serverless function and recreation of the unikernel"
        )]
        force_rebuild: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerlessConfig {
    #[serde(rename = "apiVersion")]
    api_version: String,

    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "spec")]
    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    #[serde(rename = "services")]
    pub services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "serverlessTemplate")]
    pub serverless_template: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "authors")]
    pub authors: Vec<String>,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "env")]
    pub env: Vec<Env>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Env {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value")]
    pub value: String,
}
