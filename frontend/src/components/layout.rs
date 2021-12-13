use mogwai::prelude::*;
use stylist::style;

use crate::{
    components::link::{self, ImgDim, Link},
    router::{Out, Route},
    util::get_styles,
};

pub struct LayoutStyles {
    wrap: String,
    header: String,
    nav: String,
}

pub fn styles(stylesheet: &mut Vec<String>) -> LayoutStyles {
    let wrap = style!(
        r#"
            width: 100%;
            display: flex;
            flex-direction: column;
            padding: 3vw 8vw;
        "#
    )
    .unwrap();

    let header = style!(
        r#"
            width: 100%;
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-between;

            a > img {
                width: auto;
                max-width: 100%;
                max-height: 178px;
            }

            @media only screen and (max-width: 800px) {
                a > img {
                    height: 178px;
                    width: 130px;
                    object-fit: cover;
                    object-position: top left;
                }
            }
        "#
    )
    .unwrap();

    let nav = style!(
        r#"
            display: flex;
            border-radius: 0.5rem;
            background-color: #000;
            padding: 1rem;
            padding-left: 1rem;

            a {
                color: #fff;
                display: inline-flex;
                line-height: 1.5em;
                font-size: 1em;
                text-decoration: none;
                padding: 0.5rem 0.75rem;
            }

            a:hover {
                border: 0.2rem solid #fff;
                border-radius: 0.2rem;
                padding: 0.3rem 0.55rem;
            }

            a:visited {
                color: #fff;
            }

            a.icon {
                padding: 0.25rem;
                text-decoration: none;
            }

            a.icon:hover {
                padding: 0.05rem;
            }
        "#
    )
    .unwrap();

    let styles = LayoutStyles {
        wrap: wrap.get_class_name().to_owned(),
        header: header.get_class_name().to_owned(),
        nav: nav.get_class_name().to_owned(),
    };

    stylesheet.push(get_styles(&[wrap, header, nav]));

    styles
}

pub fn view(
    tx: broadcast::Sender<Route>,
    rx: broadcast::Receiver<Out>,
    content: ViewBuilder<Dom>,
) -> ViewBuilder<Dom> {
    let LayoutStyles { wrap, header, nav } = styles(&mut vec![]);

    builder! {
        <div class=&wrap>
            <header class=&header>
                {link::image(
                    tx.clone(),
                    Link::Internal(Route::Home),
                    "",
                    "/static/images/grayblock_power_logo.png",
                    "Grayblock Power Logo",
                    ImgDim::None,
                )}
                <nav class=&nav>
                    {link::text(tx.clone(), Link::Internal(Route::Dashboard), "Dashboard")}
                    {link::text(tx.clone(), Link::Internal(Route::Projects), "Back a Project")}
                    {link::text(tx.clone(), Link::Internal(Route::Staking), "Energy Staking")}
                    {link::text(tx.clone(), Link::Internal(Route::Learn), "Learn more")}
                    {link::image(
                        tx,
                        Link::ExternalTarget("https://discord.gg/grayblockpower".to_owned()),
                        "icon",
                        "/static/images/discord.svg",
                        "Discord Logo",
                        ImgDim::Height(25),
                    )}
                </nav>
            </header>
            <slot patch:children=rx.filter_map(|out| async move { out.maybe_patch_route() })>
                {content}
            </slot>
        </div>
    }
}

/*
    <nav class=&nav>
        <a href="/about">"About"</a>
        <a href="/how-it-works">"How it works"</a>
        <a href="/community">"Join community"</a>
        <a href="/news">"News"</a>
        <a href="/faq">"FAQ"</a>
        <a href="/whitepaper">"Whitepaper"</a>
    </nav>
*/
