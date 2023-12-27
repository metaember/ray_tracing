use clap::Parser;

/// Run the ray tracer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene name
    name: String,
}
fn main() {
    let args = Args::parse();
    ray_tracing::main(&args.name);
}
