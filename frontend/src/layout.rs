use mogwai::prelude::*;
use stylist::style;

use crate::util::get_styles;

pub struct LayoutStyles {
    wrap: String,
    header: String,
    nav: String,
    button: String,
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
            display: block;
            border-radius: 0.5rem;
            background-color: #000;
            padding: 1rem;
            padding-left: 0;

            a {
                display: inline-block;
                line-height: 1.5em;
                font-size: 1em;
                text-decoration: none;
                margin-left: 1.7vw;
            }
        "#
    )
    .unwrap();

    let button = style!(
        r#"
            line-height: 1.2em;
            color: #000;
            background-color: #fff;
            border-radius: 0.25rem;
            padding: 0.8rem 1.336rem;
            margin-left: 2.5vw;
            text-decoration: none;

            &:hover {
                color: #000;
                opacity: 0.8;
            }

            &:visited {
                color: #000;
            }
        "#
    )
    .unwrap();

    let styles = LayoutStyles {
        wrap: wrap.get_class_name().to_owned(),
        header: header.get_class_name().to_owned(),
        nav: nav.get_class_name().to_owned(),
        button: button.get_class_name().to_owned(),
    };

    stylesheet.push(get_styles(&[wrap, header, nav, button]));

    styles
}

pub fn view(content: ViewBuilder<Dom>) -> ViewBuilder<Dom> {
    let LayoutStyles {
        wrap,
        header,
        nav,
        button,
    } = styles(&mut vec![]);

    builder! {
        <div class=&wrap>
            <header class=&header>
                <a href="/"><img src="/static/images/grayblock_power_logo.png" alt="Grayblock Power logo" /></a>
                <nav class=&nav>
                    <a href="/dashboard">"Dashboard"</a>
                    <a href="/offerings">"Offerings"</a>
                    <a href="/projects">"Projects"</a>
                    <a href="/about">"About"</a>
                    <a href="https://medium.com/@grayblockpower">"News"</a>
                    <a href="/files/site-docs/Grayblock_FAQ.pdf">"FAQ"</a>
                    <a href="/files/site-docs/Grayblock_Whitepaper.pdf">"Whitepaper"</a>
                    <a class=&button href="https://discord.gg/grayblockpower">"Join Community"</a>
                </nav>
            </header>
            {content}
        </div>
    }
}
