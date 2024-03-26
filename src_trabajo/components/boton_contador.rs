use leptos::*;
use leptos::logging::log;

#[component]

pub fn BotonContador() -> impl IntoView {
let (count, set_count) = create_signal(0);
view! {
    <>
        <button on:click=move|_| set_count.update(|n|{
            log!("El valor del contador es {}", n);
            *n+=1
        } )>
            "Pincha"
        </button>
        <h1>
            {count}
        </h1>       
    </>
}
}