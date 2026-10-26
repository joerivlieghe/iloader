# Windows Certificate Trust Build

This fork adds the Windows-only `rustls-tls-native-roots` feature to the same
`tokio-tungstenite` dependency used by isideload's provisioning WebSocket.
Rustls loads Windows' trusted roots alongside the existing public roots.
Certificate-chain, expiry, and hostname validation remain enabled. This is
Rustls verification using Windows roots, not the full Windows Schannel verifier.
No proxy, endpoint-security, or certificate-store settings are changed.

## Build

Run **Windows System Trust Build** from GitHub Actions on this fork. The workflow
uses the committed lockfile and builds a credential-free diagnostic, the app,
and an unsigned NSIS installer. No Apple credentials or signing secrets are needed.
The existing CI configuration disables the upstream updater for this custom build.
Only run a custom unsigned application if your organization's policy permits it.

## Verify On The Affected Windows PC

Run `websocket_trust_check.exe` from the `windows-websocket-trust-check` artifact.
It first confirms that an empty certificate trust store rejects the connection.
It then connects to StikStore's provisioning WebSocket using the application's
default TLS path and closes immediately. Both checks have bounded timeouts.
It sends no Apple credentials, device identifiers, or provisioning payloads.

The diagnostic must pass on the affected PC before claiming the TLS issue is fixed.
Cloud build success alone does not test an enterprise certificate chain.
Apple sign-in, provisioning, signing, USB detection, and installation still need
to be completed separately in the app. Keep the existing settings and keychain;
this patch does not require deleting account or device data.