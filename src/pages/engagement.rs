use leptos::*;

#[component]
pub fn EngagementPage() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (company, set_company) = create_signal(String::new());
    let (project_type, set_project_type) = create_signal(String::new());
    let (timeline, set_timeline) = create_signal(String::new());
    let (budget, set_budget) = create_signal(String::new());
    let (requirements, set_requirements) = create_signal(String::new());
    let (technical_stack, set_technical_stack) = create_signal(String::new());
    let (current_challenges, set_current_challenges) = create_signal(String::new());
    let (success_metrics, set_success_metrics) = create_signal(String::new());

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // Here you would typically send the data to your backend
        web_sys::console::log_1(&"Form submitted".into());
    };

    view! {
        <div class="min-h-screen bg-gray-50 py-12">
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
                {/* Header */}
                <div class="text-center mb-12">
                    <h1 class="text-4xl font-bold text-gray-900 mb-4">
                        "ENGAGEMENT INITIATION"
                    </h1>
                    <p class="text-xl text-gray-600 mb-2">
                        "SUBMIT PROJECT PARAMETERS FOR ARCHITECTURAL REVIEW"
                    </p>
                    <div class="w-24 h-1 bg-gradient-to-r from-blue-600 to-purple-600 mx-auto"></div>
                </div>

                {/* Form */}
                <div class="bg-white rounded-2xl shadow-xl border border-gray-100 overflow-hidden">
                    <div class="bg-gradient-to-r from-blue-600 to-purple-600 px-8 py-6">
                        <h2 class="text-2xl font-bold text-white">
                            "Project Parameters & Technical Requirements"
                        </h2>
                        <p class="text-blue-100 mt-2">
                            "Provide detailed information for comprehensive architectural assessment"
                        </p>
                    </div>

                    <form on:submit=handle_submit class="p-8 space-y-8">
                        {/* Contact Information */}
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <div>
                                <label for="name" class="block text-sm font-semibold text-gray-700 mb-2">
                                    "Full Name *"
                                </label>
                                <input
                                    type="text"
                                    id="name"
                                    required
                                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                    placeholder="Enter your full name"
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    prop:value=name
                                />
                            </div>
                            <div>
                                <label for="email" class="block text-sm font-semibold text-gray-700 mb-2">
                                    "Email Address *"
                                </label>
                                <input
                                    type="email"
                                    id="email"
                                    required
                                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                    placeholder="Enter your email address"
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    prop:value=email
                                />
                            </div>
                        </div>

                        {/* Company Information */}
                        <div>
                            <label for="company" class="block text-sm font-semibold text-gray-700 mb-2">
                                "Company/Organization *"
                            </label>
                            <input
                                type="text"
                                id="company"
                                required
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                placeholder="Enter your company name"
                                on:input=move |ev| set_company.set(event_target_value(&ev))
                                prop:value=company
                            />
                        </div>

                        {/* Project Type */}
                        <div>
                            <label for="project_type" class="block text-sm font-semibold text-gray-700 mb-2">
                                "Project Type *"
                            </label>
                            <select
                                id="project_type"
                                required
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                on:input=move |ev| set_project_type.set(event_target_value(&ev))
                                prop:value=project_type
                            >
                                <option value="">"Select project type"</option>
                                <option value="web_application">"Web Application Development"</option>
                                <option value="backend_systems">"Backend Systems & APIs"</option>
                                <option value="blockchain">"Blockchain Infrastructure"</option>
                                <option value="data_analytics">"Data Analytics & Processing"</option>
                                <option value="system_architecture">"System Architecture Review"</option>
                                <option value="performance_optimization">"Performance Optimization"</option>
                                <option value="rust_training">"Rust Training & Consulting"</option>
                                <option value="team_augmentation">"Team Augmentation"</option>
                                <option value="other">"Other (specify in requirements)"</option>
                            </select>
                        </div>

                        {/* Timeline and Budget */}
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <div>
                                <label for="timeline" class="block text-sm font-semibold text-gray-700 mb-2">
                                    "Project Timeline *"
                                </label>
                                <select
                                    id="timeline"
                                    required
                                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                    on:input=move |ev| set_timeline.set(event_target_value(&ev))
                                    prop:value=timeline
                                >
                                    <option value="">"Select timeline"</option>
                                    <option value="immediate">"Immediate (< 1 month)"</option>
                                    <option value="short_term">"Short-term (1-3 months)"</option>
                                    <option value="medium_term">"Medium-term (3-6 months)"</option>
                                    <option value="long_term">"Long-term (6+ months)"</option>
                                    <option value="ongoing">"Ongoing support"</option>
                                </select>
                            </div>
                            <div>
                                <label for="budget" class="block text-sm font-semibold text-gray-700 mb-2">
                                    "Budget Range *"
                                </label>
                                <select
                                    id="budget"
                                    required
                                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                    on:input=move |ev| set_budget.set(event_target_value(&ev))
                                    prop:value=budget
                                >
                                    <option value="">"Select budget range"</option>
                                    <option value="under_25k">"Under $25,000"</option>
                                    <option value="25k_50k">"$25,000 - $50,000"</option>
                                    <option value="50k_100k">"$50,000 - $100,000"</option>
                                    <option value="100k_250k">"$100,000 - $250,000"</option>
                                    <option value="250k_plus">"$250,000+"</option>
                                    <option value="discuss">"Prefer to discuss"</option>
                                </select>
                            </div>
                        </div>

                        {/* Technical Requirements */}
                        <div>
                            <label for="requirements" class="block text-sm font-semibold text-gray-700 mb-2">
                                "Project Requirements & Objectives *"
                            </label>
                            <textarea
                                id="requirements"
                                required
                                rows="5"
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 resize-vertical"
                                placeholder="Describe your project requirements, objectives, and expected outcomes in detail..."
                                on:input=move |ev| set_requirements.set(event_target_value(&ev))
                                prop:value=requirements
                            ></textarea>
                        </div>

                        {/* Technical Stack */}
                        <div>
                            <label for="technical_stack" class="block text-sm font-semibold text-gray-700 mb-2">
                                "Current Technical Stack & Infrastructure"
                            </label>
                            <textarea
                                id="technical_stack"
                                rows="4"
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 resize-vertical"
                                placeholder="List your current technologies, frameworks, databases, cloud providers, etc..."
                                on:input=move |ev| set_technical_stack.set(event_target_value(&ev))
                                prop:value=technical_stack
                            ></textarea>
                        </div>

                        {/* Current Challenges */}
                        <div>
                            <label for="current_challenges" class="block text-sm font-semibold text-gray-700 mb-2">
                                "Current Challenges & Pain Points"
                            </label>
                            <textarea
                                id="current_challenges"
                                rows="4"
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 resize-vertical"
                                placeholder="Describe performance issues, scalability challenges, technical debt, or other concerns..."
                                on:input=move |ev| set_current_challenges.set(event_target_value(&ev))
                                prop:value=current_challenges
                            ></textarea>
                        </div>

                        {/* Success Metrics */}
                        <div>
                            <label for="success_metrics" class="block text-sm font-semibold text-gray-700 mb-2">
                                "Success Metrics & KPIs"
                            </label>
                            <textarea
                                id="success_metrics"
                                rows="3"
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 resize-vertical"
                                placeholder="How will you measure project success? (performance improvements, user metrics, cost savings, etc.)"
                                on:input=move |ev| set_success_metrics.set(event_target_value(&ev))
                                prop:value=success_metrics
                            ></textarea>
                        </div>

                        {/* Information Notice */}
                        <div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
                            <div class="flex items-start">
                                <svg class="w-6 h-6 text-blue-600 mt-0.5 mr-3 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                <div>
                                    <h3 class="text-sm font-semibold text-blue-900 mb-2">
                                        "What happens next?"
                                    </h3>
                                    <ul class="text-sm text-blue-800 space-y-1">
                                        <li>"• Comprehensive architectural review within 48 hours"</li>
                                        <li>"• Technical feasibility assessment and recommendations"</li>
                                        <li>"• Detailed project proposal with timeline and milestones"</li>
                                        <li>"• Follow-up consultation call to discuss approach"</li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        {/* Submit Button */}
                        <div class="flex justify-center pt-6">
                            <button
                                type="submit"
                                class="bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold py-4 px-12 rounded-lg hover:from-blue-700 hover:to-purple-700 focus:outline-none focus:ring-4 focus:ring-blue-300 transition-all duration-200 transform hover:scale-105 shadow-lg"
                            >
                                "SUBMIT FOR ARCHITECTURAL REVIEW"
                            </button>
                        </div>
                    </form>
                </div>

                {/* Contact Information */}
                <div class="mt-12 text-center">
                    <p class="text-gray-600 mb-4">
                        "Prefer to discuss your project directly?"
                    </p>
                    <div class="flex flex-col sm:flex-row justify-center items-center gap-4 text-sm text-gray-500">
                        <a href="mailto:hello@atomicincrement.com" class="flex items-center hover:text-blue-600 transition-colors">
                            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 7.89a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
                            </svg>
                            "hello@atomicincrement.com"
                        </a>
                        <span class="hidden sm:inline text-gray-300">"|"</span>
                        <a href="tel:+1234567890" class="flex items-center hover:text-blue-600 transition-colors">
                            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"></path>
                            </svg>
                            "Schedule a consultation call"
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}