use leptos::*;


#[component]
fn App() -> impl IntoView {
    let (color, set_color)=create_signal("red");
    let (mensaje, set_mensaje)=create_signal("Hola Mundo");
   view! {
    <p title={mensaje}>
    Si dejas el ratón encima verás que aparece un mensaje
    </p>
    <p
     
    style:color={color} on:click={
        move |_| {
            set_color.update(|color| {
                if *color == "red" {
                    *color ="blue";    
                }else{
                    *color="red"
                }
            })
        }
    }>
    Esto puede ser rojo...pero si haces click cambiará de color
    </p>

               
    }
}

fn main() {
    leptos::mount_to_body(App)
}
