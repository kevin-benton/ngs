<p align="center">
  <h1 align="center">
    ngs
  </h1>

  <p align="center">
    <a href="https://github.com/stjude-rust-labs/ngs/actions/workflows/CI.yml" target="_blank">
      <img alt="CI: Status" src="https://github.com/stjude-rust-labs/ngs/actions/workflows/CI.yml/badge.svg" />
    </a>
    <a href="https://crates.io/crates/ngs" target="_blank">
      <img alt="crates.io version" src="https://img.shields.io/crates/v/ngs">
    </a>
    <img alt="crates.io downloads" src="https://img.shields.io/crates/d/ngs">
    <a href="https://github.com/stjude-rust-labs/ngs/blob/master/LICENSE.md" target="_blank">
      <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-blue.svg" />
    </a>
  </p>


  <p align="center">
    Command line utility for manipulating next-generation sequencing files. 
    <br />
    <a href="https://github.com/stjude-rust-labs/ngs/wiki"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/stjude-rust-labs/ngs/issues/new?assignees=&labels=&template=feature_request.md&title=Descriptive%20Title&labels=enhancement">Request Feature</a>
    ·
    <a href="https://github.com/stjude-rust-labs/ngs/issues/new?assignees=&labels=&template=bug_report.md&title=Descriptive%20Title&labels=bug">Report Bug</a>
    ·
    ⭐ Consider starring the repo! ⭐
    <br />
  </p>

  <p>
    <img src="https://raw.githubusercontent.com/stjude-rust-labs/ngs/main/.github/assets/experimental-warning.png">
  </p>
</p>


## 🎨 Features

* **[`ngs derive`](https://github.com/stjude-rust-labs/ngs/wiki/ngs-derive).** Forensic analysis tool useful in backwards computing information from next-generation sequencing data.
* **[`ngs generate`](https://github.com/stjude-rust-labs/ngs/wiki/ngs-generate).** Tool to generate next-generation sequencing files.
* **[`ngs qc`](https://github.com/stjude-rust-labs/ngs/wiki/ngs-qc).** Provides tools for checking the quality of next-generation sequencing files.

## 📚 Getting Started

### Installation

```bash
cargo install ngs
```

## 🖥️ Development

To bootstrap a development environment, please use the following commands.

```bash
# Clone the repository
git clone git@github.com:stjude-rust-labs/ngs.git
cd ngs

# Run the command line tool using cargo.
cargo run -- -h
```

## 🚧️ Tests

```bash
# Run the project's tests.
cargo test

# Ensure the project doesn't have any linting warnings.
cargo clippy
```

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check
[issues page](https://github.com/stjudecloud/oliver/issues).

## 📝 License

Copyright © 2021-Present 
[St. Jude Children's Research Hospital](https://github.com/stjude). This 
project is [MIT][license-md] licensed.

[contributing-md]: https://github.com/stjude-rust-labs/ngs/blob/master/CONTRIBUTING.md
[license-md]: https://github.com/stjude-rust-labs/ngs/blob/master/LICENSE.md