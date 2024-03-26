use leptos::*;

mod components;
use components::texto_libre::TextoLibre;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| {set_count.update(|n|*n +=1);}>
        "Pincha"
        </button>
        <h1>
        {count}
        </h1>
        <TextoLibre />

    }
}

fn main() {
    leptos::mount_to_body(App)
}
