use super::{objectheader::ObjectHeader, *};

#[derive(Debug)]
pub struct Object {
    pub header: ObjectHeader,
    pub references: Vec<Reference>,
    pub ty_name: String,

    pub parsed: serde_yaml::Value,
}

impl Object {
    pub fn from_header_body(header: ObjectHeader, body: &str) -> Result<Object> {
        let parsed = serde_yaml::from_str::<serde_yaml::Value>(body)?;
        let (ty_name, parsed) = match parsed {
            serde_yaml::Value::Mapping(v) => {
                assert_eq!(v.len(), 1);
                match v.into_iter().next().unwrap() {
                    (serde_yaml::Value::String(s), v) => (s.to_owned(), v),
                    _ => todo!(),
                }
            }
            _ => {
                todo!();
            }
        };

        let mut references = Vec::new();
        find_references(&parsed, &mut references)?;

        Ok(Object {
            header,
            references,
            ty_name,

            parsed,
        })
    }
}

fn find_references(node: &serde_yaml::Value, out: &mut Vec<Reference>) -> Result<()> {
    use serde_yaml::Value::*;

    match node {
        Mapping(v) => {
            let mut file_id = None;
            let mut guid = None;
            for (k, v) in v.iter() {
                if let String(key) = k {
                    if key == "fileID" {
                        file_id = Some(v);
                    } else if key == "guid" {
                        guid = Some(v);
                    }
                }
                find_references(v, out)?;
            }

            if let Some(file_id) = file_id {
                if let Some(file_id) = file_id.as_u64() {
                    out.push(Reference {
                        file_id: file_id as usize,
                        guid: guid.and_then(|v| v.as_str()).map(|v| v.to_owned()),
                    });
                }
            }
        }
        Sequence(v) => {
            for item in v.iter() {
                find_references(item, out)?;
            }
        }
        _ => {}
    }

    Ok(())
}
