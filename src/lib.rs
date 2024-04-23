use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "pkgx",
            "install",
            &format!("maven.apache.org@{}", version),
        ])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn validate(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "validate", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn compile(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "compile", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "test", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn package(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "package", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn intergation_test(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "integration-test", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn verify(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "verify", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn deploy(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "deploy", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn clean(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "clean", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn site(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "mvn", "site", &args])?
        .stdout()?;
    Ok(stdout)
}
