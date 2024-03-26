use leptos::*;
//TODO tengo que revisar cómo utilizar los módulos
mod components;
use components::texto_libre::TextoLibre;
use components::boton_contador::BotonContador;


#[component]
fn App() -> impl IntoView {
    let (texto, set_texto)=create_signal("Desde el componente".to_string());

   view! {
        <BotonContador />
        <TextoLibre texto={texto} num="jijiji".to_string()/>
        
    }
}

fn main() {
    leptos::mount_to_body(App)
}
