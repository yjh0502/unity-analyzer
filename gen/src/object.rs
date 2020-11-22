use super::{objectheader::ObjectHeader, *};

#[derive(Debug)]
pub struct Object {
    pub header: ObjectHeader,
    pub references: Vec<Reference>,
    pub ty_name: String,

    pub parsed: serde_yaml::Value,
}

fn try_find_value<'a, 'b>(
    value: &'a serde_yaml::Value,
    key: &'b str,
) -> Option<&'a serde_yaml::Value> {
    let m = match value {
        serde_yaml::Value::Mapping(m) => m,
        _ => return None,
    };

    for (k, v) in m.iter() {
        if let serde_yaml::Value::String(ref key_str) = k {
            if key == key_str {
                return Some(v);
            }
        }
    }
    None
}

fn try_get_file_id(value: &serde_yaml::Value) -> Option<i64> {
    try_find_value(value, "fileID")?.as_i64()
}

pub const TY_TRANSFORM: &'static str = "Transform";

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

    pub fn gameobject(&self) -> Option<i64> {
        if self.ty_name == "GameObject" {
            Some(self.header.file_id)
        } else {
            let obj = try_find_value(&self.parsed, "m_GameObject")?;
            try_get_file_id(obj)
        }
    }

    fn references_vec(&self, key: &str) -> Option<Vec<i64>> {
        let component = try_find_value(&self.parsed, key)?;
        if let serde_yaml::Value::Sequence(ref s) = component {
            Some(s.iter().filter_map(try_get_file_id).collect())
        } else {
            None
        }
    }

    pub fn components(&self) -> Option<Vec<i64>> {
        self.references_vec("m_Component")
    }

    pub fn children(&self) -> Option<Vec<i64>> {
        self.references_vec("m_Children")
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
