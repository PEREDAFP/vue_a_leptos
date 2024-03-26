use leptos::*;
//TODO tengo que revisar cómo utilizar los módulos
mod components;
use components::texto_libre::TextoLibre;
use components::texto_libre2::TextoLibre2;
use components::boton_contador::BotonContador;


#[component]
fn App() -> impl IntoView {
   view! {
        <BotonContador />
        <TextoLibre />
        <TextoLibre2 />

    }
}

fn main() {
    leptos::mount_to_body(App)
}
