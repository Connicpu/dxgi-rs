use dxgi::factory::Factory6;
use dxgi::enums::GpuPreference;
use dxgi::adapter::Adapter;

fn main() {
    let factory: Factory6 = match dxgi::factory::create() {
        Ok(factory) => factory,
        Err(_) => {
            eprintln!("You can't have a Factory6 :(");
            return;
        }
    };

    println!("{:?}", factory);

    println!(
        "Unspecified adapter order: {:#?}",
        factory
            .adapters_by_preference::<Adapter>(GpuPreference::Unspecified)
            .map(|a| a.unwrap().desc().description())
            .collect::<Vec<_>>()
    );
    println!(
        "Minimum Power adapter order: {:#?}",
        factory
            .adapters_by_preference::<Adapter>(GpuPreference::MinimumPower)
            .map(|a| a.unwrap().desc().description())
            .collect::<Vec<_>>()
    );
    println!(
        "High Performance adapter order: {:#?}",
        factory
            .adapters_by_preference::<Adapter>(GpuPreference::HighPerformance)
            .map(|a| a.unwrap().desc().description())
            .collect::<Vec<_>>()
    );
}