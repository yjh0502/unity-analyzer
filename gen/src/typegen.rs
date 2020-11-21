use super::*;

pub fn filter_ty(ty: Ty, spec: &FilterSpec) -> Ty {
    match ty {
        Ty::Map(inner) => {
            let mut matched = true;
            for neg_key in spec.neg_keys.iter() {
                let key_found = inner.iter().find(|(k, _v)| k == neg_key).is_some();
                if key_found {
                    matched = false;
                    break;
                }
            }

            for key in spec.keys.iter() {
                let key_found = inner.iter().find(|(k, _v)| k == key).is_some();
                if !key_found {
                    matched = false;
                    break;
                }
            }

            if !matched {
                let inner = inner
                    .into_iter()
                    .map(|(k, v)| (k, filter_ty(v, spec)))
                    .collect();
                Ty::Map(inner)
            } else {
                spec.alt.clone()
            }
        }
        Ty::Seq(inner) => Ty::Seq(Box::new(filter_ty(*inner, spec))),
        Ty::Some(inner) => Ty::Some(Box::new(filter_ty(*inner, spec))),
        other => other,
    }
}

pub struct FilterSpec {
    pub keys: Vec<String>,
    pub neg_keys: Vec<String>,
    pub alt: Ty,
}

impl FilterSpec {
    pub fn new(keys: &[&str], neg_keys: &[&str], alt: &[(&str, Ty)]) -> Self {
        let keys = keys.iter().map(|k| (*k).to_owned()).collect();
        let neg_keys = neg_keys.iter().map(|k| (*k).to_owned()).collect();
        let alt = Ty::Map(
            alt.iter()
                .map(|(k, ty)| ((*k).to_owned(), ty.clone()))
                .collect(),
        );
        Self {
            keys,
            neg_keys,
            alt,
        }
    }
}
