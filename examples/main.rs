use eyre::Result;
use futures::executor::block_on;
use mapbox_maps::{Config, Map};
use std::env;
use std::rc::Rc;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let window = Rc::new(window);

    let token =
        env::var("MAPBOX_ACCESS_TOKEN").expect("Provide MAPBOX_ACCESS_TOKEN as env variable.");
    let mut map = Map::new(Config::new(&token, window.clone()))?;

    map.load_style("mapbox/streets-v11").await?;

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
                block_on(map.render()).expect("Render failed");
            }
            _ => (),
        }
    });
}
