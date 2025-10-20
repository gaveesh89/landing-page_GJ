use leptos::*;

#[component]
pub fn TestimonialsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl">
                        "What Our Clients Say"
                    </h2>
                    <p class="mt-4 text-xl text-gray-600">
                        "Trusted by leading organizations worldwide"
                    </p>
                </div>
                
                // <div class="mt-20">
                //     <div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
                //         <TestimonialCard
                //             quote="The team delivered exceptional results with their Rust expertise. Our application performance improved by 300% while maintaining perfect security standards."
                //             author="Sarah Johnson"
                //             role="CTO, TechCorp"
                //         />
                //     </div>
                // </div>
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
                <div class="flex-shrink-0">
                    <svg class="h-8 w-8 text-blue-600" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M14.017 21v-7.391c0-5.704 3.731-9.57 8.983-10.609l.995 2.151c-2.432.917-3.995 3.638-3.995 5.849h4v10h-9.983zm-14.017 0v-7.391c0-5.704 3.748-9.57 9-10.609l.996 2.151c-2.433.917-3.996 3.638-3.996 5.849h3.983v10h-9.983z"/>
                    </svg>
                </div>
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