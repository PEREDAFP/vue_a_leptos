use leptos::*;


#[component]
fn App() -> impl IntoView {
    let (mensaje, set_texto)=create_signal("Hola Mundo".to_string());
   view! {
    <h1>
    {mensaje}
    </h1>
               
    }
}

fn main() {
    leptos::mount_to_body(App)
}
