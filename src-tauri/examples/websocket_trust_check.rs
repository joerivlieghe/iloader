#[cfg(not(windows))]
fn main() {
    eprintln!("This diagnostic must run on Windows.");
    std::process::exit(1);
}

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{sync::Arc, time::Duration};
    use tokio::time::timeout;
    use tokio_tungstenite::{Connector, connect_async, connect_async_tls_with_config};

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| "Failed to install the TLS crypto provider")?;

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let url = "wss://ani3.stikstore.app/v3/provisioning_session";
            let untrusted = rustls::ClientConfig::builder()
                .with_root_certificates(rustls::RootCertStore::empty())
                .with_no_client_auth();
            let untrusted_result = timeout(
                Duration::from_secs(30),
                connect_async_tls_with_config(
                    url,
                    None,
                    false,
                    Some(Connector::Rustls(Arc::new(untrusted))),
                ),
            )
            .await?;

            match untrusted_result {
                Err(error) if error.to_string().contains("UnknownIssuer") => {
                    println!("PASS: a certificate without a trusted root is rejected.");
                }
                Err(error) => {
                    return Err(format!("Unexpected negative-check failure: {error}").into());
                }
                Ok(_) => return Err("FAIL: the untrusted certificate was accepted.".into()),
            }

            let (mut socket, response) = timeout(Duration::from_secs(30), connect_async(url))
                .await??;
            println!(
                "PASS: provisioning WebSocket connected using system roots (HTTP {}).",
                response.status().as_u16()
            );
            timeout(Duration::from_secs(5), socket.close(None)).await??;
            println!("No Apple credentials, identifiers, or provisioning data were sent.");
            Ok::<(), Box<dyn std::error::Error>>(())
        })
}