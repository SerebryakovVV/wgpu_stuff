mod model;
mod cube;
use model::{Model, Vertex};
use cube::make_cube;
use cgmath::{self, Deg, Matrix4};
use cgmath::prelude::*;
use winit::keyboard::KeyCode;

use std::{mem, sync::Arc};
use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, Adapter, BindGroup, Buffer, Device, Instance, Queue, RenderPipeline, ShaderModule, Surface, TextureFormat};
use winit::{
  application::ApplicationHandler, dpi::{PhysicalPosition, PhysicalSize}, event::WindowEvent, event_loop::{
    self,
    ActiveEventLoop,
    EventLoop
  }, window::{
    Window, 
    WindowAttributes,
    WindowId
  }
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
  mvp: [[f32; 4]; 4]
}

struct View {
  uni: Uniforms,
  
  projection: Matrix4<f32>,
  fovy: Deg<f32>,
  aspect: f32, 
  near: f32, 
  far: f32,

  view: Matrix4<f32>,
  
  
  // these two for camera control
  eye: cgmath::Point3<f32>,
  eye_x: f32,
  eye_y: f32,
  eye_z: f32,
  center: cgmath::Point3<f32>,
  center_x: f32,
  center_y: f32,
  center_z: f32,
  
  
  
  up: cgmath::Vector3<f32>,

  model_matrix: Matrix4<f32>,
  rotation_x: Matrix4<f32>,
  rotation_y: Matrix4<f32>,
  rotation_z: Matrix4<f32>
}


impl View {
  fn compute_uniforms(&mut self) -> Uniforms {
    // let fovy = cgmath::Deg(75.0);
    // let aspect = size.width as f32 / size.height as f32;
    // let near = 0.1;
    // let far = 100.0;
    // let projection = cgmath::perspective(fovy, aspect, near, far);
    
    
    
    // let eye_x = 0.0;
    // let eye_y = 0.0;
    // let eye_z = 3.0;
    // let eye = cgmath::Point3::new(eye_x, eye_y, eye_z);


    // let center_x = 0.0;
    // let center_y = 0.0;
    // let center_z = 0.0;
    // let center = cgmath::Point3::new(center_x, center_y, center_z);



    // let up = cgmath::Vector3::unit_y();
    // let view = cgmath::Matrix4::look_at_rh(eye, center, up);
        
    // let rotation_x = Matrix4::from_angle_x(Deg(60.0)); 
    // let rotation_y = Matrix4::from_angle_y(Deg(60.0));
    // let rotation_z = Matrix4::from_angle_z(Deg(60.0));
    // let model_matrix = rotation_z * rotation_y * rotation_x;

    // let mvp_matrix = Uniforms {
    //   mvp: (projection * view * model_matrix).into()
    // };

    self.eye = cgmath::Point3::new(self.eye_x, self.eye_y, self.eye_z);
    self.center = cgmath::Point3::new(self.center_x, self.center_y, self.center_z);
    self.view = cgmath::Matrix4::look_at_rh(self.eye, self.center, self.up);
    self.uni.mvp = (self.projection * self.view * self.model_matrix).into();
    self.uni
  }
}

struct App {
  window: Option<Arc<Window>>,
  gfx_state: Option<GfxState>,
  bodies: Vec<Model>,
  mouse_pos: PhysicalPosition<f64>,
}

impl App {
  fn new() -> Self {
    Self {
      window: None,
      gfx_state: None,
      bodies: vec![make_cube()],
      mouse_pos: PhysicalPosition { x: 0.0, y: 0.0 }
    }
  }

  fn render(&mut self) {
    let gfx_state = self.gfx_state.as_mut().unwrap();
    let surface_texture = gfx_state.surface.get_current_texture().unwrap();
    let texture_view = surface_texture.texture.create_view(&wgpu::wgt::TextureViewDescriptor {format: Some(gfx_state.surface_fmt) ,..Default::default()});    
    let mut encoder = gfx_state.device.create_command_encoder(&Default::default());
    let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None, 
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &texture_view,
        resolve_target: None,
        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), store: wgpu::StoreOp::Store }
      })], 
      depth_stencil_attachment: None, 
      timestamp_writes: None, 
      occlusion_query_set: None 
    });
    renderpass.set_pipeline(&gfx_state.render_pipeline);
    renderpass.set_vertex_buffer(0, gfx_state.vertex_buffer.slice(..));
    renderpass.set_index_buffer(gfx_state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    renderpass.set_bind_group(0, &gfx_state.uniform_bind_group, &[]);
    renderpass.draw_indexed(0..gfx_state.index_buffer.size() as u32 / std::mem::size_of::<u16>() as u32, 0, 0..1);
    drop(renderpass);
    gfx_state.queue.submit([encoder.finish()]);
    self.window.as_ref().unwrap().pre_present_notify();
    surface_texture.present();
  }

  fn configure_surface(&mut self) {
    let gfx_state = self.gfx_state.as_mut().unwrap();
    gfx_state.size = self.window.as_mut().unwrap().inner_size();
    let config = wgpu::SurfaceConfiguration {
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      desired_maximum_frame_latency: 2,
      format: gfx_state.surface_fmt,
      height: gfx_state.size.height,
      width: gfx_state.size.width,
      present_mode: wgpu::PresentMode::AutoVsync,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      view_formats: vec![gfx_state.surface_fmt.add_srgb_suffix()]
    };
    gfx_state.surface.configure(&gfx_state.device, &config);
  }
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.window = Some(
      Arc::new(
        event_loop.create_window(
          WindowAttributes::default()
        ).unwrap()
      )
    );
    self.gfx_state = Some(
      pollster::block_on(
        GfxState::setup(
          self
          .window
          .as_ref()
          .unwrap()
          .clone()
          ,
          &self.bodies
        )
      )
    );
    self.configure_surface();
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CursorMoved { position, .. } => {


        //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // let gfx_state = self.gfx_state.as_mut().unwrap();

        // let projection = cgmath::perspective(
        //   cgmath::Deg(75.0), 
        //   gfx_state.size.width as f32 / gfx_state.size.height as f32, 
        //   0.1, 
        //   100.0
        // );
        // let view = cgmath::Matrix4::look_at_rh(
        //   cgmath::Point3::new(0.0, 0.0, 3.0),
        //   cgmath::Point3::new(0.0, 0.0, 0.0), 
        //   cgmath::Vector3::unit_y()
        // );
        // // let model_matrix = Matrix4::<f32>::from_angle_y(Deg(65.0));
        
        // let rotation_x = Matrix4::from_angle_x(Deg((position.x / 5.0) as f32)); 
        // let rotation_y = Matrix4::from_angle_y(Deg((position.x / 5.0) as f32));
        // let rotation_z = Matrix4::from_angle_z(Deg((position.x / 5.0) as f32));
        // let model_matrix = rotation_z * rotation_y * rotation_x;

        // let mvp_matrix = Uniforms {
        //   mvp: (projection * view * model_matrix).into()
        // };
        // gfx_state.queue.write_buffer(&gfx_state.uniform_buffer, 0, bytemuck::cast_slice(&[mvp_matrix]));
        // self.window.as_ref().unwrap().request_redraw();
        //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
      },
      WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
        match event.physical_key {
          winit::keyboard::PhysicalKey::Code(k) => {
            match k {
              KeyCode::KeyW => {
                let gfx_state = self.gfx_state.as_mut().unwrap();
                gfx_state.view.eye_z -= 0.1;
                gfx_state.view.center_z -= 0.1;
                let mvp_matrix = gfx_state.view.compute_uniforms();
                gfx_state.queue.write_buffer(&gfx_state.uniform_buffer, 0, bytemuck::cast_slice(&[mvp_matrix]));
                self.window.as_ref().unwrap().request_redraw();
              },
              KeyCode::KeyS => {
                let gfx_state = self.gfx_state.as_mut().unwrap();
                gfx_state.view.eye_z += 0.1;
                gfx_state.view.center_z += 0.1;
                let mvp_matrix = gfx_state.view.compute_uniforms();
                gfx_state.queue.write_buffer(&gfx_state.uniform_buffer, 0, bytemuck::cast_slice(&[mvp_matrix]));
                self.window.as_ref().unwrap().request_redraw();
              },
              KeyCode::KeyA => {
                let gfx_state = self.gfx_state.as_mut().unwrap();
                gfx_state.view.eye_x -= 0.1;
                gfx_state.view.center_x -= 0.1;
                let mvp_matrix = gfx_state.view.compute_uniforms();
                gfx_state.queue.write_buffer(&gfx_state.uniform_buffer, 0, bytemuck::cast_slice(&[mvp_matrix]));
                self.window.as_ref().unwrap().request_redraw();
              },
              KeyCode::KeyD => {
                let gfx_state = self.gfx_state.as_mut().unwrap();
                gfx_state.view.eye_x += 0.1;
                gfx_state.view.center_x += 0.1;
                let mvp_matrix = gfx_state.view.compute_uniforms();
                gfx_state.queue.write_buffer(&gfx_state.uniform_buffer, 0, bytemuck::cast_slice(&[mvp_matrix]));
                self.window.as_ref().unwrap().request_redraw();
              },
              _ => {}
            }
          },
          _ => {}
        }
      }
      WindowEvent::Resized(size) => {
        if size.height > 0 && size.width > 0 {
          self.configure_surface();
        }
      },
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::RedrawRequested => self.render(),
      _ => ()
    }
  }
}


struct GfxState {
  instance: Instance,
  surface: Surface<'static>,
  adapter: Adapter,
  device: Device,
  queue: Queue,
  surface_fmt: TextureFormat,
  size: PhysicalSize<u32>,
  vertex_buffer: Buffer,
  index_buffer: Buffer,
  shader: ShaderModule,
  render_pipeline: RenderPipeline,
  uniform_buffer: Buffer,
  uniform_bind_group: BindGroup,
  view: View
}

impl GfxState {
  async fn setup(window: Arc<Window>, bodies: &Vec<Model>) -> Self {
		let instance = Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
		let surface = instance.create_surface(window.clone()).unwrap();
    let size = window.inner_size();
		let adapter = instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				compatible_surface: Some(&surface),
				..Default::default()
			}
		).await.unwrap();
		let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor{
        required_features: wgpu::Features::POLYGON_MODE_LINE,
        ..Default::default()
      }
    ).await.unwrap();
    let surface_fmt = surface.get_capabilities(&adapter).formats[0];
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex buffer"),
      usage: wgpu::BufferUsages::VERTEX,
      contents: bytemuck::cast_slice(&bodies[0].vertices)
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Index buffer"),
      usage: wgpu::BufferUsages::INDEX,
      contents: bytemuck::cast_slice(&bodies[0].indices)
    });
    
    
    let fovy = cgmath::Deg(75.0);
    let aspect = size.width as f32 / size.height as f32;
    let near = 0.1;
    let far = 100.0;
    let projection = cgmath::perspective(fovy, aspect, near, far);
    
    
    
    let eye_x = 0.0;
    let eye_y = 0.0;
    let eye_z = 3.0;
    let eye = cgmath::Point3::new(eye_x, eye_y, eye_z);


    let center_x = 0.0;
    let center_y = 0.0;
    let center_z = 0.0;
    let center = cgmath::Point3::new(center_x, center_y, center_z);



    let up = cgmath::Vector3::unit_y();
    let view = cgmath::Matrix4::look_at_rh(eye, center, up);
        
    let rotation_x = Matrix4::from_angle_x(Deg(60.0)); 
    let rotation_y = Matrix4::from_angle_y(Deg(60.0));
    let rotation_z = Matrix4::from_angle_z(Deg(60.0));
    let model_matrix = rotation_z * rotation_y * rotation_x;

    let mvp_matrix = Uniforms {
      mvp: (projection * view * model_matrix).into()
    };

    let mut view_main = View {
      uni: mvp_matrix,
      fovy,
      aspect,
      near,
      far,
      projection,
      eye_x,
      eye_y,
      eye_z,
      eye,
      center_x,
      center_y,
      center_z,
      center,
      up,
      view,
      rotation_x,
      rotation_y,
      rotation_z,
      model_matrix
    };



    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("matrix unifrom buffer"),
      contents: bytemuck::cast_slice(&[mvp_matrix]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });
    let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("uniform buffer layout"),
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
          count: None,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform, 
            has_dynamic_offset: false, 
            min_binding_size: None 
          }
        }
      ]
    });
    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("uniform bind group"),
      layout: &uniform_bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: uniform_buffer.as_entire_binding()
      }]
    });


    let shader = device.create_shader_module(wgpu::include_wgsl!("3d_model_render_shader.wgsl"));
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
      label: Some("pipeline layout label"), 
      bind_group_layouts: &[&uniform_bind_group_layout], 
      push_constant_ranges: &[] 
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
      label: Some("Render pipeline"), 
      layout: Some(&pipeline_layout), 
      vertex: wgpu::VertexState { 
        module: &shader, 
        entry_point: Some("vs_main"), 
        compilation_options: Default::default(), 
        buffers: &[
          wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
              wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3
              },
              wgpu::VertexAttribute {
                offset: 12,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
              },
            ]
          }
        ]
      }, 
      primitive: wgpu::PrimitiveState { 
        topology: wgpu::PrimitiveTopology::TriangleList, 
        // topology: wgpu::PrimitiveTopology::LineList, 
        front_face: wgpu::FrontFace::Ccw, 
        // cull_mode: Some(wgpu::Face::Front), 
        cull_mode: Some(wgpu::Face::Back), 
        // cull_mode: None, 
        polygon_mode: wgpu::PolygonMode::Fill,
        ..Default::default()
      }, 
      depth_stencil: None, 
      multisample: wgpu::MultisampleState::default(), 
      fragment: Some(wgpu::FragmentState { 
        module: &shader, 
        entry_point: Some("fs_main"), 
        compilation_options: Default::default(), 
        targets: &[Some(wgpu::ColorTargetState { 
          format: surface_fmt, 
          blend: Some(wgpu::BlendState::REPLACE), 
          write_mask: wgpu::ColorWrites::ALL 
        })]
      }), 
      multiview: None, 
      cache: None
    });
    Self {
      instance,
      surface,
      size,
      adapter,
      device,
      queue,
      surface_fmt,
      vertex_buffer,
      index_buffer,
      shader,
      render_pipeline,
      uniform_buffer,
      uniform_bind_group,
      view: view_main
    }
  }
}


fn main() {
  env_logger::init();
  let mut app = App::new();
  let event_loop = EventLoop::new().unwrap();
  event_loop.run_app(&mut app);
}