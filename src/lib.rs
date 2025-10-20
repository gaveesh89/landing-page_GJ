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
        <Stylesheet id="leptos" href="/pkg/landing-page.css"/>
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
        <Title text="Company Landing Page"/>
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

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}