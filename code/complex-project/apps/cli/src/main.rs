// apps/cli/src/main.rs
use clap::Parser;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "demo", about = "Feature-gated plugin demo")]
struct Args {
    /// Message to dispatch
    #[arg(default_value = "hello from workspace")]
    msg: String,

    /// Also add a local echo handler
    #[arg(long)]
    echo: bool,
}

fn main() {
    let args = Args::parse();

    let mut bus = corelib::Bus::new();
    plugins::register_builtin_plugins(&mut bus);

    if args.echo {
        struct Echo;
        impl corelib::Handler for Echo {
            fn name(&self) -> &'static str { "echo" }
            fn handle(&self, msg: &str) { println!("[echo] {msg}"); }
        }
        bus.register(Arc::new(Echo));
    }

    println!("handlers: {:?}", bus.list().collect::<Vec<_>>());
    bus.dispatch(&args.msg);
}

//////////////////////////////////////////////////////////////////////////////////////////////
/// cargo run -p cli --features plugins/plugin_a
/// cargo run -p cli --features plugins/plugin_b
/// cargo run -p cli --features "plugins/plugin_a plugins/plugin_b" -- --echo "custom message"