<p align="center">
  <img width="180" src="./logo.png" alt="Elk native logo">
</p>
<h3 align="center">Elk Native</h3>
<br/>
<p align="center">
  <a href="https://chat.elk.zone"><img src="https://img.shields.io/badge/chat-discord-blue?style=flat&logo=discord" alt="discord chat"></a>
</p>
<br/>

Native version of Elk, a nimble Mastodon web client.

Elk Native is even more early alpha than the web version, but we would love your feedback and contributions.

The web client code lives at https://github.com/elk-zone/elk, a repository that will be open-sourced once we are ready to promote the app widely. If you would like to help us with testing, feedback, or contributing, join our [discord](https://chat.elk.zone) and get involved.

![Screenshot of the app, showing the federated timeline home](./Screenshot-light.png#gh-light-mode-only)
![Screenshot of the app, showing the federated timeline home](./Screenshot-dark.png#gh-dark-mode-only)

# Contributing

We're really excited that you're interested in contributing to Elk! Please read through the small guide below to get started:

> Note: 
> <br>
> This repo only holds the code necessary for the native app and just git-submodules the frontend from https://github.com/elk-zone/elk so please submit Issues and PRs concerning the frontend there! 

## Local Setup

First you will need to install Rust and other system dependencies required for developing a Tauri app, you can follow [this guide](https://tauri.app/v1/guides/getting-started/prerequisites).

First, clone the submodule in the `elk` folder.

```
git submodule update --init elk
```

Next install the frontend dependencies:

```
cd elk && pnpm i
```

Then you can run the app in development mode:

```
cargo tauri dev
```

To create a production build of the app:

```
cargo tauri build
```

## Logging

This app uses a unified logging stream that encompasses both logs from the Rust Core as well as from the frontend. Log levels and filters can be set through the `RUST_LOG` environment variable (this also works in a release mode app).

For more information on how to specify log levels and filters see [the env_logger docs](https://docs.rs/env_logger/latest/env_logger/index.html).

## License

[MIT](./LICENSE) &copy; 2022-PRESENT Elk Native contributors
