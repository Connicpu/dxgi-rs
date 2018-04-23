extern crate dxgi;
extern crate winapi;
extern crate windows_dpi;

use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;

fn main() {
    windows_dpi::enable_dpi();

    let factory = dxgi::factory::Factory::new().unwrap();

    for adapter in factory.adapters() {
        assert!(adapter.get_factory() == factory);

        let desc = adapter.get_desc();
        println!(
            "Adapter [{}, {}MB]",
            desc.description(),
            desc.dedicated_video_memory() / 1024 / 1024
        );
        
        for output in adapter.outputs() {
            let desc = output.get_desc();
            let coords = desc.desktop_coordinates();
            println!(
                "-- Output [{} @ ({}, {})]",
                desc.device_name(),
                coords.left,
                coords.top,
            );

            let mut mode_find = dxgi::output::Mode::new();
            mode_find.set_format(DXGI_FORMAT_R8G8B8A8_UNORM);
            mode_find.set_width((coords.right - coords.left) as u32);
            mode_find.set_height((coords.bottom - coords.top) as u32);
            let primary_mode = output.find_closest_matching_mode(&mode_find, None).unwrap();

            let refresh = primary_mode.refresh_rate();
            let hz = *refresh.numer() as f64 / *refresh.denom() as f64;
            println!(
                "---- Primary Mode [{}x{} @ {}Hz]",
                primary_mode.width(),
                primary_mode.height(),
                hz,
            );
            println!("------ {:?}", primary_mode);
        }
    }
}
