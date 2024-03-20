use bindings::exports::wasi::virt::transform::{
    Guest,
    // HostEnv,
    Options,
    VirtEnv,
};
use wasi_virt::WasiVirt;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn transform(_component: Vec<u8>, options: Options) -> Result<Vec<u8>, String> {
        let mut virt = WasiVirt::new();
        Self::virt_env(&mut virt, &options.env);
        virt
            .finish()
            .map(|r| r.adapter)
            .map_err(|e| e.to_string())
    }
}

impl Component {
    fn virt_env(virt: &mut WasiVirt, env: &VirtEnv) {
        let overrides = env
            .overrides
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<(&str, &str)>>();

        virt
            .env()
            .overrides(&overrides);
    }
}

bindings::export!(Component with_types_in bindings);
