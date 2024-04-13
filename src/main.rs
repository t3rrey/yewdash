use log::info;
use yew::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <header>
            <h1>{"Hello, Yew!"}</h1>
        </header>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| 0);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| {
            let new_value = *state + 1;
            state.set(new_value);
        })
    };

    let decr_counter = {
        let state = state.clone();
        Callback::from(move |_| {
            let new_value = *state - 1;
            state.set(new_value);
        })
    };

    // Use an effect to log the state change
    use_effect_with_deps(
        |state| {
            info!("State updated to: {}", *state);
            || ()
        },
        (*state).clone(),
    );

    html! {
        <>
        <Header />
            <p> {"Current count: "} {*state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            <button onclick={decr_counter}> {"-"} </button>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
