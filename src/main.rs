use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="header-container">
                <header>{ "Work In Progress" }</header>
                <small>{ "Probably forever..." }</small>
            </div>
        }
    }
}
