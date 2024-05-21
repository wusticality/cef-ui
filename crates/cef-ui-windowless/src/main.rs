use anyhow::Result;
use bevy::{
    app::{App, Update},
    prelude::{default, PluginGroup},
    render::camera::ClearColor,
    window::{PresentMode, WindowPlugin},
    DefaultPlugins
};
use cef::{Cef, CefSettings};
use cef_ui::LogSeverity;
use messages::MessageLoopEvent;
use std::{
    process::exit,
    sync::{Arc, Mutex}
};
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder}
};

mod cef;
mod messages;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);

        exit(1);
    }
}

fn try_main() -> Result<()> {
    // // This routes log macros through tracing.
    // LogTracer::init()?;

    // // Setup the tracing subscriber globally.
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(LevelFilter::from_level(Level::DEBUG))
    //     .finish();

    // set_global_default(subscriber)?;

    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(bevy::prelude::Window {
        //         title: "cef-ui-windowless".to_string(),
        //         present_mode: PresentMode::Immediate,
        //         resizable: true,
        //         ..default()
        //     }),
        //     ..default()
        // }))
        //.add_systems(Update, hello_world)
        .run();

    // // We need to create this before any window is created because CEF
    // // launches subprocesses and we don't want to create windows for them.
    // let event_loop = make_event_loop()?;

    // // Initialize the CEF context.
    // let mut cef = Cef::new(CefSettings {
    //     log_severity: LogSeverity::Warning,
    //     proxy:        Arc::new(Mutex::new(event_loop.create_proxy())),
    //     frame_rate:   60
    // })?;

    // // If this is a CEF subprocess, let it run and then
    // // emit the proper exit code so CEF can clean up.
    // if let Some(code) = cef.is_cef_subprocess() {
    //     exit(code);
    // }

    // // Make the window.
    // let window = make_window(&event_loop)?;

    // // Initialize CEF.
    // cef.initialize(window.clone())?;

    // // Run the event loop.
    // event_loop.run(move |event, _| {
    //     let close = |cef: &mut Cef| {
    //         cef.shutdown();

    //         exit(0);
    //     };

    //     match event {
    //         Event::WindowEvent {
    //             event: WindowEvent::CloseRequested,
    //             ..
    //         } => {
    //             close(&mut cef);
    //         },

    //         _ => {}
    //     }

    //     // Pass input events to CEF.
    //     if cef
    //         .update(&event, window.clone())
    //         .is_err()
    //     {
    //         close(&mut cef);
    //     }
    // })?;

    Ok(())
}

fn hello_world() {
    println!("hello world!");
}

// /// Create the event loop.
// fn make_event_loop() -> Result<EventLoop<MessageLoopEvent>> {
//     let event_loop = EventLoopBuilder::<MessageLoopEvent>::with_user_event().build()?;

//     event_loop.set_control_flow(ControlFlow::Poll);

//     Ok(event_loop)
// }

// /// Build the actual window using the correct event loop.
// fn make_window(event_loop: &EventLoop<MessageLoopEvent>) -> Result<Arc<Window>> {
//     let window = WindowBuilder::new()
//         .with_inner_size(LogicalSize::new(1280, 720))
//         .build(&event_loop)?;

//     Ok(window.into())
// }
