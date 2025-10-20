use leptos::*;

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        <section id="services" class="py-20 bg-gray-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl">
                        "What Makes Us Different"
                    </h2>
                    <p class="mt-4 text-xl text-gray-600">
                        "Our expertise spans across modern technologies and proven methodologies"
                    </p>
                </div>
                
                <div class="mt-20">
                    <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                        <FeatureCard
                            icon="⚡"
                            title="High Performance and low power"
                            description="Built with Rust for unmatched speed, safety, and reliability in production environments."
                        />
                        <FeatureCard
                            icon="🔒"
                            title="Security First"
                            description="Security-by-design approach ensuring robust and protected solutions from the ground up."
                        />
                        <FeatureCard
                            icon="🚀"
                            title="SIMD Scalable Architecture"
                            description="Save a fortune in GPU costs by using advanced SIMD techniques."
                        />
                        <FeatureCard
                            icon="🛠️"
                            title="Compiler engineers"
                            description="Seasoned professionals with deep expertise in modern technologies and best practices."
                        />
                        <FeatureCard
                            icon="☁️"
                            title="Computer science lead"
                            description="Using cutting-edge techniques to drive innovation and efficiency."
                        />
                        <FeatureCard
                            icon="📊"
                            title="Data Driven"
                            description="Comprehensive solutions spanning data processing, AI, and advanced analytics capabilities."
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative group">
            <div class="bg-white rounded-lg shadow-md p-6 h-full hover:shadow-lg transition-shadow duration-300">
                <div class="text-4xl mb-4">{icon}</div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
                <p class="text-gray-600">{description}</p>
            </div>
        </div>
    }
}