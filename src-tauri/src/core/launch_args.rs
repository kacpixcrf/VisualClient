use std::path::PathBuf;
use tokio::process::Command;

use crate::core::launch_info::LaunchInfo;
use crate::core::instances::Instance;
use crate::core::paths::get_dot_visualclient_dir;
use crate::commands::accounts::Account;

const DEFAULT_UUID: &str = "00000000-0000-0000-0000-000000000000";
const DEFAULT_TOKEN: &str = "0";
const ACCOUNT_TYPE_MICROSOFT: &str = "Microsoft";
const ACCOUNT_TYPE_OFFLINE: &str = "Offline";
const USER_TYPE_MSA: &str = "msa";
const USER_TYPE_LEGACY: &str = "legacy";
const JVM_ARG_MAX_RAM: &str = "-Xmx2G";
const JVM_ARG_UNLOCK_VM: &str = "-XX:+UnlockExperimentalVMOptions";
const JVM_ARG_G1GC: &str = "-XX:+UseG1GC";
const JVM_ARG_NATIVE_ACCESS: &str = "--enable-native-access=ALL-UNNAMED";
const CLASSPATH_SEP_WINDOWS: &str = ";";
const CLASSPATH_SEP_UNIX: &str = ":";

pub struct LaunchArgs {
    pub instance: Instance,
    pub java_path: PathBuf,
    pub profiles_dir: PathBuf,
    pub mc_dir: PathBuf,
    pub account: Account,
}

pub fn resolve_launch_args(id: &str, username: &str) -> Result<LaunchArgs, String> {
    let instances = crate::core::instances::get_instances().unwrap_or_default();
    let instance = instances.iter().find(|i| i.id == id).ok_or("Instance not found")?.clone();

    let vc_dir = get_dot_visualclient_dir();
    let mc_dir = vc_dir.join("minecraft");
    let profiles_dir = vc_dir.join("profiles").join(id);

    let java_path = if !instance.java_path.is_empty() {
        PathBuf::from(&instance.java_path)
    } else {
        return Err("No Java path".to_string());
    };

    let accounts = crate::commands::accounts::get_accounts().unwrap_or_default();
    let account = accounts.into_iter().find(|a| a.username == username).unwrap_or_else(|| {
        Account {
            username: username.to_string(),
            active: true,
            account_type: Some(ACCOUNT_TYPE_OFFLINE.to_string()),
            uuid: None,
            mc_token: None,
            refresh_token: None,
        }
    });

    Ok(LaunchArgs { instance, java_path, profiles_dir, mc_dir, account })
}

pub fn apply_jvm_args(cmd: &mut Command, launch_info: &LaunchInfo, args: &LaunchArgs) {
    let separator = if cfg!(target_os = "windows") { CLASSPATH_SEP_WINDOWS } else { CLASSPATH_SEP_UNIX };
    for arg in &launch_info.jvm_args {
        let arg = arg
            .replace("${library_directory}", &args.mc_dir.join("libraries").to_string_lossy())
            .replace("${classpath_separator}", separator)
            .replace("${version_name}", &args.instance.version)
            .replace("${classpath}", &launch_info.classpath);
        cmd.arg(arg);
    }
}

pub fn apply_game_args(
    cmd: &mut Command,
    launch_info: &LaunchInfo,
    args: &LaunchArgs,
    uuid_str: &str,
    token_str: &str,
    user_type_str: &str,
    supports_quick_play: bool,
) -> bool {
    let mut has_username = false;
    for arg in &launch_info.game_args {
        if arg.contains("--username") || arg.contains("${auth_player_name}") {
            has_username = true;
        }
        let arg = arg
            .replace("${auth_player_name}", &args.account.username)
            .replace("${version_name}", &args.instance.version)
            .replace("${game_directory}", &args.profiles_dir.to_string_lossy())
            .replace("${assets_root}", &args.mc_dir.join("assets").to_string_lossy())
            .replace("${assets_index_name}", &launch_info.asset_index)
            .replace("${auth_uuid}", uuid_str)
            .replace("${auth_access_token}", token_str)
            .replace("${user_type}", user_type_str)
            .replace("${version_type}", "release");
        if supports_quick_play && arg.starts_with("--userType") {
            continue;
        }
        cmd.arg(arg);
    }
    has_username
}

pub fn apply_fallback_game_args(
    cmd: &mut Command,
    launch_info: &LaunchInfo,
    args: &LaunchArgs,
    uuid_str: &str,
    token_str: &str,
    user_type_str: &str,
    supports_quick_play: bool,
) {
    cmd.arg("--username").arg(&args.account.username);
    cmd.arg("--version").arg(&args.instance.version);
    cmd.arg("--gameDir").arg(&args.profiles_dir);
    cmd.arg("--assetsDir").arg(args.mc_dir.join("assets"));
    cmd.arg("--assetIndex").arg(&launch_info.asset_index);
    cmd.arg("--uuid").arg(uuid_str);
    cmd.arg("--accessToken").arg(token_str);
    if !supports_quick_play {
        cmd.arg("--userType").arg(user_type_str);
    }
    cmd.arg("--versionType").arg("release");
}

pub fn check_modern_mc(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 2 {
        if let (Ok(major), Ok(minor)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            return major > 1 || (major == 1 && minor >= 18);
        }
    }
    false
}

pub fn check_quick_play(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 2 {
        if let (Ok(major), Ok(minor)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            return major > 1 || (major == 1 && minor >= 20);
        }
    }
    false
}

pub fn get_uuid_and_token(account: &Account) -> (String, String, &'static str) {
    let uuid_str = account.uuid.clone().unwrap_or_else(|| DEFAULT_UUID.to_string());
    let token_str = account.mc_token.clone().unwrap_or_else(|| DEFAULT_TOKEN.to_string());
    let user_type_str = if account.account_type.as_deref() == Some(ACCOUNT_TYPE_MICROSOFT) {
        USER_TYPE_MSA
    } else {
        USER_TYPE_LEGACY
    };
    (uuid_str, token_str, user_type_str)
}

pub fn build_base_jvm_args(cmd: &mut Command, launch_info: &LaunchInfo, args: &LaunchArgs) {
    cmd.arg(JVM_ARG_MAX_RAM);
    cmd.arg(JVM_ARG_UNLOCK_VM);
    cmd.arg(JVM_ARG_G1GC);

    let natives_dir = args.mc_dir.join("versions").join(&args.instance.version).join("natives");
    cmd.arg(format!("-Djava.library.path={}", natives_dir.to_string_lossy()));

    if check_modern_mc(&args.instance.version) {
        cmd.arg(JVM_ARG_NATIVE_ACCESS);
    }

    apply_jvm_args(cmd, launch_info, args);

    cmd.arg("-cp");
    cmd.arg(&launch_info.classpath);
    cmd.arg(&launch_info.main_class);
}
