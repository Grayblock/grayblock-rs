use mogwai::prelude::*;
use stylist::style;

use crate::{components::button, util::get_styles};

pub struct HomeStyles {
    main: String,
    button: String,
}

pub fn styles(stylesheet: &mut Vec<String>) -> HomeStyles {
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

    let button = button();

    let styles = HomeStyles {
        main: main.get_class_name().to_owned(),
        button: button.get_class_name().to_owned(),
    };

    stylesheet.push(get_styles(&[main, button]));

    styles
}

pub fn view() -> ViewBuilder<Dom> {
    let HomeStyles { main, button } = styles(&mut vec![]);

    builder! {
        <main class=&main>
            <h1>"The Future of Energy Finance, Built on "<span>"Avalanche"</span></h1>
            <h2>"crowdfunding clean energy projects with crypto"</h2>
            <a class=&button href="https://demo.grayblockpower.com/">"Launch Testnet Demo"</a>
        </main>
    }
}
