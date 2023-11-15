use hidapi::HidApi;

use super::found_headphone;

extern crate hidapi;

pub fn find_headphones(api: &HidApi) -> impl Iterator<Item = found_headphone::FoundHeadphone> + '_ {
    let devices = api
        .device_list()
        .filter(|dev| dev.vendor_id() == found_headphone::VENDOR_ID);

    let found_headphones = devices.map(|dev| {
        return found_headphone::build_found_headphone(
            dev.product_string(),
            dev.product_id(),
            dev.path(),
            dev.usage_page(),
            dev.usage(),
            dev.interface_number(),
        );
    });

    return found_headphones;
}
