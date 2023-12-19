use crate::parser::adapter::adapt_variety_of_spelling;

type OrthographicalVariant = Vec<&'static str>;

pub struct OrthographicalVariantAdapter {
    pub variant_list: Vec<OrthographicalVariant>,
}

impl OrthographicalVariantAdapter {
    pub fn apply(&self, input: &str, region_name: &str) -> Option<(String, String)> {
        for variant in self.variant_list.clone() {
            if let Some(result) = adapt_variety_of_spelling(input, region_name, variant) {
                return Some(result);
            };
        }
        None
    }
}
