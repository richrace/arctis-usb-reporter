use std::error::Error;

use hidapi::{DeviceInfo, HidApi, HidDevice};

use crate::{
    headphone_helpers::found_headphone::VENDOR_ID,
    known_headphone::{get_known_headphones, KnownHeadphone},
};

fn match_headphone(device: &DeviceInfo) -> Option<&DeviceInfo> {
    let binding = get_known_headphones();
    let known_headphone = binding.get(&device.product_id());

    if device.vendor_id() == VENDOR_ID && known_headphone.is_some() {
        return Some(device);
    }

    return None;
}

fn connectable_headphone(device: &DeviceInfo, matched_headphone: &KnownHeadphone) -> bool {
    if matched_headphone.usage_page != 0 && matched_headphone.usage != 0 {
        return device.usage_page() == matched_headphone.usage_page
            && device.usage() == matched_headphone.usage
            && device.interface_number() == matched_headphone.interface;
    }

    return device.interface_number() == matched_headphone.interface;
}

fn get_report(headphone: &KnownHeadphone, hid_device: HidDevice) -> Result<(), Box<dyn Error>> {
    const PACKET_SIZE: usize = 8;
    const TIMEOUT: i32 = 200;

    let mut buf = [0; PACKET_SIZE];

    hid_device.write(headphone.write_bytes)?;

    let result = hid_device.read_timeout(&mut buf[..], TIMEOUT)?;

    if result == 0 {
        panic!("HSC_TIMEOUT");
    }

    if buf[2] == 0x01 {
        println!("\tBATTERY_UNAVAILABLE");
    } else {
        let bat = buf[headphone.battery_percent_index];
        let charging_status;
        let charging_support;
        let mic_status;
        let mic_support;

        match headphone.charging_status_index {
            Some(i) => {
                charging_status = buf[i];
                charging_support = true;
            }
            None => {
                charging_status = 0;
                charging_support = false;
            }
        }

        match headphone.mic_status_index {
            Some(i) => {
                mic_status = buf[i];
                mic_support = true;
            }
            None => {
                mic_status = 0;
                mic_support = false;
            }
        }

        println!("Headset: {}", headphone.name);
        println!("\tBattery is {}", bat);
        if charging_support {
            let is_charging = charging_status == 1;
            println!("\tis Charging {}", is_charging);
            println!("\tis Discharging {}", !is_charging);
        }
        if mic_support {
            let is_muted = mic_status == 1;
            println!("\tis Muted {}", is_muted);
        }
    }

    Ok(())
}

fn find_headphones(api: &HidApi) {
    let filtered_devices = api.device_list().filter(|device: &&DeviceInfo| {
        let matched_headphone = match_headphone(device);

        if matched_headphone.is_none() {
            return false;
        }

        let binding = get_known_headphones();
        let headphone = &mut binding.get(&device.product_id()).unwrap();

        if !connectable_headphone(device, headphone) {
            return false;
        }

        return true;
    });

    filtered_devices.for_each(|device| {
        let hid_result = device.open_device(api);

        match hid_result {
            Ok(hid_device) => {
                let binding = get_known_headphones();
                let headphone = &mut binding.get(&device.product_id()).unwrap();
                let _ = get_report(headphone, hid_device);
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    })
}

pub fn report_headphones(api: &HidApi) {
    find_headphones(api);
}
