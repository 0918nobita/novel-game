#[macro_use]
extern crate log;

use anyhow::Context;
use ash::version::{EntryV1_0, InstanceV1_0};
use once_cell::sync::Lazy;
use std::{ffi::CString, intrinsics::transmute};

static APPLICATION_NAME: Lazy<CString> = Lazy::new(|| CString::new("Hello Triangle").unwrap());
static ENGINE_NAME: Lazy<CString> = Lazy::new(|| CString::new("No Engine").unwrap());
static VALIDATION_LAYERS: Lazy<Vec<CString>> = Lazy::new(|| {
    if cfg!(feature = "validation_layers") {
        vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]
    } else {
        vec![]
    }
});

pub struct ManagedInstance {
    raw: ash::Instance,
}

impl ManagedInstance {
    pub fn new(entry: &ash::Entry) -> anyhow::Result<Self> {
        let app_info = ash::vk::ApplicationInfo::builder()
            .application_name(APPLICATION_NAME.as_c_str())
            .application_version(ash::vk::make_version(0, 1, 0))
            .engine_name(ENGINE_NAME.as_c_str())
            .build();
        let enabled_layer_names = (*VALIDATION_LAYERS)
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<_>>();
        let instance_create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&[])
            .enabled_layer_names(&enabled_layer_names)
            .build();
        let raw = unsafe { entry.create_instance(&instance_create_info, None) }
            .context("Failed to create Vulkan instance")?;
        trace!("[CREATED] Vulkan instance");
        Ok(ManagedInstance { raw })
    }

    pub fn get_raw(&self) -> &ash::Instance {
        &self.raw
    }

    pub fn enumerate_physical_device(&self) -> anyhow::Result<Vec<ManagedPhysicalDevice>> {
        unsafe { self.raw.enumerate_physical_devices() }
            .context("Failed to enumerate physical devices")
            .map(|raw_physical_devices| {
                raw_physical_devices
                    .into_iter()
                    .map(|raw_physical_device| {
                        ManagedPhysicalDevice::new(&self, raw_physical_device)
                    })
                    .collect::<Vec<_>>()
            })
    }
}

impl Drop for ManagedInstance {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_instance(None) }
        trace!("[DESTROYED] Vulkan instance")
    }
}

pub struct ManagedPhysicalDevice<'a> {
    instance: &'a ManagedInstance,
    raw: ash::vk::PhysicalDevice,
}

impl<'a> ManagedPhysicalDevice<'a> {
    fn new(instance: &'a ManagedInstance, raw: ash::vk::PhysicalDevice) -> Self {
        ManagedPhysicalDevice { instance, raw }
    }
}

impl std::fmt::Debug for ManagedPhysicalDevice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let props = unsafe {
            self.instance
                .get_raw()
                .get_physical_device_properties(self.raw)
        };
        let device_name: &[u8] =
            unsafe { transmute(std::slice::from_raw_parts(props.device_name.as_ptr(), 256)) };
        let device_name = device_name.to_vec();
        let device_name = String::from_utf8(device_name).unwrap();
        write!(f, "{} ({:?})", device_name, props.device_type)
    }
}
