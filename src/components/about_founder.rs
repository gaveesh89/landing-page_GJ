use leptos::*;

#[component]
pub fn AboutFounderSection() -> impl IntoView {
    view! {
        <section id="founder" class="py-20 bg-gradient-to-br from-gray-900 via-slate-800 to-gray-900 relative overflow-hidden">
            {/* Background Elements */}
            <div class="absolute inset-0 bg-gradient-to-br from-blue-600/5 via-purple-600/5 to-cyan-500/5"></div>
            <div class="absolute top-20 left-10 w-32 h-32 bg-blue-500/10 rounded-full blur-xl"></div>
            <div class="absolute bottom-20 right-10 w-24 h-24 bg-purple-500/10 rounded-full blur-xl"></div>
            
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    {/* Content Column */}
                    <div class="order-2 lg:order-1">
                        <div class="mb-6">
                            <span class="inline-flex items-center rounded-full bg-blue-900/50 border border-blue-700/50 px-4 py-2 text-sm font-medium text-blue-300">
                                "Founder Story"
                            </span>
                        </div>
                        
                        <h2 class="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6 leading-tight">
                            "Built by a Systems Programming "
                            <span class="bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                                "Pioneer"
                            </span>
                        </h2>
                        
                        <div class="space-y-4 text-gray-300 text-base sm:text-lg leading-relaxed">
                            <p>
                                "Atomic Increment was founded by Andy Thomason, a systems programming expert with over 30 years of experience architecting game engines, compilers, and blockchain infrastructure. What started as a passion for teaching Rust at Oxford has grown into a consultancy trusted by enterprises to build high-performance, memory-safe solutions that scale."
                            </p>
                            
                            <p>
                                "From leading the UK's first Skillset-accredited MSc Games Development program to pioneering blockchain node development, Andy brings rare depth: someone who built production systems before Rust existed—and now teaches the world why Rust solves problems they didn't know they had."
                            </p>
                        </div>
                        
                        {/* Key Achievements */}
                        <div class="mt-8 grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="flex items-center space-x-3">
                                <div class="w-2 h-2 bg-blue-400 rounded-full"></div>
                                <span class="text-gray-300 text-sm font-medium">"30+ Years Systems Programming"</span>
                            </div>
                            <div class="flex items-center space-x-3">
                                <div class="w-2 h-2 bg-purple-400 rounded-full"></div>
                                <span class="text-gray-300 text-sm font-medium">"Oxford University Instructor"</span>
                            </div>
                            <div class="flex items-center space-x-3">
                                <div class="w-2 h-2 bg-cyan-400 rounded-full"></div>
                                <span class="text-gray-300 text-sm font-medium">"MSc Games Development Pioneer"</span>
                            </div>
                            <div class="flex items-center space-x-3">
                                <div class="w-2 h-2 bg-emerald-400 rounded-full"></div>
                                <span class="text-gray-300 text-sm font-medium">"Blockchain Infrastructure Expert"</span>
                            </div>
                        </div>
                        
                        {/* CTA */}
                        <div class="mt-10">
                            <a 
                                href="/about"
                                class="group inline-flex items-center px-6 py-3 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 text-white font-semibold rounded-lg shadow-lg hover:shadow-xl transition-all duration-300 transform hover:scale-105"
                            >
                                "Learn Our Story"
                                <svg class="ml-2 w-4 h-4 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                </svg>
                            </a>
                        </div>
                    </div>
                    
                    {/* Visual Column */}
                    <div class="order-1 lg:order-2">
                        <div class="relative">
                            {/* Main Profile Card */}
                            <div class="bg-gradient-to-br from-slate-800/80 to-gray-900/80 backdrop-blur-sm rounded-2xl p-8 border border-gray-700/50 shadow-2xl">
                                {/* Profile Header */}
                                <div class="text-center mb-6">
                                    <div class="w-24 h-24 mx-auto mb-4 bg-gradient-to-br from-blue-500 to-purple-600 rounded-full flex items-center justify-center shadow-lg">
                                        <span class="text-3xl font-bold text-white">"AT"</span>
                                    </div>
                                    <h3 class="text-xl font-bold text-white">"Andy Thomason"</h3>
                                    <p class="text-blue-400 font-medium">"Founder & Principal Engineer"</p>
                                </div>
                                
                                {/* Stats Grid */}
                                <div class="grid grid-cols-2 gap-4 mb-6">
                                    <div class="text-center p-4 bg-gray-800/50 rounded-lg border border-gray-700/30">
                                        <div class="text-2xl font-bold text-blue-400">"30+"</div>
                                        <div class="text-xs text-gray-400">"Years Experience"</div>
                                    </div>
                                    <div class="text-center p-4 bg-gray-800/50 rounded-lg border border-gray-700/30">
                                        <div class="text-2xl font-bold text-purple-400">"1000+"</div>
                                        <div class="text-xs text-gray-400">"Students Taught"</div>
                                    </div>
                                </div>
                                
                                {/* Expertise Tags */}
                                <div class="flex flex-wrap gap-2 justify-center">
                                    <span class="px-3 py-1 bg-blue-900/50 text-blue-300 text-xs font-medium rounded-full border border-blue-700/50">
                                        "Game Engines"
                                    </span>
                                    <span class="px-3 py-1 bg-purple-900/50 text-purple-300 text-xs font-medium rounded-full border border-purple-700/50">
                                        "Compilers"
                                    </span>
                                    <span class="px-3 py-1 bg-cyan-900/50 text-cyan-300 text-xs font-medium rounded-full border border-cyan-700/50">
                                        "Blockchain"
                                    </span>
                                    <span class="px-3 py-1 bg-emerald-900/50 text-emerald-300 text-xs font-medium rounded-full border border-emerald-700/50">
                                        "Systems Programming"
                                    </span>
                                </div>
                            </div>
                            
                            {/* Floating Elements */}
                            <div class="absolute -top-4 -right-4 w-16 h-16 bg-gradient-to-br from-blue-500/20 to-purple-500/20 rounded-full border border-blue-400/30 flex items-center justify-center animate-pulse">
                                <svg class="w-8 h-8 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                                </svg>
                            </div>
                            
                            <div class="absolute -bottom-4 -left-4 w-12 h-12 bg-gradient-to-br from-purple-500/20 to-cyan-500/20 rounded-full border border-purple-400/30 flex items-center justify-center animate-pulse" style="animation-delay: 1s;">
                                <svg class="w-6 h-6 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"></path>
                                </svg>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}