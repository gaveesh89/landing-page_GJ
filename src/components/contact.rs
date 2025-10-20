use leptos::*;

#[component]
pub fn ContactSection() -> impl IntoView {
    view! {
        <section id="contact" class="py-20 bg-gray-50">
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
                            action="andy@atomicincrement.com"
                            link="mailto:andy@atomicincrement.com"
                        />
                        // <ContactCard
                        //     icon="📍"
                        //     title="Visit Us"
                        //     description="Our office location for in-person meetings"
                        //     action="35 Bourne Road, Pangbourne RG8 7DK, UK"
                        //     link=""
                        // />
                        // <ContactCard
                        //     icon="📅"
                        //     title="Book Consultation"
                        //     description="Schedule a free technical consultation"
                        //     action="Book Now"
                        //     link="/consultation"
                        // />
                    </div>
                </div>
                
                <div class="mt-16 text-center">
                    <a href="#contact" class="inline-flex items-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
                        "Get In Touch"
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
    link: &'static str,
) -> impl IntoView {
    let is_link = !link.is_empty();
    
    view! {
        <div class="bg-white rounded-lg shadow-md p-6 text-center">
            <div class="text-4xl mb-4">{icon}</div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
            <p class="text-gray-600 mb-4">{description}</p>
            <div class="text-blue-600 font-medium">
                {if is_link {
                    view! {
                        <a href={link} class="hover:text-blue-800 transition-colors">
                            {action}
                        </a>
                    }.into_view()
                } else {
                    view! {
                        <span>{action}</span>
                    }.into_view()
                }}
            </div>
        </div>
    }
}