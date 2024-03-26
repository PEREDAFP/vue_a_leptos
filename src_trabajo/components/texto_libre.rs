use leptos::*;

#[component]
pub fn TextoLibre(
    texto: ReadSignal<String>,
    #[prop(optional)]
    num:String
) -> impl IntoView {
    
    view! {
        <h1>
        {texto}
        </h1>
        <h2>
        {num}
        </h2>
    }
}