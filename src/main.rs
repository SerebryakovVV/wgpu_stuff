// first of all we create a window
// for the window we need an event loop
// event loop has the run_app function that takes a struct that implements ApplicationHandler trait
// we create an App struct that implements this trait, and pass it to the run_app
// the trait requires two methods, and has some number of optinal methods
// the window_event method on ApplicationHandler trait handles all the events of the window

// now with window created we make an Instance, it takes env variables to configure itself
// with this instance we can create a surface and an adapter
// with adapter we can create device and a queue




mod example_compute;
use example_compute::run_example_compute;








// use std::io::Write;

// #[derive(Default)]
// struct App {
//   window: Option<winit::window::Window>,
// }

// impl winit::application::ApplicationHandler for App {
//   fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
//     if let None = self.window {
//       let win = event_loop
//         .create_window(winit::window::Window::default_attributes())
//         .expect("Error creating window!");
//       self.window = Some(win);
//       println!("resumed from none");
//     };
//     println!("resumed");
//   }

//   fn window_event(
//     &mut self,
//     event_loop: &winit::event_loop::ActiveEventLoop,
//     window_id: winit::window::WindowId,
//     event: winit::event::WindowEvent,
//   ) {
//     match event {
//       winit::event::WindowEvent::CursorEntered { .. } => println!("Cursor entered"),
//       winit::event::WindowEvent::Resized(size) => println!("Resized to {:#?}", size),
//       winit::event::WindowEvent::CloseRequested => {
//         println!("Close requested");
//         event_loop.exit()
//       }
//       _ => {
//         // println!("Rest of the events");
//       }
//     }
//   }
//   // those are required methods, this trait also has some optional ones
// }


// async fn setup(window: &winit::window::Window) {
// 		let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
// 		// i need to create the window first
// 		// add the new() method to the app
// 		let surface = instance.create_surface(window).unwrap();
// 		let adapter = instance.request_adapter(
// 			&wgpu::RequestAdapterOptions {
// 				compatible_surface: Some(&surface),
// 				..Default::default()
// 			}
// 		).await.unwrap();
// 		let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor{..Default::default()}).await.unwrap();

		

// 		// Its primary use is to create Adapters and Surfaces.

// }


fn main() {



  run_example_compute();
  return;



  // let event_loop = winit::event_loop::EventLoop::new().unwrap();
  // let mut app = App::default();
	// if let Some(w) = app.window.as_ref() {
	// 	setup(w);
	// }
  // event_loop.run_app(&mut app);
}
