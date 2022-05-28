#![allow(dead_code)]
/// if the target name is set then for the final path it will override the name of the crate
pub fn build_helper_crate(
    payload_path: std::path::PathBuf,
    crate_name: &str,
    target: Option<&str>,
    release: bool,
    ext: &str,
    lib_only: bool,
    target_name: Option<&str>,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let payload_crate_path = payload_path.join(crate_name).canonicalize()?;
    let mut command = std::process::Command::new("cargo");
    command
        .arg("build")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    if let Some(target) = target {
        command.arg("--target").arg(target);
    }
    if release {
        command.arg("--release");
    }
    if lib_only {
        command.arg("--lib");
    }
    let exit_code = command.current_dir(&payload_crate_path).spawn()?.wait()?;
    assert!(
        exit_code.success(),
        "Failed to build crate {} for target {} at path {:?}",
        crate_name,
        target.unwrap_or("default"),
        payload_path
    );

    let mut payload_artifact_path = payload_crate_path;
    payload_artifact_path.push("target");

    if let Some(target) = target {
        payload_artifact_path.push(target);
    }

    payload_artifact_path.push(if release { "release" } else { "debug" });
    payload_artifact_path.push(format!(
        "{}.{}",
        if let Some(target_name) = target_name {
            target_name
        } else {
            crate_name
        },
        ext
    ));
    assert!(&payload_artifact_path.exists());

    Ok(payload_artifact_path)
}

pub fn build_payload_x64() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    build_helper_crate(
        std::env::current_dir()?,
        "",
        Some(&find_x64_variant_of_target()),
        false,
        "dll",
        true,
        Some("payload"),
    )
}

pub fn build_payload_x86() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    build_helper_crate(
        std::env::current_dir()?,
        "",
        Some(&find_x86_variant_of_target()),
        false,
        "dll",
        true,
        Some("payload"),
    )
}

fn find_x64_variant_of_target() -> String {
    current_platform::CURRENT_PLATFORM.replace("i686", "x86_64")
}

fn find_x86_variant_of_target() -> String {
    current_platform::CURRENT_PLATFORM.replace("x86_64", "i686")
}
