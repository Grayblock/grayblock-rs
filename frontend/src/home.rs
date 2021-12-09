use mogwai::prelude::*;
use stylist::style;

pub fn view() -> ViewBuilder<Dom> {
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
            padding: 8vw;
            padding-bottom: 20vmax;

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
                margin: 1rem 0;
            }

             a {
                margin: 2rem 0;
                padding: 1rem 1.67rem;
                font-size: calc((1.5 - 1) * 1.2vw + 1rem);
                font-weight: 500;
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
        "#
    )
    .unwrap();

    builder! {
        <div class=wrap.get_class_name()>
            <header class=header.get_class_name()>
                <a href="/"><img src="/static/images/grayblock_power_logo.png" alt="Grayblock Power logo" /></a>
                <nav class=nav.get_class_name()>
                    <a href="https://medium.com/@grayblockpower">"News"</a>
                    <a href="/files/project-docs/Grayblock_FAQ.pdf">"FAQ"</a>
                    <a href="/files/project-docs/Grayblock_Whitepaper.pdf">"Whitepaper"</a>
                    <a class=button.get_class_name() href="https://discord.gg/grayblockpower">"Join Community"</a>
                </nav>
            </header>
            <main class=main.get_class_name()>
                <h1>"The Future of Energy Finance, Built on "<span>"Avalanche"</span></h1>
                <h2>"crowdfunding clean energy projects with crypto"</h2>
                <a class=button.get_class_name() href="https://discord.gg/grayblockpower">"Launch Testnet Demo"</a>
            </main>
        </div>
    }
}
