use mogwai::prelude::*;
use stylist::style;

use crate::util::get_styles;

pub struct HomeStyles {
    wrap: String,
    main: String,
    header: String,
    nav: String,
    button: String,
}

pub fn styles(stylesheet: &mut Vec<String>) -> HomeStyles {
    let wrap = style!(
        r#"
            width: 100%;
            display: flex;
            flex-direction: column;
            padding: 3vw 8vw;
        "#
    )
    .unwrap();

    let main = style!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 8vw 8vw 20vmin;

            h1 {
                text-align: center;
                white-space: pre-wrap;
                margin: 1rem 0;
                font-family: Manrope;
                font-weight: 600;
                font-size: calc((3.6 - 1) * 1.2vw + 1rem);
                line-height: 1.1648em;
            }

            h1 > span {
                color: #F53636;
                word-wrap: break-word;
                text-align: center;
                white-space: pre-wrap;
            }

            h2 {
                font-family: Manrope;
                font-weight: 400;
                font-size: calc((1.4 - 1) * 1.2vw + 1rem);
                text-align: center;
                margin: 1rem 0;
            }

             a {
                margin: 2rem 0;
                padding: 1rem 1.67rem;
                font-size: calc((1.5 - 1) * 1.2vw + 1rem);
                font-weight: 500;
             }

            @media only screen and (max-width: 800px) {
                & {
                    padding: 0 0 20vmin;
                }

                h1 {
                    font-size: calc((3.6 - 1) * 1.2vh + 1rem);
                }

                h2 {
                    font-size: calc((1.4 - 1) * 1.2vh + 1rem);
                }
            }
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
            border-radius: 0.4rem;
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

    let styles = HomeStyles {
        wrap: wrap.get_class_name().to_owned(),
        main: main.get_class_name().to_owned(),
        header: header.get_class_name().to_owned(),
        nav: nav.get_class_name().to_owned(),
        button: button.get_class_name().to_owned(),
    };

    stylesheet.push(get_styles(&[wrap, main, header, nav, button]));

    styles
}

pub fn view() -> ViewBuilder<Dom> {
    let HomeStyles {
        wrap,
        main,
        header,
        nav,
        button,
    } = styles(&mut vec![]);

    builder! {
        <div class=&wrap>
            <header class=&header>
                <a href="/"><img src="/static/images/grayblock_power_logo.png" alt="Grayblock Power logo" /></a>
                <nav class=&nav>
                    <a href="https://medium.com/@grayblockpower">"News"</a>
                    <a href="/files/project-docs/Grayblock_FAQ.pdf">"FAQ"</a>
                    <a href="/files/project-docs/Grayblock_Whitepaper.pdf">"Whitepaper"</a>
                    <a class=&button href="https://discord.gg/grayblockpower">"Join Community"</a>
                </nav>
            </header>
            <main class=&main>
                <h1>"The Future of Energy Finance, Built on "<span>"Avalanche"</span></h1>
                <h2>"crowdfunding clean energy projects with crypto"</h2>
                <a class=&button href="https://demo.grayblockpower.com/">"Launch Testnet Demo"</a>
            </main>
        </div>
    }
}
