use yew_router::Routable;

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub id: usize,
    pub title: String,
    pub tooltip: String,
    pub route: Route,
    pub route_string: String,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/indicators")]
    Indicators,
    #[not_found]
    #[at("/404")]
    NotFound,
}
