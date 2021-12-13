use mogwai::prelude::*;

pub fn view() -> ViewBuilder<Dom> {
    builder! {
        <main>
            <h1>"Projects"</h1>
            <p>"Coming soon. See our Testnet Demo for an older version of this page:"</p>
            <a href="https://gpnswap-stagingwebsite.netlify.app">"Testnet Demo"</a>
        </main>
    }
}
