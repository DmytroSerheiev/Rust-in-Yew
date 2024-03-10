use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <p>{ "John Doe: Building and breaking things" }</p>
                <p>{ "Jane Smith: The development process" }</p>
                <p>{ "Matt Miller: The Web 7.0" }</p>
                <p>{ "Tom Jerry: Mouseless development" }</p>
            </div>
            <div>
                <h3>{ "John Doe: Building and breaking things" }</h3>
                // Вставте код для вбудованого відеоплеєра YouTube
                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/k-gVqe3HgwY"
                    frameborder="0"
                    allowfullscreen=true
                ></iframe>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
