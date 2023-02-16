use crate::utils;
use anyhow::Result;
use std::path::Path;
use std::process::Output;
use std::sync::Mutex;

#[cfg(target_family = "unix")]
use {
    nix::sys::signal::{kill, Signal},
    nix::unistd::Pid,
};

static INSTALLING_TEMPLATES_MUTEX: Mutex<i32> = Mutex::new(0);
static INSTALLING_PLUGINS_MUTEX: Mutex<i32> = Mutex::new(0);

pub fn template_install(mut args: Vec<&str>) -> Result<Output> {
    let mut cmd = vec!["spin", "templates", "install"];
    cmd.append(&mut args);

    let x = INSTALLING_TEMPLATES_MUTEX.lock().unwrap();
    let result = utils::run(cmd, None, None);

    //this ensure we have mutex lock until here
    drop(x);

    result
}

pub fn new_app<'a>(
    template_name: &'a str,
    app_name: &'a str,
    mut args: Vec<&'a str>,
) -> Result<Output> {
    let basedir = utils::testcases_base_dir();
    let mut cmd = vec!["spin", "new", template_name, app_name, "--accept-defaults"];
    if args.len() > 0 {
        cmd.append(&mut args);
    }

    return utils::run(cmd, Some(basedir.as_str()), None);
}

pub fn install_plugins(plugins: Vec<&str>) -> Result<Output> {
    // lock mutex to ensure one install_plugins runs at a time
    let x = INSTALLING_PLUGINS_MUTEX.lock().unwrap();

    let mut output = utils::run(vec!["spin", "plugin", "update"], None, None)?;

    for plugin in plugins {
        output = utils::run(
            vec!["spin", "plugin", "install", plugin, "--yes"],
            None,
            None,
        )?;
    }

    //this ensure we have mutex lock until here
    drop(x);

    Ok(output)
}

pub fn build_app(appname: &str) -> Result<Output> {
    let appdir = appdir(appname);
    utils::run(vec!["spin", "build"], Some(&appdir), None)
}

pub fn appdir(appname: &str) -> String {
    let dir = Path::new(utils::testcases_base_dir().as_str()).join(appname);
    dir.into_os_string().into_string().unwrap()
}

#[cfg(target_family = "unix")]
pub async fn stop_app_process(process: &mut tokio::process::Child) -> Result<(), anyhow::Error> {
    let pid = process.id().unwrap();
    println!("stopping app with pid {}", pid);
    let pid = Pid::from_raw(pid as i32);
    kill(pid, Signal::SIGINT).map_err(anyhow::Error::msg)
}

#[cfg(target_family = "windows")]
pub async fn stop_app_process(process: &mut tokio::process::Child) -> Result<(), anyhow::Error> {
    // stop the app at the end of testcase
    let _ = &mut process.kill().await.map_err(anyhow::Error::msg);

    match process.wait().await {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::Error::msg(e)),
    }
}
