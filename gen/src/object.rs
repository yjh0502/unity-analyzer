use super::{objectheader::ObjectHeader, *};

// type Yaml = serde_yaml::Value;
pub type Yaml = yaml_rust::yaml::Yaml;

#[derive(Debug)]
pub struct Object {
    pub header: ObjectHeader,
    pub references: Vec<Reference>,
    pub ty_name: String,

    pub parsed: Yaml,
}

fn try_find_value<'a, 'b>(value: &'a Yaml, key: &'b str) -> Option<&'a Yaml> {
    let m = value.as_hash()?;

    for (k, v) in m.iter() {
        if let Yaml::String(ref key_str) = k {
            if key == key_str {
                return Some(v);
            }
        }
    }
    None
}

pub(crate) fn try_get_file_id(value: &Yaml) -> Option<i64> {
    try_find_value(value, "fileID")?.as_i64()
}

pub(crate) fn try_get_guid(value: &Yaml) -> Option<&str> {
    try_find_value(value, "guid")?.as_str()
}

impl Object {
    pub fn from_header_body(header: ObjectHeader, body: &str) -> Result<Object> {
        let mut parsed = yaml_rust::yaml::YamlLoader::load_from_str(body)?;

        let (ty_name, parsed) = match parsed.pop().unwrap() {
            Yaml::Hash(v) => {
                assert_eq!(v.len(), 1);
                match v.into_iter().next().unwrap() {
                    (Yaml::String(s), v) => (s.to_owned(), v),
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
        if let Yaml::Array(ref s) = component {
            Some(s.iter().filter_map(try_get_file_id).collect())
        } else {
            None
        }
    }

    pub fn components(&self) -> Option<Vec<i64>> {
        let component = try_find_value(&self.parsed, "m_Component")?;
        if let Yaml::Array(ref s) = component {
            Some(
                s.iter()
                    .filter_map(|m| {
                        let m = match m {
                            Yaml::Hash(m) => m,
                            _ => return None,
                        };
                        let tup = m.iter().next().expect("empty map");
                        match tup.0 {
                            Yaml::String(s) => {
                                assert_eq!(s, "component");
                            }
                            _ => {
                                todo!();
                            }
                        };
                        try_get_file_id(tup.1)
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    // transform
    pub fn is_transform(&self) -> bool {
        self.ty_name == "Transform" || self.ty_name == "RectTransform"
    }
    pub fn children(&self) -> Option<Vec<i64>> {
        self.references_vec("m_Children")
    }

    pub fn father(&self) -> Option<i64> {
        let obj = try_find_value(&self.parsed, "m_Father")?;
        try_get_file_id(obj)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        let obj = try_find_value(&self.parsed, key)?;
        if let Yaml::String(ref s) = obj {
            Some(s)
        } else {
            None
        }
    }

    pub fn is_prefab_transform(&self) -> bool {
        self.header.tag == "stripped"
    }

    pub fn dbg_yaml(&self) -> Result<String> {
        todo!();
    }
}

fn find_references(node: &Yaml, out: &mut Vec<Reference>) -> Result<()> {
    use yaml_rust::Yaml::*;

    match node {
        Hash(v) => {
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
                if let Some(file_id) = file_id.as_i64() {
                    out.push(Reference {
                        file_id,
                        guid: guid.and_then(|v| v.as_str()).map(|v| v.to_owned()),
                    });
                }
            }
        }
        Array(v) => {
            for item in v.iter() {
                find_references(item, out)?;
            }
        }
        _ => {}
    }

    Ok(())
}
