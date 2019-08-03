extern crate dxgi;
extern crate windows_dpi;

use dxgi::adapter::{IAdapter, IAdapter1};
use dxgi::enums::Format;
use dxgi::factory::{Factory1, IFactory1};
use dxgi::output::IOutput;

fn main() {
    windows_dpi::enable_dpi();

    let factory: Factory1 = dxgi::factory::create().unwrap();

    for adapter in factory.adapters1() {
        assert!(adapter.factory().as_ref() == Some(&factory));

        let desc = adapter.desc1();
        println!(
            "Adapter [{}, {}MB]",
            desc.description(),
            desc.dedicated_video_memory / 1024 / 1024
        );

        for output in adapter.outputs() {
            let desc = output.desc();
            let coords = desc.desktop_coordinates;
            println!(
                "-- Output [{} @ ({}, {})]",
                desc.device_name(),
                coords.left,
                coords.top,
            );

            let mut mode_find = dxgi::descriptions::Mode::default();
            mode_find.format = Format::R8G8B8A8Unorm.into();
            mode_find.width = (coords.right - coords.left) as u32;
            mode_find.height = (coords.bottom - coords.top) as u32;
            let primary_mode = output.find_closest_matching_mode(&mode_find, None).unwrap();

            println!(
                "---- Primary Mode [{}x{} @ {}Hz]",
                primary_mode.width,
                primary_mode.height,
                primary_mode.refresh_rate.to_f64(),
            );
            println!("------ {:#?}", primary_mode);
        }
    }
}
