use custom_logger::*;
use mirror_error::MirrorError;
use std::process::Command;

pub async fn build(log: &Logging) -> Result<(), MirrorError> {
    log.ex("[build] cargo build");

    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        log.ex("[build] completed successfully");
    }
    log.debug(&format!(
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    ));
    log.debug(&format!(
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    ));
    if !output.status.success() {
        return Err(MirrorError::new(&format!(
            "[build] {:?}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    assert!(output.status.success());
    Ok(())
}

pub async fn create_unikernel(log: &Logging, name: String) -> Result<(), MirrorError> {
    log.ex("[create_unikernel] creating unikernel");
    let output = Command::new("ops")
        .arg("build")
        .arg("-c")
        .arg("config.json")
        .arg("-i")
        .arg(name.clone())
        .arg(format!("target/release/{}", name))
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        log.ex("[create_unikernel] completed successfully");
    }
    log.debug(&format!(
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    ));
    log.debug(&format!(
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    ));

    assert!(output.status.success());
    Ok(())
}
