use anyhow::Context;

use crate::{
    Runtime,
    spin::{SpinMode, IoMode, Request},
};
use std::str;
use anyhow::anyhow;

use std::{
    path::Path,
    collections::HashMap,
};
use regex::Regex;

/// A wrapper around a running Spin instance
pub struct Cloud {
    io_mode: IoMode,
}

impl Cloud {
    pub fn start(
        spin_binary_path: &Path,
        current_dir: &Path,
        spin_up_args: Vec<impl AsRef<std::ffi::OsStr>>,
        mode: SpinMode,
    ) -> anyhow::Result<Self> {
        match mode {
            SpinMode::Http => Self::start_http(spin_binary_path, current_dir, spin_up_args),
            SpinMode::Redis => Self::start_redis(spin_binary_path, current_dir, spin_up_args),
        }
    }

    /// Start Spin in `current_dir` using the binary at `spin_binary_path`
    pub fn start_http(
        _: &Path,
        current_dir: &Path,
        _: Vec<impl AsRef<std::ffi::OsStr>>,
    ) -> anyhow::Result<Self> {
        let cmd = vec!["spin", "deploy"];
        let env: HashMap<&str, &str> = [(
            "FERMYON_DEPLOYMENT_ENVIRONMENT",
            "cloud-e2e-tests",
        )]
        .iter()
        .cloned()
        .collect();

        let output = crate::utils::run(&cmd, Some(&current_dir), Some(env))?;

        let logs = match str::from_utf8(&output.stdout) {
            Ok(logs) => logs,
            Err(_) => return Err(anyhow!("")),
        };

        let appname_from_logs = extract_appname_from_logs(logs);

        //TODO: fetch dynamically using api call
        let app = crate::cloud_api::get_app_by_name(&appname_from_logs)?;

        let cloudapp = Self {
            io_mode: IoMode::HttpUrl(app.domain.unwrap().name),
        };
        let start = std::time::Instant::now();
        loop {
            //TODO: get from app
            let domain = "domain".to_string();
            match std::net::TcpStream::connect(domain.clone()) {
                Ok(_) => {
                    log::debug!("Spin started on {:?}.", domain);
                    return Ok(cloudapp);
                }
                Err(e) => {
                    // let stderr = spin.stderr.output_as_str().unwrap_or("<non-utf8>");
                    log::trace!("Checking that the Spin server started returned an error: {e}");
                    log::trace!("Current spin stderr = 'TODO'");
                }
            }

            if start.elapsed() > std::time::Duration::from_secs(2 * 60) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        anyhow::bail!(
            "`spin up` did not start server or error after two minutes. stderr:\n\tTODO")
    }

    pub fn start_redis(
        _: &Path,
        _: &Path,
        _: Vec<impl AsRef<std::ffi::OsStr>>,
    ) -> anyhow::Result<Self> {
        Err(anyhow!("redis not supported on cloud"))
    }

    /// Make an HTTP request against Spin
    ///
    /// Will fail if Spin has already exited or if the io mode is not HTTP
    pub fn make_http_request(
        &mut self,
        request: Request<'_>,
    ) -> anyhow::Result<reqwest::blocking::Response> {
        let IoMode::HttpUrl(ref url) = self.io_mode else {
            anyhow::bail!("Spin is not running in HTTP mode");
        };
       
        log::debug!("Connecting to HTTP server on {}...", url.clone());
        let mut outgoing = reqwest::blocking::Request::new(
            request.method,
            reqwest::Url::parse(&url)
                .unwrap()
                .join(request.uri)
                .context("could not construct url for request against Spin")?,
        );
        outgoing
            .headers_mut()
            .extend(request.headers.iter().map(|(k, v)| {
                (
                    reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                    reqwest::header::HeaderValue::from_str(v).unwrap(),
                )
            }));
        *outgoing.body_mut() = request.body.map(Into::into);
        let response = reqwest::blocking::Client::new().execute(outgoing)?;
        log::debug!("Awaiting response from server");
        Ok(response)
    }
}

impl Drop for Cloud {
    fn drop(&mut self) {
        //TODO: undeploy app
    }
}

impl Runtime for Cloud {
    fn error(&mut self) -> anyhow::Result<()> {
        //TODO
        Ok(())
    }
}

/// Extracts name of app being deployed by parsing logs
pub fn extract_appname_from_logs(logs: &str) -> String {
    println!("logs are\n{}", logs);
    let re: Regex =
        Regex::new(format!("^Uploading (.*) version (.*) to Fermyon Cloud....*").as_str()).unwrap();
    let v = match re.captures(logs) {
        None => "",
        Some(v) => v.get(1).unwrap().as_str(),
    };

    v.to_string()
}