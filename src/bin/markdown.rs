use altagos::markdown::Markdown;
use yew_agent::PublicWorker;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Markdown::register();
}
