use crate::features::{
    common::ui::entities::{MenuItem, Route},
    utils::get_active_class,
};
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::{use_location, use_navigator, Link, Location};
const STYLE_FILE: &str = include_str!("../styles/header.css");

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub links: Vec<MenuItem>,
}

#[function_component(Header)]
pub fn header(HeaderProps { links }: &HeaderProps) -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let location = use_location().unwrap().path().to_string();

    html! {
        <div class={classes!(stylesheet, "FlexRow", "JustifyBetween", "FlexGrow")}>
            <div class={"Logo"}></div>
            <div class={"Menu FlexRow JustifyBetween"}>
                {
                    links
                    .iter()
                    .map(|link| {
                        html! {
                            <Link<Route> classes={classes!("Link", get_active_class(location.ends_with(&link.route_string)))} to={link.route.clone()}>{link.title.clone()}</Link<Route>>
                        }
                    })
                    .collect::<Html>()
                }
            </div>
        </div>
    }
}
