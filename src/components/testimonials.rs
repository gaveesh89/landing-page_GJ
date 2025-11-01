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
                </div>
                
                <div class="mt-20">
                    <div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
                        <TestimonialCard
                            quote="Atomic Increment accelerated our team's adoption timeline by 60%. Their guidance on the ownership model was non-negotiable for our embedded systems project."
                            author="VP of Engineering"
                            role="Global Infrastructure Firm"
                        />
                        <TestimonialCard
                            quote="The transition from C++ to Rust was de-risked by their architectural review. Production deployment achieved 4x performance gain on core logic."
                            author="Chief Architect"
                            role="High-Frequency Trading Platform"
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