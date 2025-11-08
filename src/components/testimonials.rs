use leptos::*;

#[component]
pub fn TestimonialsSection() -> impl IntoView {
    view! {
        <section id="about" class="py-20 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl">
                        "What Our Clients Say"
                    </h2>
                    <p class="mt-4 text-xl text-gray-600">
                        "Trusted by leading organizations worldwide"
                    </p>
                </div>
                
                <div class="mt-20">
                    <div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
                        <TestimonialCard
                            quote="Amy is a fantastic, engaging educator with exceptional technical knowledge. They are also able to bring their extensive real-world experience into the classroom."
                            author="Ian Watson"
                            role="Framework Training"
                        />
                        <TestimonialCard
                            quote="Amy covered Rust succinctly over a 3-day workshop for a group of 15 engineers from various disciplines. Amy interleaved many relevant anecdotes to keep the course engaging. I would thoroughly recommend Amy as an excellent Rust instructor."
                            author="Gajinder Panesar"
                            role="Chief Architect, Optalysys"
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TestimonialCard(
    quote: &'static str,
    author: &'static str,
    role: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-gray-50 rounded-lg p-8">
            <div class="flex items-start">
                <div class="ml-4">
                    <blockquote class="text-lg text-gray-900 font-medium">
                        {quote}
                    </blockquote>
                    <div class="mt-4">
                        <div class="font-semibold text-gray-900">{author}</div>
                        <div class="text-gray-600">{role}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}