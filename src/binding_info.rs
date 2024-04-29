#[derive(Clone)]
pub struct BindingInfo {
    pub binding: u32,
    pub buffer_type: Box<str>,
    pub read_only: bool
}

// Hard-coded before eventual regex extraction
pub fn generate_binding_info_from_wgsl(wgsl_code: &String) -> Vec<BindingInfo> {
    return [
        BindingInfo {binding: 0, buffer_type: "storage".into(), read_only: false},
        BindingInfo {binding: 1, buffer_type: "storage".into(), read_only: false},
        BindingInfo {binding: 2, buffer_type: "storage".into(), read_only: false},
        BindingInfo {binding: 3, buffer_type: "uniform".into(), read_only: true},
        BindingInfo {binding: 4, buffer_type: "unfirom".into(), read_only: true}
    ].to_vec();
}