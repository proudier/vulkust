use vulkano::device::{Device, DeviceExtensions};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError,
};
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() {
    let instance = Instance::new(
        Some(&vulkano::app_info_from_cargo_toml!()),
        &vulkano_win::required_extensions(),
        None,
    )
    .expect("Failed to create Vulkan instance");
    dump_vulkan_info(&instance);
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device compatible with Vulkan are available");
    let (device, mut queues) = {
        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("Couldn't find a graphical queue family for this physical device");
        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("Failed to create Vulkan device")
    };
    let queue = queues.next().expect("Failed to get Vulkan queue");

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .with_title(env!("CARGO_PKG_NAME"))
        .with_resizable(false)
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Failed to create Vulkan surface");

    let (mut swapchain, images) = {
        let caps = surface.capabilities(physical).unwrap();
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let (format, color_space) = pick_format(&caps.supported_formats);
        let dimensions: [u32; 2] = surface.window().inner_size().into();
        let image_count = caps.min_image_count +1;
        // caps.max_image_count.expect

        vulkano::swapchain::Swapchain::new(
            device.clone(),
            surface.clone(),
            image_count,
            format,
            dimensions,
            1,
            ImageUsage::color_attachment(),
            &queue,
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            FullscreenExclusive::Default,
            true,
            color_space,
        )
        .unwrap()
    };

    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::MainEventsCleared => {
            // Application update code.
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        _ => (),
    });
}

fn dump_vulkan_info(instance: &std::sync::Arc<Instance>) {
    println!("Vulkan compatible devices:");
    for phy_dev in PhysicalDevice::enumerate(instance) {
        println!(
            "- index: {}, name: {}, api version: {}, driver version: {}",
            phy_dev.index(),
            phy_dev.name(),
            phy_dev.api_version(),
            phy_dev.driver_version()
        );
        println!("  memory heaps:");
        for mem_heap in phy_dev.memory_heaps() {
            println!(
                "  - id: {}, size (Mb): {}, device local: {}",
                mem_heap.id(),
                mem_heap.size() / (1024 * 1024),
                mem_heap.is_device_local()
            );
        }
        println!("  queue families:");
        for que_fam in phy_dev.queue_families() {
            println!(
                "  - id: {}, queue count: {}, graphical: {}",
                que_fam.id(),
                que_fam.queues_count(),
                que_fam.supports_graphics()
            );
        }
    }
}

fn pick_format(supported_formats: &[(Format, ColorSpace)]) -> (Format, ColorSpace) {
    for v in supported_formats {
        let (fmt, col_spa) = *v;
        if fmt == Format::B8G8R8A8Srgb && col_spa == ColorSpace::SrgbNonLinear {
            println!("Using B8G8R8A8_SRGB format in SrgbNonLinear color space");
            return (fmt, col_spa);
        }
    }
    println!("Using fallback format {}", supported_formats[0].0 as i32);
    supported_formats[0]
}
