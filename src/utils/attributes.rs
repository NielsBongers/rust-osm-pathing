use quick_xml::events::attributes::Attributes;

use std::collections::HashMap;

pub fn read_attributes(attributes: Attributes) -> HashMap<String, String> {
    let mut attribute_hashmap: HashMap<String, String> = HashMap::new();

    for attribute in attributes {
        let attribute = attribute.expect("Failed to read attribute");

        let value = std::str::from_utf8(&attribute.value)
            .expect("Failed to convert into utf8")
            .to_string();

        let key = std::str::from_utf8(attribute.key.as_ref())
            .expect("Failed to read name")
            .to_string();

        attribute_hashmap.insert(key, value);
    }

    attribute_hashmap
}
