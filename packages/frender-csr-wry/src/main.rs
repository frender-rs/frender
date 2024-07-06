use std::time::Duration;

use futures_lite::StreamExt;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello World")
        .build(&event_loop)
        .unwrap();

    let webview = WebViewBuilder::new(&window)
        .with_html("")?
        // .with_url("https://tauri.studio")?
        .build()?;

    let proxy = event_loop.create_proxy();
    let waker = {
        let proxy = proxy.clone();
        waker_fn::waker_fn(move || _ = proxy.send_event(()))
    };

    let mut timer = async_io::Timer::interval(Duration::from_secs(1));
    let mut init_instant = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Wry has started!");
                _ = proxy.send_event(());
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::UserEvent(()) => loop {
                match timer.poll_next(&mut std::task::Context::from_waker(&waker)) {
                    std::task::Poll::Ready(Some(instant)) => {
                        let duration = if let Some(init_instant) = init_instant {
                            instant.duration_since(init_instant).as_secs()
                        } else {
                            init_instant = Some(instant);
                            0
                        };

                        println!("{:}", duration);
                        webview
                            .evaluate_script(&format!("document.body.innerText = '{}'", duration))
                            .unwrap();
                    }
                    std::task::Poll::Ready(None) => webview
                        .evaluate_script("document.body.innerText = 'end'")
                        .unwrap(),
                    std::task::Poll::Pending => break,
                }
            },
            _ => (),
        }
    });
}
