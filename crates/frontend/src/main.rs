mod app;
mod components;
mod services;
mod state;
mod utils;

fn main() {
    dioxus::launch(app::App);
}
