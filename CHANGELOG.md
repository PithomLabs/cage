# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0-alpha.3 - 2020-04-10

### Added

- When using `config/vault.yml`, it is now possible to use `no_default_policies` to indicate that a pod should not receive the default Vault policies.
- Some Windows debugging code has been added to try to figure out the template failures on Appveyor.

## 0.3.0-alpha.2 - 2020-04-09

### Added

- When running on Linux, we now set up `host.docker.internal` in the internal DNS.

### Changed

- The Vault plugin is now enabled on all platforms, including Windows.
- `boondock` now uses `rustls` on all platforms, which should help make Windows support a bit easier.

### Fixed

- The `cage status` command works again, thanks to an updated version of `boondock`.
- Logging and error message newlines have been fixed.

### Removed

- There are no longer any special `cargo` features to disable SSL, since it should now work everywhere.

## 0.3.0-alpha.1 - 2020-04-08

### Added

- Vault and OpenSSL support are enabled in Mac binaries by default.

### Changed

- The code has been updated to reasonably idiomatic Rust 2018.
- We have replaced our `boondock` Docker client with `dockworker`, which is a better-maintained fork of `boondock`. Many thanks to the `dockworker` maintainers!
- We have upgraded all of our immediate dependencies to something reasonably modern, and replaced a few of them.
- `compose_yml` has been upgraded to a similarly modernized version.

### Fixed

- `docker-compose.yml` validation has been re-enabled, thanks to the `compose_yml` update.
