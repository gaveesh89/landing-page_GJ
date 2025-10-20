use leptos::*;

#[component]
pub fn ContactSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl">
                        "Ready to Get Started?"
                    </h2>
                    <p class="mt-4 text-xl text-gray-600">
                        "Let's discuss how we can help accelerate your project"
                    </p>
                </div>
                
                <div class="mt-16">
                    <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
                        <ContactCard
                            icon="📧"
                            title="Email Us"
                            description="Get in touch for project inquiries and consultations"
                            action="info@atomicincrement.com"
                        />
                        <ContactCard
                            icon="📞"
                            title="Call Us"
                            description="Speak directly with our technical experts"
                            action="+1 (555) 123-4567"
                        />
                        <ContactCard
                            icon="📅"
                            title="Book Consultation"
                            description="Schedule a free technical consultation"
                            action="Book Now"
                        />
                    </div>
                </div>
                
                <div class="mt-16 text-center">
                    <a href="/consultation" class="inline-flex items-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
                        "Start Your Project Today"
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ContactCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    action: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md p-6 text-center">
            <div class="text-4xl mb-4">{icon}</div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
            <p class="text-gray-600 mb-4">{description}</p>
            <div class="text-blue-600 font-medium">{action}</div>
        </div>
    }
}