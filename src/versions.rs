pub fn tryorama_version() -> String {
    String::from("^0.11.0")
}

pub fn holochain_client_version() -> String {
    String::from("^0.12.0")
}

pub fn hdi_version() -> String {
    holochain::HDI_VERSION.to_string()
}

pub fn hdk_version() -> String {
    holochain::HDK_VERSION.to_string()
}

pub fn holochain_version() -> String {
    holochain::HOLOCHAIN_VERSION.to_string()
}

pub fn holochain_nix_version() -> String {
    String::from("0_1")
}
