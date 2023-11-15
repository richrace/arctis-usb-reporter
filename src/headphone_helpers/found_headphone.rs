use std::ffi::CStr;

pub static VENDOR_ID: u16 = 4152;

pub struct FoundHeadphone {
    pub name: String,
    pub product_id: u16,
    pub path: String,
    pub usage_page: u16,
    pub usage: u16,
    pub interface: i32,
}

pub fn build_found_headphone(
    name: Option<&str>,
    product_id: u16,
    path: &CStr,
    usage_page: u16,
    usage: u16,
    interface: i32,
) -> FoundHeadphone {
    let product_name: String;

    match name {
        Some(s) => product_name = s.to_owned(),
        None => product_name = "".to_owned(),
    }

    return FoundHeadphone {
        name: product_name,
        product_id,
        path: path.to_string_lossy().into_owned(),
        usage_page,
        usage,
        interface,
    };
}
