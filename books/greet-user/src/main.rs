use anyhow;
use clap::Parser;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};

wasmtime::component::bindgen!({
    path: "../greet/wit",
    world: "greetable-provider",
});

fn main() {
    let args = Args::parse();

    if let Err(e) = start(args) {
        println!("Error: {:?}", e);
    };
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    wasm_file: String,
}

fn start(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, args.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let provider = GreetableProvider::instantiate(&mut store, &component, &linker)?;

    // 呼び出し①
    let result = provider
        .garaoh70_greet_greetable()
        .call_greet(&mut store, "arg0")?;
    println!("greet: {}", result);

    // 呼び出し②
    let name = provider.garaoh70_greet_greetable().call_name(&mut store)?;
    println!("name: {}", name);

    Ok(())
}
