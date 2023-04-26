use yew::prelude::*; // Import the Yew module

#[function_component]
fn App() -> Html {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap(); // Use the web-sys library to access values in local storage. Unwrap it twice to get it as a string

    let value = local_storage.get_item("numberOfVisits").unwrap(); // Use the local storage object to get the 'numberOfVisits' value from the localstorage
    html! { // Create html code
        <div>
            <p>{ value }</p> // Create a paragraph that contains the 'numberOfVisits' value from before
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // Render the html with Yew
}
