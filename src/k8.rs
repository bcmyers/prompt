use std::convert::TryFrom;
use std::process::Command;

use anyhow::anyhow;
// use colored::Colorize;
use serde::{Deserialize, Serialize};

pub(crate) fn k8() -> Option<String> {
    let command = &["kubectl", "config", "view", "--minify", "-o", "json"];

    let output = Command::new(command[0])
        .args(command.iter().skip(1))
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let k = serde_json::from_slice::<K>(&output.stdout).ok()?;
    let output = Output::try_from(k).ok()?;
    let ns = output.namespace?;

    let mut s = String::new();
    s.push_str(&format!(" cluster:   {}\n", output.cluster));
    s.push_str(&format!(" context:   {}\n", ns));
    s.push_str(&format!(" namespace: {}", ns));

    Some(s)
}

// fn colorize<S>(s: S) -> String
// where
//     S: AsRef<str>,
// {
//     format!(" {} ", s.as_ref())
//         .bold()
//         .color("bright white")
//         .on_color("bright blue")
//         .to_string()
// }

#[derive(Clone, Debug, Deserialize)]
struct K {
    contexts: Vec<KContext>,
    #[serde(rename = "current-context")]
    current_context: String,
}

#[derive(Clone, Debug, Deserialize)]
struct KContext {
    name: String,
    #[serde(rename = "context")]
    inner: KInner,
}

#[derive(Clone, Debug, Deserialize)]
struct KInner {
    cluster: String,
    namespace: Option<String>,
    user: String,
}

#[derive(Clone, Debug, Serialize)]
struct Output {
    cluster: String,
    context: String,
    namespace: Option<String>,
    user: String,
}

impl TryFrom<K> for Output {
    type Error = anyhow::Error;

    fn try_from(k: K) -> Result<Self, Self::Error> {
        let current = k.current_context;
        let context = k
            .contexts
            .into_iter()
            .find(|item| &item.name == &current)
            .ok_or_else(|| anyhow!("Missing current context details"))?;
        Ok(Self {
            context: current,
            cluster: context.inner.cluster,
            namespace: context.inner.namespace,
            user: context.inner.user,
        })
    }
}
