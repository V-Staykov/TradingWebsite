use stylist::Style;
use trading_website::features::common::ui::entities::Route;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

use yew_router::prelude::*;

use trading_website::features::common::ui::{components::Header, entities::MenuItem};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Blog => html! { <h1>{ "Blog" }</h1> },
        Route::Indicators => html! { <h1>{ "Indictors" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let main_styles = Style::new(STYLE_FILE).unwrap();

    let links: Vec<MenuItem> = vec![
        MenuItem {
            id: 1,
            title: "Начало".to_owned(),
            tooltip: "".to_owned(),
            route_string: "/".to_owned(),
            route: Route::Home,
        },
        MenuItem {
            id: 2,
            title: "Блог".to_owned(),
            tooltip: "Блог".to_owned(),
            route_string: "/blog".to_owned(),
            route: Route::Blog,
        },
        MenuItem {
            id: 3,
            title: "Индикатори".to_owned(),
            tooltip: "Индикатори".to_owned(),
            route_string: "/indicators".to_owned(),
            route: Route::Indicators,
        },
    ];

    html! {
        <main class={classes!("Main", main_styles)}>
            <BrowserRouter>
                <Header links = { links }/>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
