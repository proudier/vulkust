extern crate glfw;

use glfw::{Action, Context, Key, WindowHint};
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW library.");

    let extensions: vulkano::instance::RawInstanceExtensions =
        vulkano_glfw_v2::get_required_raw_instance_extensions(&glfw)
            .expect("Failed to retrieve Vulkan instance extensions required by GLFW");
    let instance = Instance::new(
        Some(&vulkano::app_info_from_cargo_toml!()),
        extensions,
        None,
    )
    .expect("Failed to create Vulkan instance");

    println!("Dumping Vulkan compatible devices:");
    for phy_dev in PhysicalDevice::enumerate(&instance) {
        println!(
            "  - index: {}, name: {}, api version: {}, driver version: {}",
            phy_dev.index(),
            phy_dev.name(),
            phy_dev.api_version(),
            phy_dev.driver_version()
        );
    }
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device compatible with Vulkan are available");

    println!("Dumping memory heaps:");
    for memory_heap in physical.memory_heaps() {
        println!(
            "  - id: {}, size (Mb): {}, device local: {}",
            memory_heap.id(),
            memory_heap.size() / (1024 * 1024),
            memory_heap.is_device_local()
        );
    }

    println!("Dumping queue families:");
    for que_fam in physical.queue_families() {
        println!(
            "  - id: {}, queue count: {}, graphical: {}",
            que_fam.id(),
            que_fam.queues_count(),
            que_fam.supports_graphics()
        );
    }
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();

    glfw.window_hint(WindowHint::Resizable(false));
    glfw.window_hint(WindowHint::DoubleBuffer(true));
    glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    let (mut window, events) = glfw
        .create_window(1920, 1080, "Vulkust", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
        window.set_key_polling(true);

    let surface = vulkano_glfw_v2::create_window_surface(instance, window).unwrap();

    // window.make_current();

    while !surface.window().should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
