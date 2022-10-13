use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use haphazard::AtomicPtr;
use iced_native::{renderer, Color, Debug, Size};
use iced_wgpu::{wgpu, Backend, Renderer, Settings, Viewport};
use iced_winit::winit::dpi::{PhysicalPosition, PhysicalSize};
use iced_winit::winit::event::{Event, WindowEvent};
use iced_winit::winit::event_loop::ControlFlow;
use iced_winit::winit::{
    self, event::ModifiersState, event_loop::EventLoop, monitor::MonitorHandle,
};
use iced_winit::{conversion, Clipboard, Error};
mod home;
use haphazard::HazardPointer;
use home::{Home, Message, PublicScreenState};

use tokio::runtime;
use tokio::sync::mpsc;

async fn start() {
    let event_loop = EventLoop::new();
    let mut monitors: Vec<MonitorHandle> = vec![];
    event_loop.available_monitors().for_each(|handle| {
        println!("Monitor of : {}", &handle.name().unwrap());
        monitors.push(handle);
    });
    // let window = winit::window::Window::new(&event_loop).unwrap();
    let current_size = PhysicalSize {
        height: 500,
        width: 900,
    };
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(current_size)
        // .with_fullscreen(Some(Fullscreen::Borderless(Some(
        //     monitors.get(0).unwrap().clone(),
        // ))))
        .with_title(String::from("Wgpu Render"))
        .build(&event_loop)
        .unwrap();
    let physical_size = window.inner_size();
    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
    );
    // let cursor_position = Arc::new(AtomicPtr::from(Box::new(PhysicalPosition::new(-1.0, -1.0))));
    // let mouse_state = Arc::new(AtomicPtr::from(Box::new(MOUSEEVENTF_MOVE)));
    // let cursor_position_clone = Arc::clone(&cursor_position);
    // let mouse_state_clone = Arc::clone(&mouse_state);
    let mut modifiers = ModifiersState::default();
    let mut clipboard = Clipboard::connect(&window);
    let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER,
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");
    // Create the logical device and command queue
    let swapchain_format = surface
        .get_supported_formats(&adapter)
        .first()
        .copied()
        .expect("Get preferred format");
    // Load the shaders from disk
    let mut resized = false;

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: current_size.width,
        height: current_size.height,
        // width: size1.width,
        // height: size1.height,
        present_mode: wgpu::PresentMode::Immediate,
    };
    surface.configure(&device, &config);
    let mut staging_belt = wgpu::util::StagingBelt::new(5 * 1024);

    // Initialize scene and GUI controls

    let mut home = Home::new();
    // let wgpusurface = Wgpusurface::new(&device, swapchain_format, size1, size2);
    let mut debug = Debug::new();
    let mut renderer = Renderer::new(Backend::new(&device, Settings::default(), swapchain_format));
    let mut state =
        iced_native::program::State::new(home, viewport.logical_size(), &mut renderer, &mut debug);
    let mut cursor_position = PhysicalPosition::new(0.0, 0.0);

    let mut a: i32 = 0;
    let mut total: i128 = 0;
    let mut current_latency = 0;

    event_loop.run(move |event, _, control_flow| {
        if total >= 1000000 {
            current_latency = a;
            total = 0;
            a = 0;
        }
        let now = Instant::now();

        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        // cursor_position_clone.swap(Box::new(position));
                        // mouse_state_clone.swap(Box::new(MOUSEEVENTF_MOVE));
                        // send_indicator_clone2.swap(Box::new(1));
                        cursor_position = position;
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers;
                    }

                    WindowEvent::Resized(new_size) => {
                        viewport = Viewport::with_physical_size(
                            Size::new(new_size.width, new_size.height),
                            window.scale_factor(),
                        );

                        resized = true;
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
                if let Some(event1) =
                    iced_winit::conversion::window_event(&event, window.scale_factor(), modifiers)
                {
                    //let lockarc = eventarcclone.lock();
                    state.queue_event(event1);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                window.request_redraw();
                if !state.is_queue_empty() {
                    // let mut h = HazardPointer::new();
                    // let cursor_position_temp =
                    //     cursor_position_clone.safe_load(&mut h).expect("msg");
                    // We update iced
                    let _ = state.update(
                        viewport.logical_size(),
                        conversion::cursor_position(
                            cursor_position.cast(),
                            viewport.scale_factor(),
                        ),
                        &mut renderer,
                        &iced_wgpu::Theme::Dark,
                        &renderer::Style {
                            text_color: Color::WHITE,
                        },
                        &mut clipboard,
                        &mut debug,
                    );

                    // and request a redraw
                }
            }
            Event::RedrawRequested(_) => {
                if resized {
                    let size = window.inner_size();

                    surface.configure(
                        &device,
                        &wgpu::SurfaceConfiguration {
                            format: swapchain_format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::Immediate,
                        },
                    );
                    resized = false;
                }
                match surface.get_current_texture() {
                    Ok(frame) => {
                        let view_texture = &frame.texture;
                        let view =
                            view_texture.create_view(&wgpu::TextureViewDescriptor::default());
                        //surface_texture_view_clone.swap(Box::new(view));
                        // let mut ind2 = HazardPointer::new();
                        // let surface_texture_view_clone_temp = surface_texture_view_clone
                        //     .safe_load(&mut ind2)
                        //     .expect("msg");
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                        // let program = state.program();

                        if current_latency != 0 {
                            state.queue_message(Message::UpdateHomeScreen(home::HomeScreenState {
                                fps: current_latency.to_string(),
                            }));
                        }

                        renderer.with_primitives(|backend, primitive| {
                            backend.present(
                                &device,
                                &mut staging_belt,
                                &mut encoder,
                                &view,
                                primitive,
                                &viewport,
                                &debug.overlay(),
                            );
                        });

                        // Then we submit the work
                        staging_belt.finish();
                        // Update the mouse cursor
                        window.set_cursor_icon(iced_winit::conversion::mouse_interaction(
                            state.mouse_interaction(),
                        ));

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                        staging_belt.recall();
                        // And recall staging buffers

                        total = total + now.elapsed().as_micros() as i128;
                        a = a + 1;
                    }
                    Err(error) => match error {
                        wgpu::SurfaceError::OutOfMemory => {
                            panic!("Swapchain error: {}. Rendering cannot continue.", error)
                        }
                        _ => {
                            // Try rendering again next frame.
                            //windowclone2.request_redraw();
                        }
                    },
                }
            }
            _ => {}
        }
    })
}

#[tokio::main]
async fn main() {
    start().await;
}
