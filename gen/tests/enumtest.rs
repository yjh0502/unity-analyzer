use serde_derive::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Serialize, Deserialize, Debug)]
enum Root {
    Foo(Foo),
    Bar(Bar),
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    a: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bar {
    x: usize,
}

fn main() -> Result<()> {
    let yaml = r#"
---
Foo:
  a: 1
"#;

    let parsed: Root = serde_yaml::from_str(yaml)?;
    eprintln!("{:?}", parsed);

    Ok(())
}
