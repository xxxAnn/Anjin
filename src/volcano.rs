use vulkano::{
    instance::{Instance, InstanceCreateInfo}, 
    buffer::{BufferUsage, CpuAccessibleBuffer}, 
    device::physical::{PhysicalDevice}, 
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage},
    device::{Device, Features, DeviceCreateInfo, QueueCreateInfo}
};

pub fn initiate_volcano() {
    let instance = Instance::new(InstanceCreateInfo::default()).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

    for family in physical.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        }
    ).expect("failed to create a device");

    let queue = queues.next().unwrap();

    let source_content: Vec<i32> = (0..64).collect();
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
        .expect("failed to create buffer");

    let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, destination_content)
        .expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    
    builder.copy_buffer(source.clone(), destination.clone()).unwrap();
    
    let command_buffer = builder.build().unwrap();
    
}