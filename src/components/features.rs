use leptos::*;

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        <section id="expertise" class="py-20 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-16">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl mb-4">
                        "Areas of Expertise"
                    </h2>
                    <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                        "Comprehensive Rust solutions tailored to accelerate your development cycle and optimize system performance"
                    </p>
                </div>
                
                <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
                    <ExpertiseCard
                        title="Rust Training"
                        description="Comprehensive training programs designed to upskill your development team. From fundamentals to advanced systems programming, our hands-on workshops accelerate Rust adoption with real-world projects and best practices."
                        cta_text="Learn More"
                        cta_link="/training"
                        icon_color="blue"
                    />
                    <ExpertiseCard
                        title="Rust Consulting & Team Augmentation"
                        description="Strategic consulting and expert developers to supplement your team. We provide architectural guidance, code reviews, and embedded expertise to ensure successful Rust implementation and project delivery."
                        cta_text="Get Consulting"
                        cta_link="/consulting"
                        icon_color="purple"
                    />
                    <ExpertiseCard
                        title="Software Development"
                        description="End-to-end software development services leveraging Rust's performance and safety. From high-performance backend systems to blockchain infrastructure, we build scalable solutions that meet enterprise demands."
                        cta_text="Start Project"
                        cta_link="/development"
                        icon_color="emerald"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ExpertiseCard(
    title: &'static str,
    description: &'static str,
    cta_text: &'static str,
    cta_link: &'static str,
    icon_color: &'static str,
) -> impl IntoView {
    let icon_classes = match icon_color {
        "blue" => "bg-blue-100 text-blue-600",
        "purple" => "bg-purple-100 text-purple-600", 
        "emerald" => "bg-emerald-100 text-emerald-600",
        _ => "bg-gray-100 text-gray-600",
    };
    
    let cta_classes = match icon_color {
        "blue" => "text-blue-600 hover:text-blue-700 border-blue-200 hover:border-blue-300 hover:bg-blue-50",
        "purple" => "text-purple-600 hover:text-purple-700 border-purple-200 hover:border-purple-300 hover:bg-purple-50",
        "emerald" => "text-emerald-600 hover:text-emerald-700 border-emerald-200 hover:border-emerald-300 hover:bg-emerald-50",
        _ => "text-gray-600 hover:text-gray-700 border-gray-200 hover:border-gray-300 hover:bg-gray-50",
    };

    view! {
        <div class="relative group">
            <div class="bg-white rounded-xl shadow-lg border border-gray-200 p-8 h-full hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1">
                {/* Icon */}
                <div class=format!("w-16 h-16 rounded-lg {icon_classes} flex items-center justify-center mb-6")>
                    {match icon_color {
                        "blue" => view! {
                            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                            </svg>
                        }.into_view(),
                        "purple" => view! {
                            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
                            </svg>
                        }.into_view(),
                        "emerald" => view! {
                            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"></path>
                            </svg>
                        }.into_view(),
                        _ => view! {
                            <div class="w-8 h-8 bg-gray-400 rounded"></div>
                        }.into_view(),
                    }}
                </div>
                
                {/* Content */}
                <h3 class="text-xl font-bold text-gray-900 mb-4">{title}</h3>
                <p class="text-gray-600 mb-6 leading-relaxed">{description}</p>
                
                {/* CTA */}
                <div class="mt-auto">
                    <a 
                        href={cta_link}
                        class=format!("inline-flex items-center px-4 py-2 border rounded-lg text-sm font-semibold transition-all duration-200 {cta_classes}")
                    >
                        {cta_text}
                        <svg class="ml-2 w-4 h-4 transition-transform duration-200 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                        </svg>
                    </a>
                </div>
            </div>
        </div>
    }
}