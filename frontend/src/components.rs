use stylist::{style, Style};

pub mod layout;

pub fn button() -> Style {
    style!(
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
    .unwrap()
}
