use eyre::Result;
use mapbox_maps::{Config, Map};
use std::env;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    let token =
        env::var("MAPBOX_ACCESS_TOKEN").expect("Provide MAPBOX_ACCESS_TOKEN as env variable.");
    let mut map = Map::new(Config::new(&token))?;

    map.load_style("mapbox/streets-v11").await?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } = input
                    {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                println!("redraw requested");
            }
            _ => (),
        }
    });
}
