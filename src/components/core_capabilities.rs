use leptos::*;

#[component]
pub fn CoreCapabilitiesSection() -> impl IntoView {
    view! {
        <section id="capabilities" class="py-20 bg-gray-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-16">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl mb-4">
                        "Core Capabilities"
                    </h2>
                    <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                        "Advanced technical expertise delivering robust, high-performance solutions"
                    </p>
                </div>
                
                <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                    <CapabilityCard
                        title="Performance-Critical Systems"
                        description="Architected with Rust for unmatched speed, safety, and reliability in production environments."
                        icon_type="performance"
                    />
                    <CapabilityCard
                        title="Memory Safety Mandate"
                        description="Security-by-design approach ensuring robust, protected solutions. Eliminate common vulnerabilities at the compiler level."
                        icon_type="security"
                    />
                    <CapabilityCard
                        title="SIMD Optimization"
                        description="Advanced SIMD techniques deployed for maximum throughput and cost-efficient computation."
                        icon_type="optimization"
                    />
                    <CapabilityCard
                        title="Systems-Level Expertise"
                        description="Seasoned professionals with deep expertise in Rust's compiler, ownership model, and tooling ecosystems."
                        icon_type="expertise"
                    />
                    <CapabilityCard
                        title="Architectural Precision"
                        description="Leveraging advanced computer science principles to drive innovation and efficiency in systems design."
                        icon_type="architecture"
                    />
                    <CapabilityCard
                        title="Data Integrity"
                        description="Comprehensive solutions for high-throughput data processing and advanced analytics, built on Rust's reliability."
                        icon_type="data"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn CapabilityCard(
    title: &'static str,
    description: &'static str,
    icon_type: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative group">
            <div class="bg-white rounded-lg shadow-lg border border-gray-200 p-6 h-full hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1">
                {/* Icon */}
                <div class="w-12 h-12 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center mb-4">
                    {match icon_type {
                        "performance" => view! {
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                        }.into_view(),
                        "security" => view! {
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5-6a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                        }.into_view(),
                        "optimization" => view! {
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                            </svg>
                        }.into_view(),
                        "expertise" => view! {
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
                            </svg>
                        }.into_view(),
                        "architecture" => view! {
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                            </svg>
                        }.into_view(),
                        "data" => view! {
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"></path>
                            </svg>
                        }.into_view(),
                        _ => view! {
                            <div class="w-6 h-6 bg-gray-400 rounded"></div>
                        }.into_view(),
                    }}
                </div>
                
                {/* Content */}
                <h3 class="text-lg font-semibold text-gray-900 mb-3">{title}</h3>
                <p class="text-gray-600 leading-relaxed">{description}</p>
            </div>
        </div>
    }
}