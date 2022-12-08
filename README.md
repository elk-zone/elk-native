<p align="center">
  <img width="180" src="./logo.png" alt="Elk desktop logo">
</p>
<h3 align="center">Elk Desktop</h3>
<br/>
<p align="center">
  <a href="https://chat.elk.zone"><img src="https://img.shields.io/badge/chat-discord-blue?style=flat&logo=discord" alt="discord chat"></a>
  <a href="https://pr.new/elk-zone/elk"><img src="https://developer.stackblitz.com/img/start_pr_dark_small.svg" alt="Start new PR in StackBlitz Codeflow"></a>
</p>
<br/>

Desktop version of Elk, a nimble Mastodon web client made with 🧡 

Elk Desktop is even more early alpha than the web version, but we would love your feedback and contributions.

![Screenshot of the app, showing the federated timeline home](./Screenshot-light.png#gh-light-mode-only)
![Screenshot of the app, showing the federated timeline home](./Screenshot-dark.png#gh-dark-mode-only)

# Contributing

We're really excited that you're interested in contributing to Elk! Please read through the small guide below to get started:

> Note: 
> <br>
> This repo only holds the code necessary for the desktop app and just git-submodules the frontend from https://github.com/elk-zone/elk so please submit Issues and PRs concerning the frontend there! 

## Local Setup

First you will need to install Rust and other system dependencies required for developing a Tauri app, you can follow [this guide](https://tauri.app/v1/guides/getting-started/prerequisites). Then install the frontend dependencies:

```
cd elk
```

Then you can run the app in development mode:

```
cargo tauri dev
```

To create a production build of the app:

```
cargo tauri build
```