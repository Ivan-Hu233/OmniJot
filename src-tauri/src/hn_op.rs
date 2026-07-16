use quick_xml::se::to_string;
use quick_xml::de::from_str;
use platform_dirs::{UserDirs};
use crate::hn_struct::HyperNote;

pub fn get_hypernote(file_name: &str) -> Result<HyperNote, quick_xml::DeError> {
    let data = std::fs::read_to_string(UserDirs::new().unwrap().document_dir.join("Nope").join(file_name.to_string() + ".hn")).unwrap();
    deserialize_hn_xml(&data)
}

fn deserialize_hn_xml(xml: &str) -> Result<HyperNote, quick_xml::DeError> {
    from_str(xml)
}

fn serialize_hn_xml(hn: &HyperNote) -> Result<String, quick_xml::SeError> {
    to_string(hn)
}