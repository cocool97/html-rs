/// Generates an attibute pair name="value" if needed.
pub fn generate_attribute(attr_name: &str, attr_value: &Option<String>) -> String {
    if let Some(value) = attr_value {
        format!(" {}=\"{}\"", attr_name, value)
    } else {
        String::from("")
    }
}
