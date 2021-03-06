use crate::{
    command_pool::ManagedCommandPool, framebuffer::ManagedFramebuffer,
    linear_image::ManagedAndLinearImage, optimized_image::ManagedAndOptimizedImage,
    render_pass::ManagedRenderPass,
};
use ash::{
    version::DeviceV1_0,
    vk::{PhysicalDevice, Queue},
    Device, Instance,
};

pub struct ManagedLogicalDevice<'a> {
    instance: &'a Instance,
    physical_device: PhysicalDevice,
    device_raw: Device,
    queue_indices: Vec<u32>,
}

impl<'a> ManagedLogicalDevice<'a> {
    pub fn new(
        instance: &'a Instance,
        physical_device: PhysicalDevice,
        device_raw: Device,
        queue_indices: Vec<u32>,
    ) -> ManagedLogicalDevice<'a> {
        // 三角形を画像を描画するのが直近の目標なので、グラフィックスキューだけ利用して表示キューは放置
        ManagedLogicalDevice {
            instance,
            physical_device,
            device_raw,
            queue_indices,
        }
    }

    pub fn get_graphics_queue(&self) -> Queue {
        let graphics_queue_family_index = self.queue_indices[0];
        unsafe {
            self.device_raw
                .get_device_queue(graphics_queue_family_index, 0)
        }
    }

    pub fn create_command_pool(&self) -> anyhow::Result<ManagedCommandPool> {
        let graphics_queue_family_index = self.queue_indices[0];
        ManagedCommandPool::new(&self.device_raw, graphics_queue_family_index)
    }

    pub fn create_optimized_image(
        &self,
        width: u32,
        height: u32,
    ) -> anyhow::Result<ManagedAndOptimizedImage> {
        ManagedAndOptimizedImage::new(
            self.instance,
            &self.physical_device,
            &self.device_raw,
            width,
            height,
        )
    }

    pub fn create_linear_image(
        &self,
        width: u32,
        height: u32,
    ) -> anyhow::Result<ManagedAndLinearImage> {
        ManagedAndLinearImage::new(
            self.instance,
            &self.physical_device,
            &self.device_raw,
            width,
            height,
        )
    }

    pub fn create_render_pass(&self) -> anyhow::Result<ManagedRenderPass> {
        ManagedRenderPass::new(&self.device_raw)
    }

    pub fn create_framebuffer(
        &'a self,
        render_pass: &'a ManagedRenderPass,
        connectable_image: &'a ManagedAndOptimizedImage,
        width: u32,
        height: u32,
    ) -> anyhow::Result<ManagedFramebuffer<'a>> {
        ManagedFramebuffer::new(
            &self.device_raw,
            render_pass,
            connectable_image,
            width,
            height,
        )
    }
}

impl Drop for ManagedLogicalDevice<'_> {
    fn drop(&mut self) {
        unsafe { self.device_raw.destroy_device(None) };
        trace!("Logical device was destroyed")
    }
}
