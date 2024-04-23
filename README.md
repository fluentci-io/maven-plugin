# Maven Plugin

[![ci](https://github.com/fluentci-io/maven-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/maven-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [maven](https://maven.apache.org/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm maven setup
```

## Functions

| Name   | Description                                |
| ------ | ------------------------------------------ |
| setup  | Installs a specific version of maven.      |
| validate |  validate the project is correct and all necessary information is available |
| compile | compile the source code of the project |
| test | test the compiled source code using a suitable unit testing framework |
| package | take the compiled code and package it in its distributable format, such as a JAR. |
| integration-test | process and deploy the package if necessary into an environment where integration tests can be run |
| verify | run any checks to verify the package is valid and meets quality criteria |
| install | install the package into the local repository, for use as a dependency in other projects locally |
| deploy | done in the build environment, copies the final package to the remote repository for sharing with other developers and projects. |
| clean | cleans up artifacts created by prior builds |
| site  | generates site documentation for the project |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/maven@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: maven
    args: |
      setup
    working-directory: example
- name: Show maven version
  run: |
    type mvn
    mvn --version
```
