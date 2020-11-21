use super::*;

/// yaml 파일에 대한 정보. prefab/scene 등이 이에 해당합니다.
/// 내부에서 object tree 구조를 가지고 있습니다.
/// `--- !u!1001 &33075763` 에서, object_id=101, file_id=33075763
#[derive(Debug)]
pub struct ObjectHeader {
    /// file-local object id. 따로 쓰는 것 같지는 않지만 일단 파싱합니다
    pub object_id: u64,
    /// fileID
    pub file_id: i64,
}

impl ObjectHeader {
    pub fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        if !s.starts_with("!u!") {
            bail!("unknown header");
        }
        let s = &s[3..];
        let mut split = s.split(" ");

        let object_id = match split.next() {
            Some(s) => s.parse::<u64>()?,
            None => bail!("unknown header"),
        };

        let file_id = match split.next() {
            Some(s) => {
                if !s.starts_with("&") {
                    bail!("unknown header");
                }
                (&s[1..]).parse::<i64>()?
            }
            None => bail!("unknown header"),
        };

        Ok(Self { object_id, file_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        ObjectHeader::from_str("!u!29 &1").expect("failed to parse");
    }
}
