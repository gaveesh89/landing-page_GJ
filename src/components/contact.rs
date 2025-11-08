use leptos::*;
use leptos::ev::SubmitEvent;

#[component]
pub fn ContactSection() -> impl IntoView {
    let (submitted, set_submitted) = create_signal(false);
    let (entity_name, set_entity_name) = create_signal(String::new());
    let (role_title, set_role_title) = create_signal(String::new());
    let (contact_email, set_contact_email) = create_signal(String::new());
    let (project_scope, set_project_scope) = create_signal(String::new());
    let (budget_allocation, set_budget_allocation) = create_signal(String::new());
    let (timeline, set_timeline) = create_signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_submitted.set(true);
    };

    view! {
        <section id="contact" class="py-20 bg-gray-50">
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-16">
                    <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl mb-4">
                        "ENGAGEMENT INITIATION."
                    </h2>
                    <p class="text-xl font-semibold text-gray-800">
                        "SUBMIT PROJECT PARAMETERS FOR ARCHITECTURAL REVIEW."
                    </p>
                </div>
                
                <div class="bg-white rounded-lg shadow-lg p-8">
                    {move || if submitted.get() {
                        view! {
                            <div class="text-center py-16">
                                <h3 class="text-2xl font-bold text-gray-900">
                                    "PARAMETERS RECEIVED. ARCHITECTURAL REVIEW INITIATED."
                                </h3>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <form on:submit=on_submit class="space-y-8">
                                <div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
                                    <div>
                                        <label for="entity_name" class="block text-sm font-bold text-gray-900 mb-2">
                                            "ENTITY NAME"
                                        </label>
                                        <input
                                            type="text"
                                            id="entity_name"
                                            required
                                            placeholder="ACME Corp / Project Chimera"
                                            class="w-full px-4 py-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-gray-900 focus:border-gray-900 text-gray-900"
                                            on:input=move |ev| set_entity_name.set(event_target_value(&ev))
                                        />
                                    </div>
                                    
                                    <div>
                                        <label for="role_title" class="block text-sm font-bold text-gray-900 mb-2">
                                            "ROLE / TITLE"
                                        </label>
                                        <input
                                            type="text"
                                            id="role_title"
                                            required
                                            placeholder="VP of Systems Engineering"
                                            class="w-full px-4 py-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-gray-900 focus:border-gray-900 text-gray-900"
                                            on:input=move |ev| set_role_title.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                                
                                <div>
                                    <label for="contact_email" class="block text-sm font-bold text-gray-900 mb-2">
                                        "CONTACT EMAIL"
                                    </label>
                                    <input
                                        type="email"
                                        id="contact_email"
                                        required
                                        placeholder="name@domain.com"
                                        class="w-full px-4 py-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-gray-900 focus:border-gray-900 text-gray-900"
                                        on:input=move |ev| set_contact_email.set(event_target_value(&ev))
                                    />
                                </div>
                                
                                <div>
                                    <label for="project_scope" class="block text-sm font-bold text-gray-900 mb-2">
                                        "PROJECT SCOPE"
                                    </label>
                                    <textarea
                                        id="project_scope"
                                        required
                                        rows="4"
                                        placeholder="Mandate: Migrate C++ core logic to Rust. Target: Embedded Linux, ARM architecture. Constraint: 6-month timeline."
                                        class="w-full px-4 py-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-gray-900 focus:border-gray-900 text-gray-900"
                                        on:input=move |ev| set_project_scope.set(event_target_value(&ev))
                                    ></textarea>
                                </div>
                                
                                <div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
                                    <div>
                                        <label for="budget_allocation" class="block text-sm font-bold text-gray-900 mb-2">
                                            "BUDGET ALLOCATION"
                                        </label>
                                        <select
                                            id="budget_allocation"
                                            required
                                            class="w-full px-4 py-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-gray-900 focus:border-gray-900 text-gray-900"
                                            on:change=move |ev| set_budget_allocation.set(event_target_value(&ev))
                                        >
                                            <option value="">Select Budget Range</option>
                                            <option value="under-50k">"< $50K"</option>
                                            <option value="50k-150k">"$50K - $150K"</option>
                                            <option value="150k-300k">"$150K - $300K"</option>
                                            <option value="over-300k">"> $300K"</option>
                                        </select>
                                    </div>
                                    
                                    <div>
                                        <label for="timeline" class="block text-sm font-bold text-gray-900 mb-2">
                                            "TIMELINE"
                                        </label>
                                        <select
                                            id="timeline"
                                            required
                                            class="w-full px-4 py-3 border border-gray-300 rounded-md focus:ring-2 focus:ring-gray-900 focus:border-gray-900 text-gray-900"
                                            on:change=move |ev| set_timeline.set(event_target_value(&ev))
                                        >
                                            <option value="">Select Timeline</option>
                                            <option value="immediate">"Immediate (1-4 Weeks)"</option>
                                            <option value="short-term">"Short-Term (1-3 Months)"</option>
                                            <option value="mid-term">"Mid-Term (3-6 Months)"</option>
                                            <option value="long-term">"Long-Term (> 6 Months)"</option>
                                        </select>
                                    </div>
                                </div>
                                
                                <div class="text-center pt-8">
                                    <button
                                        type="submit"
                                        class="inline-flex items-center px-12 py-4 border border-transparent text-base font-bold rounded-md text-white bg-gray-900 hover:bg-gray-800 transition-colors focus:ring-2 focus:ring-gray-900 focus:ring-offset-2"
                                    >
                                        "INITIATE REVIEW"
                                    </button>
                                </div>
                            </form>
                        }.into_view()
                    }}
                </div>
            </div>
        </section>
    }
}