use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let (is_mobile_menu_open, set_mobile_menu_open) = create_signal(false);

    view! {
        <header class="sticky top-0 z-50 bg-white shadow-sm border-b">
            <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Logo section
                    <div class="flex-shrink-0 flex items-center">
                        <a href="/" class="flex items-center space-x-2">
                            <div class="w-8 h-8 bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg flex items-center justify-center">
                                <span class="text-white font-bold text-lg">"A"</span>
                            </div>
                            <span class="text-xl font-bold text-gray-900">"AtomicIncrement"</span>
                        </a>
                    </div>

                    // Desktop navigation
                    <div class="hidden md:block">
                        <div class="ml-10 flex items-baseline space-x-8">
                            <a href="/" class="text-gray-900 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">
                                "Home"
                            </a>
                            <a href="#services" class="text-gray-600 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">
                                "Services"
                            </a>
                            <a href="#about" class="text-gray-600 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">
                                "About"
                            </a>
                            <a href="/blog" class="text-gray-600 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">
                                "Blog"
                            </a>
                            <a href="#contact" class="text-gray-600 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">
                                "Contact"
                            </a>
                        </div>
                    </div>

                    // CTA button and mobile menu button
                    <div class="flex items-center space-x-4">
                        <a href="/consultation" class="hidden sm:inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
                            "Get Started"
                        </a>
                        
                        // Mobile menu button
                        <button
                            class="md:hidden inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100"
                            on:click=move |_| set_mobile_menu_open.set(!is_mobile_menu_open.get())
                        >
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                {move || if is_mobile_menu_open.get() {
                                    view! {
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    }
                                } else {
                                    view! {
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                    }
                                }}
                            </svg>
                        </button>
                    </div>
                </div>

                // Mobile menu
                {move || if is_mobile_menu_open.get() {
                    view! {
                        <div class="md:hidden">
                            <div class="px-2 pt-2 pb-3 space-y-1 sm:px-3 bg-white border-t">
                                <a href="/" class="text-gray-900 hover:text-blue-600 block px-3 py-2 text-base font-medium">
                                    "Home"
                                </a>
                                <a href="#services" class="text-gray-600 hover:text-blue-600 block px-3 py-2 text-base font-medium">
                                    "Services"
                                </a>
                                <a href="#about" class="text-gray-600 hover:text-blue-600 block px-3 py-2 text-base font-medium">
                                    "About"
                                </a>
                                <a href="/blog" class="text-gray-600 hover:text-blue-600 block px-3 py-2 text-base font-medium">
                                    "Blog"
                                </a>
                                <a href="#contact" class="text-gray-600 hover:text-blue-600 block px-3 py-2 text-base font-medium">
                                    "Contact"
                                </a>
                                <a href="/consultation" class="block w-full text-left px-3 py-2 text-base font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md mt-4">
                                    "Get Started"
                                </a>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
            </nav>
        </header>
    }
}