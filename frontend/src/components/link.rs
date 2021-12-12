// Warning: This whole component is a mess.

use mogwai::prelude::*;

use crate::router::Route;

#[allow(dead_code)]
pub enum Link {
    Internal(Route),
    External(String),
    ExternalTarget(String),
}

pub fn text(tx: broadcast::Sender<Route>, link: Link, content: &str) -> ViewBuilder<Dom> {
    match link {
        Link::Internal(route) => {
            builder! {
                <a
                    href=route.to_string()
                    on:click=tx.sink().contra_filter_map(move |e: DomEvent| {
                        let ev = e.browser_event()?;
                        ev.prevent_default();
                        Some(route)
                    })
                >{content}</a>
            }
        }
        Link::External(href) => {
            builder! {
                <a
                    href=href
                >
                {content}
                </a>
            }
        }
        Link::ExternalTarget(href) => {
            builder! {
                <a
                    href=href
                    target="_blank"
                >
                {content}
                </a>
            }
        }
    }
}

#[allow(dead_code)]
pub enum ImgDim {
    None,
    Height(usize),
    Width(usize),
    WidthHeight(usize, usize),
}

pub fn image(
    tx: broadcast::Sender<Route>,
    link: Link,
    class: &str,
    src: &str,
    alt: &str,
    dim: ImgDim,
) -> ViewBuilder<Dom> {
    let img = match dim {
        ImgDim::None => builder! {
            <img src=src alt=alt />
        },
        ImgDim::Height(height) => builder! {
            <img src=src alt=alt height=height.to_string() />
        },
        ImgDim::Width(width) => builder! {
            <img src=src alt=alt width=width.to_string() />
        },
        ImgDim::WidthHeight(width, height) => builder! {
            <img src=src alt=alt width=width.to_string() height=height.to_string() />
        },
    };

    match link {
        Link::Internal(route) => {
            builder! {
                <a
                    href=route.to_string()
                    on:click=tx.sink().contra_filter_map(move |e: DomEvent| {
                        let ev = e.browser_event()?;
                        ev.prevent_default();
                        Some(route)
                    })
                    class=class
                >
                    {img}
                </a>
            }
        }
        Link::External(href) => {
            builder! {
                <a
                    href=href
                    class=class
                >
                    {img}
                </a>
            }
        }
        Link::ExternalTarget(href) => {
            builder! {
                <a
                    href=href
                    class=class
                    target="_blank"
                >
                    {img}
                </a>
            }
        }
    }
}
