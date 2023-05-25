use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

#[function_component(Canvas)]
fn canvas() -> Html {
    html!{
        <canvas id="gameCanvas" width="200" height="200"></canvas>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}