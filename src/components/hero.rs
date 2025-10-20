use leptos::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section id="home" class="relative bg-white overflow-hidden">
            <div class="max-w-7xl mx-auto">
                <div class="relative z-10 pb-8 bg-white sm:pb-16 md:pb-20 lg:max-w-2xl lg:w-full lg:pb-28 xl:pb-32">
                    <svg class="hidden lg:block absolute right-0 inset-y-0 h-full w-48 text-white transform translate-x-1/2" fill="currentColor" viewBox="0 0 100 100" preserveAspectRatio="none" aria-hidden="true">
                        <polygon points="50,0 100,0 50,100 0,100"/>
                    </svg>

                    <div class="pt-10 mx-auto max-w-7xl px-4 sm:pt-12 sm:px-6 md:pt-16 lg:pt-20 lg:px-8 xl:pt-28">
                        <div class="sm:text-center lg:text-left">
                            <h1 class="text-3xl tracking-tight font-extrabold text-gray-900 sm:text-4xl md:text-5xl">
                                <span class="block xl:inline">"Rust Training"</span>
                                <span class="block xl:inline">"Blockchain building"</span>
                                <span class="block xl:inline">"Data science"</span>
                            </h1>
                            <p class="mt-3 text-base text-gray-500 sm:mt-5 sm:text-lg sm:max-w-xl sm:mx-auto md:mt-5 md:text-xl lg:mx-0">
                                "Cutting edge solutions to data analysis for the modern AI enabled world."
                            </p>
                            <div class="mt-5 sm:mt-8 sm:flex sm:justify-center lg:justify-start">
                                <div class="rounded-md shadow">
                                    <a href="/consultation" class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10 transition-colors">
                                        "Start Your Project"
                                    </a>
                                </div>
                                <div class="mt-3 sm:mt-0 sm:ml-3">
                                    <a href="/about" class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-blue-700 bg-blue-100 hover:bg-blue-200 md:py-4 md:text-lg md:px-10 transition-colors">
                                        "Learn More"
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="lg:absolute lg:inset-y-0 lg:right-0 lg:w-1/2">
                <div class="h-56 w-full bg-gradient-to-r from-blue-400 to-purple-500 sm:h-72 md:h-96 lg:w-full lg:h-full flex items-center justify-center">
                    <div class="text-center text-white">
                        <div class="flex justify-center space-x-8 mb-6">
                            // Rust Ferris Logo
                            <div class="w-20 h-20 bg-white bg-opacity-20 rounded-full flex items-center justify-center p-2">
                                <img src="https://upload.wikimedia.org/wikipedia/commons/0/0f/Original_Ferris.svg" 
                                     alt="Ferris the Rust mascot" 
                                     class="w-full h-full object-contain"/>
                            </div>
                            // Ethereum Logo
                            <div class="w-20 h-20 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
                                <svg class="w-12 h-12" viewBox="0 0 256 417" fill="currentColor">
                                    <path d="M127.961 0l-2.795 9.5v275.668l2.795 2.79 127.962-75.638z"/>
                                    <path d="M127.962 0L0 212.32l127.962 75.639V154.158z"/>
                                    <path d="M127.961 312.187l-1.575 1.92v98.199l1.575 4.6L256 236.587z"/>
                                    <path d="M127.962 416.905v-104.72L0 236.585z"/>
                                    <path d="M127.961 287.958l127.96-75.637-127.96-58.162z"/>
                                    <path d="M0 212.32l127.96 75.638v-133.8z"/>
                                </svg>
                            </div>
                        </div>
                        <p class="text-lg font-medium">"Building the Future"</p>
                        <p class="text-sm mt-2 opacity-90">"Rust • Blockchain • Innovation"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}