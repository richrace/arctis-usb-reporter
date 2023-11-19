use crate::headphone_helpers::found_headphone::VENDOR_ID;
use std::collections::HashMap;

pub struct KnownHeadphone<'a> {
    pub name: &'static str,
    pub vendor_id: u16,
    pub product_id: u16,
    pub write_bytes: &'a [u8; 2],
    pub usage_page: u16,
    pub usage: u16,
    pub interface: i32,
    pub battery_percent_index: usize,
    pub charging_status_index: Option<usize>,
    pub mic_status_index: Option<usize>,
}

pub fn get_known_headphones() -> HashMap<u16, KnownHeadphone<'static>> {
    let mut known_headphones = HashMap::new();

    known_headphones.insert(
        0x12d7,
        KnownHeadphone {
            name: "Arctis 7X",
            vendor_id: VENDOR_ID,
            product_id: 0x12d7,
            write_bytes: &[0x06, 0x12],
            usage_page: 0xff43,
            usage: 0x202,
            interface: 0x03,
            battery_percent_index: 3,
            charging_status_index: Some(4),
            mic_status_index: Some(5),
        },
    );

    known_headphones.insert(
        0x12b3,
        KnownHeadphone {
            name: "Arctis 1 Wireless",
            vendor_id: VENDOR_ID,
            product_id: 0x12b3,
            write_bytes: &[0x06, 0x12],
            usage_page: 0xff43,
            usage: 0x202,
            interface: 0x03,
            battery_percent_index: 3,
            charging_status_index: Some(4),
            mic_status_index: None,
        },
    );

    return known_headphones;
}
