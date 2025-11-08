use leptos::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="relative isolate overflow-hidden bg-gray-900 min-h-screen flex items-center">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-16 sm:py-20 lg:py-24">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 items-center">
                    {/* Content Column */}
                    <div class="order-2 lg:order-1 text-center lg:text-left">
                        <div class="mb-6 flex flex-wrap justify-center lg:justify-start gap-2">
                            <span class="inline-flex items-center rounded-full bg-gray-800 px-3 py-1 text-xs font-medium text-gray-300 border border-gray-700">
                                "Rust"
                            </span>
                            <span class="inline-flex items-center rounded-full bg-gray-800 px-3 py-1 text-xs font-medium text-gray-300 border border-gray-700">
                                "Blockchain"
                            </span>
                            <span class="inline-flex items-center rounded-full bg-gray-800 px-3 py-1 text-xs font-medium text-gray-300 border border-gray-700">
                                "Innovation"
                            </span>
                        </div>
                        
                        <h1 class="text-3xl sm:text-4xl lg:text-5xl xl:text-6xl font-bold tracking-tight text-white leading-tight">
                            "Build Software That Scales at the Speed of Innovation"
                        </h1>
                        
                        <p class="mt-6 text-base sm:text-lg lg:text-xl leading-relaxed text-gray-300 max-w-2xl mx-auto lg:mx-0">
                            "Empowering businesses with next-generation Rust solutions—high-performance applications, secure blockchain infrastructure, and AI-powered data analytics that transform complexity into competitive advantage."
                        </p>
                        
                        <div class="mt-8 flex flex-col sm:flex-row justify-center lg:justify-start items-center gap-4">
                            <a
                                href="/engagement"
                                class="w-full sm:w-auto inline-flex items-center justify-center rounded-lg bg-blue-600 px-8 py-4 text-base font-semibold text-white shadow-lg hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all duration-200 transform hover:scale-105"
                                aria-label="Start your project consultation"
                            >
                                "Start Your Project"
                            </a>
                            <a 
                                href="/about" 
                                class="w-full sm:w-auto inline-flex items-center justify-center text-base font-semibold leading-6 text-gray-300 hover:text-white transition-colors duration-200 py-4 px-8"
                                aria-label="Learn more about our services"
                            >
                                "Learn More"
                                <span aria-hidden="true" class="ml-2 transition-transform duration-200 group-hover:translate-x-1">"→"</span>
                            </a>
                        </div>
                        
                        {/* Social proof indicators */}
                        <div class="mt-12 pt-8 border-t border-gray-800">
                            <div class="flex flex-wrap justify-center lg:justify-start items-center gap-8 text-gray-400">
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                                    <span class="text-sm font-medium">"Enterprise Ready"</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-blue-400 rounded-full"></div>
                                    <span class="text-sm font-medium">"Memory Safe"</span>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 bg-purple-400 rounded-full"></div>
                                    <span class="text-sm font-medium">"Zero Downtime"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    {/* Visual Column */}
                    <div class="order-1 lg:order-2">
                        <div class="relative aspect-square max-w-lg mx-auto lg:max-w-none">
                            {/* Abstract Tech Visualization */}
                            <div class="hero-tech-visualization h-full w-full rounded-2xl bg-gradient-to-br from-slate-900 via-gray-900 to-black shadow-2xl ring-1 ring-slate-700 overflow-hidden relative">
                                {/* Background gradient mesh */}
                                <div class="absolute inset-0 bg-gradient-to-br from-blue-600/20 via-purple-600/10 to-cyan-500/15"></div>
                                
                                {/* Animated gradient overlay */}
                                <div class="absolute inset-0 bg-gradient-to-r from-transparent via-blue-500/5 to-transparent animate-pulse"></div>
                                
                                {/* Network nodes - Large */}
                                <div class="absolute top-12 left-16 w-4 h-4 bg-blue-400 rounded-full shadow-lg shadow-blue-400/50 floating-particle" style="animation-delay: 0s;"></div>
                                <div class="absolute top-24 right-20 w-3 h-3 bg-cyan-400 rounded-full shadow-lg shadow-cyan-400/50 floating-particle" style="animation-delay: 0.5s;"></div>
                                <div class="absolute top-32 left-1/3 w-5 h-5 bg-purple-400 rounded-full shadow-lg shadow-purple-400/50 floating-particle" style="animation-delay: 1s;"></div>
                                <div class="absolute bottom-20 right-16 w-4 h-4 bg-emerald-400 rounded-full shadow-lg shadow-emerald-400/50 floating-particle" style="animation-delay: 1.5s;"></div>
                                <div class="absolute bottom-32 left-20 w-3 h-3 bg-blue-300 rounded-full shadow-lg shadow-blue-300/50 floating-particle" style="animation-delay: 2s;"></div>
                                
                                {/* Network nodes - Medium */}
                                <div class="absolute top-20 left-1/2 w-2 h-2 bg-cyan-300 rounded-full shadow-lg shadow-cyan-300/50 animate-ping" style="animation-delay: 0.3s;"></div>
                                <div class="absolute top-40 right-1/3 w-2 h-2 bg-purple-300 rounded-full shadow-lg shadow-purple-300/50 animate-ping" style="animation-delay: 0.8s;"></div>
                                <div class="absolute bottom-24 left-1/4 w-2 h-2 bg-emerald-300 rounded-full shadow-lg shadow-emerald-300/50 animate-ping" style="animation-delay: 1.3s;"></div>
                                
                                {/* Connection lines */}
                                <svg class="absolute inset-0 w-full h-full opacity-30" viewBox="0 0 400 300">
                                    <defs>
                                        <linearGradient id="lineGradient1" x1="0%" y1="0%" x2="100%" y2="100%">
                                            <stop offset="0%" style="stop-color:#3b82f6;stop-opacity:0.8" />
                                            <stop offset="100%" style="stop-color:#06b6d4;stop-opacity:0.2" />
                                        </linearGradient>
                                        <linearGradient id="lineGradient2" x1="0%" y1="0%" x2="100%" y2="100%">
                                            <stop offset="0%" style="stop-color:#8b5cf6;stop-opacity:0.8" />
                                            <stop offset="100%" style="stop-color:#10b981;stop-opacity:0.2" />
                                        </linearGradient>
                                    </defs>
                                    <path d="M64 48 L320 96 L240 128 L80 160" stroke="url(#lineGradient1)" stroke-width="1" fill="none" class="network-line" />
                                    <path d="M133 128 L266 60 L320 240 L80 192" stroke="url(#lineGradient2)" stroke-width="1" fill="none" class="network-line" style="animation-delay: 1s;" />
                                    <path d="M64 240 L200 100 L350 180" stroke="url(#lineGradient1)" stroke-width="1" fill="none" class="network-line" style="animation-delay: 2s;" />
                                </svg>
                                
                                {/* Floating geometric elements */}
                                <div class="absolute top-16 right-24 transform rotate-12">
                                    <div class="w-8 h-8 border border-blue-400/40 rotate-45 animate-spin" style="animation-duration: 8s;"></div>
                                </div>
                                <div class="absolute bottom-28 left-32 transform -rotate-12">
                                    <div class="w-6 h-6 border border-purple-400/40 rotate-45 animate-spin" style="animation-duration: 6s; animation-direction: reverse;"></div>
                                </div>
                                <div class="absolute top-1/2 right-1/3 transform rotate-45">
                                    <div class="w-4 h-4 border border-cyan-400/40 animate-spin" style="animation-duration: 4s;"></div>
                                </div>
                                
                                {/* Hexagonal elements */}
                                <div class="absolute top-8 left-1/2 transform -translate-x-1/2">
                                    <div class="hexagon w-6 h-6 bg-gradient-to-br from-blue-500/20 to-purple-500/20 border border-blue-400/30"></div>
                                </div>
                                <div class="absolute bottom-12 right-1/4">
                                    <div class="hexagon w-8 h-8 bg-gradient-to-br from-emerald-500/20 to-cyan-500/20 border border-emerald-400/30"></div>
                                </div>
                                
                                {/* Data flow streams */}
                                <div class="absolute top-1/4 left-0 w-full h-px bg-gradient-to-r from-transparent via-blue-400/50 to-transparent transform -rotate-12 animate-pulse"></div>
                                <div class="absolute bottom-1/4 left-0 w-full h-px bg-gradient-to-r from-transparent via-purple-400/50 to-transparent transform rotate-12 animate-pulse" style="animation-delay: 1s;"></div>
                                
                                {/* Central focal point */}
                                <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
                                    <div class="w-12 h-12 rounded-full bg-gradient-to-br from-blue-500/30 to-purple-600/30 border-2 border-blue-400/50 flex items-center justify-center animate-pulse">
                                        <div class="w-6 h-6 rounded-full bg-gradient-to-br from-cyan-400 to-blue-600 shadow-lg shadow-blue-500/50"></div>
                                    </div>
                                </div>
                                
                                {/* Particle effects */}
                                <div class="absolute top-1/3 left-1/4 w-1 h-1 bg-blue-400 rounded-full animate-ping" style="animation-delay: 0.5s;"></div>
                                <div class="absolute top-2/3 right-1/3 w-1 h-1 bg-purple-400 rounded-full animate-ping" style="animation-delay: 1.5s;"></div>
                                <div class="absolute bottom-1/3 left-2/3 w-1 h-1 bg-cyan-400 rounded-full animate-ping" style="animation-delay: 2.5s;"></div>
                                
                                {/* Tech performance metrics overlay */}
                                <div class="absolute bottom-4 left-4 right-4">
                                    <div class="bg-black/40 backdrop-blur-sm rounded-lg p-3 border border-gray-700/50">
                                        <div class="grid grid-cols-3 gap-2 text-xs text-slate-300 font-mono">
                                            <div class="text-center">
                                                <div class="text-green-400 font-bold">"99.9%"</div>
                                                <div class="text-gray-400">"Uptime"</div>
                                            </div>
                                            <div class="text-center">
                                                <div class="text-blue-400 font-bold">"<1ms"</div>
                                                <div class="text-gray-400">"Latency"</div>
                                            </div>
                                            <div class="text-center">
                                                <div class="text-purple-400 font-bold">"0x"</div>
                                                <div class="text-gray-400">"Memory Leaks"</div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            {/* Background gradient overlay */}
            <div class="absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]" aria-hidden="true">
                <div class="relative left-[calc(50%+3rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 bg-gradient-to-tr from-blue-600 to-purple-600 opacity-20 sm:left-[calc(50%+36rem)] sm:w-[72.1875rem]"></div>
            </div>
            
            {/* Scroll indicator */}
            <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 hidden lg:block">
                <div class="flex flex-col items-center text-gray-400 animate-bounce">
                    <span class="text-xs font-medium mb-2">"Scroll to explore"</span>
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                    </svg>
                </div>
            </div>
        </section>
    }
}
