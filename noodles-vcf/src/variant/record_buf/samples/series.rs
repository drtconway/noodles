use std::io;

use super::sample::Value;

/// A variant record samples buffer series.
pub struct Series<'a> {
    name: &'a str,
    values: &'a [Vec<Option<Value>>],
    i: usize,
}

impl<'a> Series<'a> {
    pub(super) fn new(name: &'a str, values: &'a [Vec<Option<Value>>], i: usize) -> Self {
        Self { name, values, i }
    }

    /// Returns the value at the given index.
    pub fn get(&self, i: usize) -> Option<Option<&Value>> {
        self.values
            .get(i)
            .map(|sample| sample.get(self.i).and_then(|value| value.as_ref()))
    }
}

impl<'a> crate::variant::record::samples::Series for Series<'a> {
    fn name(&self) -> &str {
        self.name
    }

    fn iter(
        &self,
    ) -> Box<
        dyn Iterator<Item = io::Result<Option<crate::variant::record::samples::series::Value<'_>>>>
            + '_,
    > {
        Box::new(self.values.iter().map(|sample| {
            Ok(sample
                .get(self.i)
                .and_then(|value| value.as_ref().map(|v| v.into())))
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variant::{record::samples::Series as _, record_buf::samples::keys::key};

    #[test]
    fn test_name() {
        let series = Series::new(key::GENOTYPE, &[], 0);
        assert_eq!(series.name(), key::GENOTYPE);
    }

    #[test]
    fn test_get() {
        let values = [
            vec![Some(Value::from("0|0")), Some(Value::from(7))],
            vec![Some(Value::from("1/1"))],
            vec![],
        ];

        let series = Series::new(key::GENOTYPE, &values, 0);
        assert_eq!(series.get(0), Some(Some(&Value::from("0|0"))));
        assert_eq!(series.get(1), Some(Some(&Value::from("1/1"))));
        assert_eq!(series.get(2), Some(None));
        assert_eq!(series.get(3), None);

        let series = Series::new(key::CONDITIONAL_GENOTYPE_QUALITY, &values, 1);
        assert_eq!(series.get(0), Some(Some(&Value::from(7))));
        assert_eq!(series.get(1), Some(None));
        assert_eq!(series.get(2), Some(None));
        assert_eq!(series.get(3), None);
    }
}
