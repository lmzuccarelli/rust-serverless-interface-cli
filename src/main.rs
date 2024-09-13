use crate::api::schema::*;
use crate::config::read::*;
use clap::Parser;
use custom_logger::*;
use mirror_error::MirrorError;
use mirror_utils::fs_handler;
use shell::process::{build, create_unikernel};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process;

mod api;
mod config;
mod shell;

#[tokio::main]
async fn main() -> Result<(), MirrorError> {
    let args = Cli::parse();

    let lvl = args.loglevel.as_ref().unwrap();

    let l = match lvl.as_str() {
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };

    let log = &Logging { log_level: l };

    match &args.command {
        Some(Commands::Generate {
            working_dir,
            config_file,
            no_cleanup,
            force_rebuild,
        }) => {
            fs_handler(format!("{}/generated", working_dir), "create_dir", None).await?;
            let config = load_config(config_file.to_string()).await?;
            let sc = parse_yaml_config(config)?;
            log.info(&format!("working-dir {}", working_dir));
            log.info(&format!("Serverless struct {:#?}", sc));

            // read the cargo.toml file first
            let cargo_toml = fs_handler("templates/Cargo.tmpl".to_string(), "read", None).await?;
            let main = fs_handler("templates/main.tmpl".to_string(), "read", None).await?;
            let config_json = fs_handler("templates/config.tmpl".to_string(), "read", None).await?;
            for service in sc.spec.services.iter() {
                // check if we have previously built
                if !Path::new(&format!(
                    "{}/generated/{}/target/release/{}",
                    working_dir, service.name, service.name
                ))
                .exists()
                    || force_rebuild.clone()
                {
                    fs_handler(
                        format!("{}/generated/{}/src", working_dir, service.name),
                        "create_dir",
                        None,
                    )
                    .await?;
                    let repo = match service.serverless_template.clone() {
                        x if x.contains("path://") => {
                            format!(
                                "{{ path = \"{}\" }}",
                                service.serverless_template.replace("path://", "")
                            )
                        }
                        x if x.contains("git") => {
                            format!("{{ git = \"{}\" }}", service.serverless_template)
                        }
                        _ => "".to_string(),
                    };
                    let updated_tompl = cargo_toml
                        .replace("{{ NAME }}", &service.name)
                        .replace("{{ VERSION }}", &service.version)
                        .replace("{{ REPO }}", &repo)
                        .replace("{{ AUTHORS }}", &format!("{:?}", service.authors));
                    fs_handler(
                        format!(
                            "{}/generated/{}/{}",
                            working_dir, service.name, "Cargo.toml"
                        ),
                        "write",
                        Some(updated_tompl.clone()),
                    )
                    .await?;

                    // get all relevant envars
                    let mut env_map: HashMap<String, String> = HashMap::new();
                    for env in service.env.iter() {
                        env_map.insert(env.name.clone(), env.value.clone());
                    }

                    let updated_main = main
                        .replace("{{ IP }}", env_map.get("IP").unwrap())
                        .replace("{{ PORT }}", env_map.get("SERVER_PORT").unwrap())
                        .replace("{{ LOG_LEVEL }}", env_map.get("LOG_LEVEL").unwrap());
                    fs_handler(
                        format!(
                            "{}/generated/{}/src/{}",
                            working_dir, service.name, "main.rs"
                        ),
                        "write",
                        Some(updated_main.clone()),
                    )
                    .await?;

                    // finally create config.json for unikernel
                    let mut env_vars = "".to_string();
                    let size = env_map.len();
                    let mut count = 1;
                    for (k, v) in env_map {
                        if count < size {
                            env_vars = env_vars + &format!("\t\t\"{}\":\"{}\",\n", k, v);
                        } else {
                            env_vars = env_vars + &format!("\t\t\"{}\":\"{}\"\n", k, v);
                        }
                        count += 1;
                    }
                    let updated_config_json = config_json.replace("{{ envars }}", &env_vars);
                    fs_handler(
                        format!(
                            "{}/generated/{}/{}",
                            working_dir, service.name, "config.json"
                        ),
                        "write",
                        Some(updated_config_json.clone()),
                    )
                    .await?;
                }

                // finally we can build and create the unikernel
                // change directory
                env::set_current_dir(&format!("generated/{}", service.name.clone())).unwrap();
                build(log).await?;
                create_unikernel(log, service.name.clone()).await?;
                env::set_current_dir("../../").unwrap();
                if !no_cleanup {
                    fs_handler("generated".to_string(), "remove_dir", None).await?;
                }
            }
        }
        None => {
            log.error("sub command not recocgnized");
            process::exit(1);
        }
    }
    Ok(())
}
