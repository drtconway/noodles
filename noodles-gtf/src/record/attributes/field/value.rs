use std::{borrow::Cow, io, iter};

use bstr::BStr;
use noodles_gff as gff;

#[derive(Debug, Eq, PartialEq)]
pub enum Value<'r> {
    String(Cow<'r, BStr>),
    Array(Vec<Cow<'r, BStr>>),
}

impl<'r> Value<'r> {
    /// An iterator over values.
    pub fn iter(&self) -> Box<dyn Iterator<Item = &Cow<'r, BStr>> + '_> {
        match self {
            Self::String(value) => Box::new(iter::once(value)),
            Self::Array(values) => Box::new(values.iter()),
        }
    }

    pub(crate) fn push(&mut self, s: Cow<'r, BStr>) {
        match self {
            Self::String(t) => {
                let values = vec![t.clone(), s];
                *self = Self::Array(values);
            }
            Self::Array(array) => array.push(s),
        }
    }
}

impl<'r> From<&'r Value<'r>> for gff::feature::record::attributes::field::Value<'r> {
    fn from(value: &'r Value<'_>) -> Self {
        match value {
            Value::String(value) => Self::String(value.clone()),
            Value::Array(values) => Self::Array(Box::new(Array(values))),
        }
    }
}

struct Array<'r>(&'r [Cow<'r, BStr>]);

impl<'r> gff::feature::record::attributes::field::value::Array<'r> for Array<'r> {
    fn iter(&self) -> Box<dyn Iterator<Item = io::Result<Cow<'r, BStr>>> + 'r> {
        Box::new(self.0.iter().map(|value| Ok(value.clone())))
    }
}

impl From<&Value<'_>> for gff::feature::record_buf::attributes::field::Value {
    fn from(value: &Value<'_>) -> Self {
        match value {
            Value::String(value) => Self::String(value.as_ref().into()),
            Value::Array(values) => {
                let vs: Vec<_> = values.iter().map(|s| s.as_ref().into()).collect();
                Self::from(vs)
            }
        }
    }
}
