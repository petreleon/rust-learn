use anyhow::Result;
use aws_config::{self, BehaviorVersion};
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use std::env;
use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::infra::notifications::NotificationsState;
use chrono::Utc;
use reqwest::Client as ReqwestClient;
use std::process::Stdio;
use tokio::fs as tokio_fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command as TokioCommand;

/// Async S3 client wrapper. Works with any S3-compatible service (AWS S3, RustFS, etc.).
#[derive(Clone)]
pub struct S3State(Arc<Client>);

fn configured_region() -> Region {
    Region::new(env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into()))
}

fn internal_endpoint_from_env() -> String {
    let host = env::var("S3_INTERNAL_DOMAIN").unwrap_or_else(|_| "rustfs".into());
    let port = env::var("S3_INTERNAL_PORT").unwrap_or_else(|_| "9000".into());
    let scheme = env::var("S3_INTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
    let host = normalize_internal_s3_host_for_debug(
        host,
        &port,
        cfg!(debug_assertions),
        host_port_resolves,
    );

    format!("{}://{}:{}", scheme, host, port)
}

fn normalize_internal_s3_host_for_debug<F>(
    host: String,
    port: &str,
    is_debug_build: bool,
    host_resolves: F,
) -> String
where
    F: Fn(&str, u16) -> bool,
{
    if !is_debug_build || host != "rustfs" {
        return host;
    }

    let Ok(port) = port.parse::<u16>() else {
        return host;
    };

    if host_resolves("rustfs", port) {
        host
    } else {
        "localhost".to_string()
    }
}

fn host_port_resolves(host: &str, port: u16) -> bool {
    (host, port)
        .to_socket_addrs()
        .map(|mut addresses| addresses.next().is_some())
        .unwrap_or(false)
}

async fn configured_client(endpoint: String) -> Client {
    let user = env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "rustfsadmin".into());
    let pass = env::var("S3_SECRET_KEY").unwrap_or_else(|_| "rustfsadmin".into());
    let creds = Credentials::new(user, pass, None, None, "env");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(creds)
        .region(configured_region())
        .endpoint_url(endpoint)
        .load()
        .await;
    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .force_path_style(true)
        .build();

    Client::from_conf(s3_config)
}
