use dxgi::adapter::{Adapter4, IAdapter4};
use dxgi::enums::GpuPreference;
use dxgi::factory::{Factory6, IFactory6};

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
            .adapters_by_preference::<Adapter4>(GpuPreference::Unspecified)
            .map(|a| a.unwrap().desc3().description())
            .collect::<Vec<_>>()
    );
    println!(
        "Minimum Power adapter order: {:#?}",
        factory
            .adapters_by_preference::<Adapter4>(GpuPreference::MinimumPower)
            .map(|a| a.unwrap().desc3().description())
            .collect::<Vec<_>>()
    );
    println!(
        "High Performance adapter order: {:#?}",
        factory
            .adapters_by_preference::<Adapter4>(GpuPreference::HighPerformance)
            .map(|a| a.unwrap().desc3())
            .collect::<Vec<_>>()
    );
}
