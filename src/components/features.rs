use leptos::*;

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        <section id="services" class="py-20 bg-gray-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl">
                        "CORE CAPABILITIES"
                    </h2>
                </div>
                
                <div class="mt-20">
                    <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                        <FeatureCard
                            title="PERFORMANCE-CRITICAL SYSTEMS"
                            description="Architected with Rust for unmatched speed, safety, and reliability in production environments."
                        />
                        <FeatureCard
                            title="MEMORY SAFETY MANDATE"
                            description="Security-by-design approach ensuring robust, protected solutions. Eliminate common vulnerabilities at the compiler level."
                        />
                        <FeatureCard
                            title="SIMD OPTIMIZATION"
                            description="Advanced SIMD techniques deployed for maximum throughput and cost-efficient computation."
                        />
                        <FeatureCard
                            title="SYSTEMS-LEVEL EXPERTISE"
                            description="Seasoned professionals with deep expertise in Rust's compiler, ownership model, and tooling ecosystems."
                        />
                        <FeatureCard
                            title="ARCHITECTURAL PRECISION"
                            description="Leveraging advanced computer science principles to drive innovation and efficiency in systems design."
                        />
                        <FeatureCard
                            title="DATA INTEGRITY"
                            description="Comprehensive solutions for high-throughput data processing and advanced analytics, built on Rust's reliability."
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative group">
            <div class="bg-white rounded-lg shadow-md p-6 h-full hover:shadow-lg transition-shadow duration-300">
                <h3 class="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
                <p class="text-gray-600">{description}</p>
            </div>
        </div>
    }
}