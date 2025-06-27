#![no_main]
// use std::{num::NonZeroU64, str::FromStr};
// use wgpu::util::DeviceExt;

// pub fn run_example_compute() {
//   let arguments: Vec<f32> = std::env::args()
//     .skip(1)
//     .map(|s| {
//       f32::from_str(&s).unwrap_or_else(|_| panic!("Cannot parse argument {s:?} as a float."))
//     })
//     .collect();
//   if arguments.is_empty() {
//     println!("No arguments provided.");
//     return;
//   };
//   println!("Parsed {} arguments", arguments.len());
//   env_logger::init();
//   let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
//   let adapter = pollster::block_on(
//     instance.request_adapter(&wgpu::RequestAdapterOptions::default())
//   ).expect("Failed to create an adapter");
//   println!("Running on Adapter: {:#?}", adapter.get_info());
//   let downlevel_capabilities = adapter.get_downlevel_capabilities();
//   if !downlevel_capabilities.flags.contains(wgpu::DownlevelFlags::COMPUTE_SHADERS) {
//     panic!("Adapter does not support compute shaders");
//   }
//   let (device, queue) = pollster::block_on(
//     adapter.request_device(
//       &wgpu::DeviceDescriptor {
//         label: None,
//         required_features: wgpu::Features::empty(),
//         required_limits: wgpu::Limits::downlevel_defaults(),
//         memory_hints: wgpu::MemoryHints::MemoryUsage,
//         trace: wgpu::Trace::Off
//       }
//     )
//   ).expect("Failed to create device.");
//   let module = device.create_shader_module(
//     wgpu::include_wgsl!("example_compute_shader.wgsl")
//   );
//   let input_data_buffer = device.create_buffer_init(
//     &wgpu::util::BufferInitDescriptor {
//       label: None,
//       contents: bytemuck::cast_slice(&arguments),
//       usage: wgpu::BufferUsages::STORAGE
//     }
//   );
//   let output_data_buffer = device.create_buffer(
//     &wgpu::BufferDescriptor {
//       label: None,
//       size: input_data_buffer.size(),
//       usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
//       mapped_at_creation: false
//     }
//   );
//   let download_buffer = device.create_buffer(
//     &wgpu::BufferDescriptor {
//       label: None, 
//       size: input_data_buffer.size(),
//       usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
//       mapped_at_creation: false
//     }
//   );
//   let bind_group_layout = device.create_bind_group_layout(
//     &wgpu::BindGroupLayoutDescriptor {
//       label: None,
//       entries: &[
//         wgpu::BindGroupLayoutEntry {
//           binding: 0,
//           visibility: wgpu::ShaderStages::COMPUTE,
//           ty: wgpu::BindingType::Buffer {
//             ty: wgpu::BufferBindingType::Storage {
//               read_only: true
//             }, 
//             has_dynamic_offset: false, 
//             min_binding_size: Some(NonZeroU64::new(4).unwrap())
//           },
//           count: None
//         },
//         wgpu::BindGroupLayoutEntry {
//           binding: 1,
//           visibility: wgpu::ShaderStages::COMPUTE,
//           ty: wgpu::BindingType::Buffer {
//             ty: wgpu::BufferBindingType::Storage {
//               read_only: false
//             }, 
//             has_dynamic_offset: false, 
//             min_binding_size: Some(NonZeroU64::new(4).unwrap())
//           },
//           count: None
//         },
//       ]
//     }
//   );
//   let bind_group = device.create_bind_group(
//     &wgpu::BindGroupDescriptor {
//       label: None,
//       layout: &bind_group_layout,
//       entries: &[
//         wgpu::BindGroupEntry {
//           binding: 0,
//           resource: input_data_buffer.as_entire_binding()
//         },
//         wgpu::BindGroupEntry {
//           binding: 1,
//           resource: output_data_buffer.as_entire_binding()
//         }
//       ]
//     }
//   );
//   let pipeline_layout = device.create_pipeline_layout(
//     &wgpu::PipelineLayoutDescriptor {
//       label: None,
//       bind_group_layouts: &[&bind_group_layout],
//       push_constant_ranges: &[]
//     }
//   );
//   let pipeline = device.create_compute_pipeline(
//     &wgpu::ComputePipelineDescriptor {
//       label: None,
//       layout: Some(&pipeline_layout),
//       module: &module,
//       entry_point: Some("doubleMe"),
//       compilation_options: wgpu::PipelineCompilationOptions::default(),
//       cache: None
//     }
//   );
//   let mut encoder = device.create_command_encoder(
//     &wgpu::CommandEncoderDescriptor {
//       label: None
//     }
//   );
//   let mut compute_pass = encoder.begin_compute_pass(
//     &wgpu::ComputePassDescriptor {
//       label: None,
//       timestamp_writes: None
//     }
//   );
//   compute_pass.set_pipeline(&pipeline);
//   compute_pass.set_bind_group(0, &bind_group, &[]);
//   let workgroup_count = arguments.len().div_ceil(64);
//   compute_pass.dispatch_workgroups(workgroup_count as u32, 1, 1);
//   drop(compute_pass);
//   encoder.copy_buffer_to_buffer(
//     &output_data_buffer,
//     0, 
//     &download_buffer, 
//     0, 
//     output_data_buffer.size()
//   );
//   let command_buffer = encoder.finish();
//   queue.submit([command_buffer]);
//   let buffer_slice = download_buffer.slice(..);
//   buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
//   device.poll(wgpu::PollType::Wait).unwrap();
//   let data = buffer_slice.get_mapped_range();
//   let result: &[f32] = bytemuck::cast_slice(&data);
//   println!("Result: {:?}", result);
// }