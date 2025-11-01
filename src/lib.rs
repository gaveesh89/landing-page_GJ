use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

mod components;
use components::*;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="Atomic Increment Ltd."/>
        <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
            <Header/>
            <HeroSection/>
            <FeaturesSection/>
            <TestimonialsSection/>
            <ContactSection/>
            <Footer/>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
