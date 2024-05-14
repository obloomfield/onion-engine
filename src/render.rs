use wgpu::util::DeviceExt;
use winit::window::Window;
use super::instance::Instance;

use super::graphics::gl::Vertex as Vertex;
use super::graphics::gl::BufferContents as BufferContents;
use ultraviolet as uv;
use super::camera::{Camera, CameraUniform};
use super::graphics::texture::Texture;

use super::game_interface::app::App;

use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  keyboard::KeyCode,
  keyboard::PhysicalKey::Code,
  window::WindowBuilder,
};

pub struct State<'window> {
  surface: wgpu::Surface<'window>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  window: &'window Window,
  render_pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,
  num_vertices: u32,
  index_buffer: wgpu::Buffer,
  num_indices: u32,
  diffuse_bind_group: wgpu::BindGroup,
  diffuse_texture: Texture,
  pub camera: Camera,
  camera_uniform: CameraUniform,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
  instances: Vec<Instance>,
  instance_buffer: wgpu::Buffer,
  depth_texture: Texture,
}

pub async fn run(mut app: Box<dyn App>) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    let _ = event_loop.run(|event, elwt| {
        match event {
            Event::AboutToWait { .. } => {
                state.window().request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match &event {
                    WindowEvent::KeyboardInput { event, .. } => {
                        state.input(&mut app, event);
                        if event.state.is_pressed() {
                            match event.physical_key {
                                Code(KeyCode::Escape) => {
                                    elwt.exit();
                                }
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(&mut app, *physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { .. } => state.resize(&mut app, window.inner_size()),
                    WindowEvent::RedrawRequested => {
                        state.update(&mut app);
                        match state.render() {
                            Ok(_) => {}
                            // Recreate the swap_chain if lost
                            Err(wgpu::SurfaceError::Lost) => state.resize(&mut app, state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("Some unhandled error {:?}", e),
                        }
                    }
                    _ => {}
                }
            }
            _ => (),
        }
    });
}

impl<'window> State<'window> {
  // Creating some of the wgpu types requires async code
  async fn new(window: &'window Window) -> Self {

      let size = window.inner_size();

      let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
          backends: wgpu::Backends::all(),
          ..Default::default()
      });

      let surface: wgpu::Surface<'window> = instance.create_surface(window).unwrap();

      let adapter = instance
          .request_adapter(&wgpu::RequestAdapterOptions {
              power_preference: wgpu::PowerPreference::default(),
              compatible_surface: Some(&surface),
              force_fallback_adapter: false,
          })
          .await
          .unwrap();

      let (device, queue) = adapter
          .request_device(
              &wgpu::DeviceDescriptor {
                  required_features: wgpu::Features::empty(),
                  required_limits: wgpu::Limits::default(),
                  label: None,
              },
              None,
          )
          .await
          .unwrap();

      let surface_caps = surface.get_capabilities(&adapter);
      // Shader code in this tutorial assumes an sRGB surface texture. Using a different
      // one will result in all the colors coming out darker. If you want to support non
      // sRGB surfaces, you'll need to account for that when drawing to the frame.
      let surface_format = surface_caps
          .formats
          .iter()
          .copied()
          .filter(|f| f.is_srgb())
          .next()
          .unwrap_or(surface_caps.formats[0]);
      let config = wgpu::SurfaceConfiguration {
          usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
          format: surface_format,
          width: size.width,
          height: size.height,
          present_mode: surface_caps.present_modes[0],
          desired_maximum_frame_latency: 1,
          alpha_mode: surface_caps.alpha_modes[0],
          view_formats: vec![],
      };

      surface.configure(&device, &config);

      let diffuse_bytes = include_bytes!("images/happy-tree.png");
      let diffuse_texture =
          Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap(); // CHANGED!
      let texture_bind_group_layout =
          device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
              entries: &[
                  wgpu::BindGroupLayoutEntry {
                      binding: 0,
                      visibility: wgpu::ShaderStages::FRAGMENT,
                      ty: wgpu::BindingType::Texture {
                          multisampled: false,
                          view_dimension: wgpu::TextureViewDimension::D2,
                          sample_type: wgpu::TextureSampleType::Float { filterable: true },
                      },
                      count: None,
                  },
                  wgpu::BindGroupLayoutEntry {
                      binding: 1,
                      visibility: wgpu::ShaderStages::FRAGMENT,
                      // This should match the filterable field of the
                      // corresponding Texture entry above.
                      ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                      count: None,
                  },
              ],
              label: Some("texture_bind_group_layout"),
          });

      let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
          layout: &texture_bind_group_layout,
          entries: &[
              wgpu::BindGroupEntry {
                  binding: 0,
                  resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
              },
              wgpu::BindGroupEntry {
                  binding: 1,
                  resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
              },
          ],
          label: Some("diffuse_bind_group"),
      });

      let camera = Camera {
          // position the camera 1 unit up and 2 units back
          // +z is out of the screen
          eye: (0.0, 1.0, 2.0).into(),
          // have it look at the origin
          target: (0.0, 0.0, 0.0).into(),
          // which way is "up"
          up: uv::Vec3::unit_y(),
          aspect: config.width as f32 / config.height as f32,
          fov: 45.0,
          near: 0.1,
          far: 100.0,
      };

      let mut camera_uniform = CameraUniform::new();
      camera_uniform.update_view_proj(&camera);

      let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Camera Buffer"),
          contents: bytemuck::cast_slice(&[camera_uniform]),
          usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      });

      let camera_bind_group_layout =
          device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
              entries: &[wgpu::BindGroupLayoutEntry {
                  binding: 0,
                  visibility: wgpu::ShaderStages::VERTEX,
                  ty: wgpu::BindingType::Buffer {
                      ty: wgpu::BufferBindingType::Uniform,
                      has_dynamic_offset: false,
                      min_binding_size: None,
                  },
                  count: None,
              }],
              label: Some("camera_bind_group_layout"),
          });

      let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
          layout: &camera_bind_group_layout,
          entries: &[wgpu::BindGroupEntry {
              binding: 0,
              resource: camera_buffer.as_entire_binding(),
          }],
          label: Some("camera_bind_group"),
      });

      let instances = (0..NUM_INSTANCES_PER_ROW)
        .flat_map(|z| {
          (0..NUM_INSTANCES_PER_ROW).map(move |x| {
            let pos = uv::Vec3::new(x as f32, 0.0, z as f32) - INSTANCE_DISPLACEMENT;
            let rot = if pos.x != 0.0 || pos.z != 0.0 {
              uv::Rotor3::from_angle_plane(
                  std::f32::consts::PI / 4.0,
                  uv::Bivec3::from_normalized_axis(pos.normalized()), 
              )
            } else {
              uv::Rotor3::identity()
            };

            Instance {
              position: pos,
              rotation: rot,
              scale: 1.0,
            }
          })
        }).collect::<Vec<_>>();

      let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

      let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
      let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Instance Buffer"),
          contents: bytemuck::cast_slice(&instance_data),
          usage: wgpu::BufferUsages::VERTEX,
      });

      let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));

      let render_pipeline_layout =
          device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
              label: Some("Render Pipeline Layout"),
              bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
              push_constant_ranges: &[],
          });
      let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
          label: Some("Render Pipeline"),
          layout: Some(&render_pipeline_layout),
          vertex: wgpu::VertexState {
              module: &shader,
              entry_point: "vs_main",
              buffers: &[Vertex::desc()],
          },
          fragment: Some(wgpu::FragmentState {
              // 3.
              module: &shader,
              entry_point: "fs_main",
              targets: &[Some(wgpu::ColorTargetState {
                  // 4.
                  format: config.format,
                  blend: Some(wgpu::BlendState::REPLACE),
                  write_mask: wgpu::ColorWrites::ALL,
              })],
          }),
          primitive: wgpu::PrimitiveState {
              topology: wgpu::PrimitiveTopology::TriangleList,
              strip_index_format: None,
              front_face: wgpu::FrontFace::Ccw,
              cull_mode: Some(wgpu::Face::Back),
              // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
              polygon_mode: wgpu::PolygonMode::Fill,
              // Requires Features::DEPTH_CLIP_CONTROL
              unclipped_depth: false,
              // Requires Features::CONSERVATIVE_RASTERIZATION
              conservative: false,
          },
          depth_stencil: None, // 1.
          multisample: wgpu::MultisampleState {
              count: 1,
              mask: !0,
              alpha_to_coverage_enabled: false,
          },
          multiview: None, // 5.
      });

      let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Vertex Buffer"),
          contents: bytemuck::cast_slice(EXAMPLE_BUFFER.vertices),
          usage: wgpu::BufferUsages::VERTEX,
      });

      let num_vertices = EXAMPLE_BUFFER.vertices.len() as u32;

      let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Index Buffer"),
          contents: bytemuck::cast_slice(EXAMPLE_BUFFER.indices),
          usage: wgpu::BufferUsages::INDEX,
      });

      let num_indices = EXAMPLE_BUFFER.num_indices;

      

      Self {
          surface,
          device,
          queue,
          config,
          size,
          window: &window,
          render_pipeline,
          vertex_buffer,
          num_vertices,
          index_buffer,
          num_indices,
          diffuse_bind_group,
          diffuse_texture,
          camera,
          camera_uniform,
          camera_buffer,
          camera_bind_group,
          instances,
          instance_buffer,
          depth_texture
      }
  }

  pub fn window(&self) -> &Window {
      &self.window
  }

  fn resize(&mut self, app: &mut Box<dyn App>, new_size: winit::dpi::PhysicalSize<u32>) {
    app.resize(self, new_size);
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
      self.window().request_redraw();
    }
  }

  fn input(&mut self, app: &mut Box<dyn App>, event: &KeyEvent) {
    app.input(self, event);
  }

  fn update(&mut self, app: &mut Box<dyn App>) {
      app.update(self);
      self.camera_uniform.update_view_proj(&self.camera);
      self.queue.write_buffer(
          &self.camera_buffer,
          0,
          bytemuck::cast_slice(&[self.camera_uniform]),
      );
  }

  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
      let output = self.surface.get_current_texture()?;
      let view = output
          .texture
          .create_view(&wgpu::TextureViewDescriptor::default());
      let mut encoder = self
          .device
          .create_command_encoder(&wgpu::CommandEncoderDescriptor {
              label: Some("Render Encoder"),
          });
      {
          let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
              label: Some("Render Pass"),
              color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                  view: &view,
                  resolve_target: None,
                  ops: wgpu::Operations {
                      load: wgpu::LoadOp::Clear(wgpu::Color {
                          r: 0.1,
                          g: 0.2,
                          b: 0.3,
                          a: 1.0,
                      }),
                      store: wgpu::StoreOp::Store,
                  },
              })],
              depth_stencil_attachment: None,
              occlusion_query_set: None,
              timestamp_writes: None,
          });

          render_pass.set_pipeline(&self.render_pipeline);
          render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
          render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

          render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
          render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
          render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
      }

      // submit will accept anything that implements IntoIter
      self.queue.submit(std::iter::once(encoder.finish()));
      output.present();

      Ok(())
  }
}

const NUM_INSTANCES_PER_ROW: u32 = 10;
// const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(NUM_INSTANCES_PER_ROW as f32 * 0.5, 0.0, NUM_INSTANCES_PER_ROW as f32 * 0.5);
const INSTANCE_DISPLACEMENT: uv::Vec3 = uv::Vec3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);
const EXAMPLE_BUFFER : BufferContents = BufferContents {
vertices: &[
  // Changed
  Vertex {
      position: [-0.0868241, 0.49240386, 0.0],
      tex_coords: [0.4131759, 0.00759614],
  }, // A
  Vertex {
      position: [-0.49513406, 0.06958647, 0.0],
      tex_coords: [0.0048659444, 0.43041354],
  }, // B
  Vertex {
      position: [-0.21918549, -0.44939706, 0.0],
      tex_coords: [0.28081453, 0.949397],
  }, // C
  Vertex {
      position: [0.35966998, -0.3473291, 0.0],
      tex_coords: [0.85967, 0.84732914],
  }, // D
  Vertex {
      position: [0.44147372, 0.2347359, 0.0],
      tex_coords: [0.9414737, 0.2652641],
  }, // E
],
indices: &[0, 1, 4, 1, 2, 4, 2, 3, 4],
num_indices: 9,
};

impl Vertex {
  fn desc() -> wgpu::VertexBufferLayout<'static> {
      use std::mem;
      wgpu::VertexBufferLayout {
          array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
          step_mode: wgpu::VertexStepMode::Vertex,
          attributes: &[
              wgpu::VertexAttribute {
                  offset: 0,
                  shader_location: 0,
                  format: wgpu::VertexFormat::Float32x3,
              },
              wgpu::VertexAttribute {
                  offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                  shader_location: 1,
                  format: wgpu::VertexFormat::Float32x2, // NEW!
              },
          ],
      }
  }
}
