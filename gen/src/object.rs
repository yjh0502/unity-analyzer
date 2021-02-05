use super::*;
use parse::{UnityYaml, UnityYamlObject};

// type Yaml = serde_yaml::Value;
pub type Yaml = yaml_rust::yaml::Yaml;

#[derive(Debug)]
pub struct Object<'a> {
    pub header: parse::ObjectHeader,
    pub references: Vec<Reference>,
    pub ty_name: &'a str,

    pub parsed: UnityYaml<'a>,
}

pub(crate) fn try_find_value_str<'a, 'b, 'c>(
    value: &'b UnityYaml<'a>,
    key: &'c str,
) -> Option<&'a str> {
    if let Some(parse::LineValue::Map(ref kv)) = value.as_value() {
        for (k, v) in kv {
            if *k == key {
                return Some(*v);
            }
        }
    }

    if !value.is_object() {
        return None;
    }

    for item in value.iter() {
        if item.key() == Some(key) {
            return item.as_str();
        }
    }
    None
}

pub(crate) fn try_find_value<'a, 'b, 'c>(
    value: &'b UnityYaml<'a>,
    key: &'c str,
) -> Option<UnityYaml<'a>> {
    if !value.is_object() {
        return None;
    }

    for item in value.iter() {
        if item.key() == Some(key) {
            return Some(item);
        }
    }
    None
}

pub(crate) fn try_get_file_id<'a>(value: &'a UnityYaml<'a>) -> Option<i64> {
    let v = try_find_value_str(value, "fileID")?;
    v.parse().ok()
}

pub(crate) fn try_get_guid<'a, 'b>(value: &'b UnityYaml<'a>) -> Option<&'a str> {
    try_find_value_str(value, "guid")
}

impl<'a> Object<'a> {
    pub fn from_yaml_object(obj: UnityYamlObject<'a>) -> Result<Object<'a>> {
        let header = obj.header.clone();
        let parsed = obj.iter().next().unwrap();

        let mut references = Vec::new();
        find_references(&parsed, &mut references)?;

        Ok(Object {
            header,
            references,
            ty_name: parsed.key().unwrap(),

            parsed,
        })
    }

    pub fn gameobject(&self) -> Option<i64> {
        if self.ty_name == "GameObject" {
            Some(self.header.file_id)
        } else {
            let obj = try_find_value(&self.parsed, "m_GameObject")?;
            try_get_file_id(&obj)
        }
    }

    fn references_vec(&self, key: &str) -> Option<Vec<i64>> {
        let component = try_find_value(&self.parsed, key)?;
        if !component.is_array() {
            return None;
        }

        Some(
            component
                .iter()
                .filter_map(|v| try_get_file_id(&v))
                .collect(),
        )
    }

    pub fn components(&self) -> Option<Vec<i64>> {
        let component = try_find_value(&self.parsed, "m_Component")?;

        if !component.is_array() {
            return None;
        }

        Some(
            component
                .iter()
                .filter_map(|m| {
                    if !m.is_object() {
                        return None;
                    }

                    let first = m.iter().next().expect("empty map");
                    try_get_file_id(&first)
                })
                .collect(),
        )
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
        try_get_file_id(&obj)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        try_find_value_str(&self.parsed, key)
    }

    pub fn is_prefab_transform(&self) -> bool {
        self.header.stripped
    }

    pub fn dbg_yaml(&self) -> Result<String> {
        todo!();
    }
}

fn find_references(node: &UnityYaml, out: &mut Vec<Reference>) -> Result<()> {
    if let Some(parse::LineValue::Map(ref vec)) = node.as_value() {
        let mut file_id = None;
        let mut guid = None;

        for (k, v) in vec {
            if *k == "fileID" {
                file_id = Some(*v);
            } else if *k == "guid" {
                guid = Some((*v).to_owned());
            }
        }

        if let Some(file_id) = file_id {
            if let Ok(file_id) = file_id.parse::<i64>() {
                out.push(Reference { file_id, guid });
            }
        }
    }

    for item in node.iter() {
        find_references(&item, out)?;
    }

    Ok(())
}
