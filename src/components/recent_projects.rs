use leptos::*;

#[component]
pub fn RecentProjectsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gradient-to-br from-slate-50 to-blue-50 overflow-hidden">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                // Section header
                <div class="text-center mb-16">
                    <h2 class="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
                        "Recent " <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600">"Projects"</span>
                    </h2>
                    <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                        "Showcasing our latest work in systems programming, performance optimization, and cutting-edge technology solutions."
                    </p>
                </div>

                // Projects carousel container
                <div class="relative">
                    // Projects scroll container
                    <div 
                        id="projects-scroll"
                        class="flex gap-6 overflow-x-auto scroll-smooth pb-4 px-2"
                        style="scrollbar-width: none; -ms-overflow-style: none;"
                    >
                        // Project 1: High-Performance Trading Engine
                        <div class="flex-none w-80 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 group">
                            <div class="p-6">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="w-12 h-12 bg-gradient-to-r from-green-500 to-emerald-600 rounded-xl flex items-center justify-center">
                                        <span class="text-white text-xl font-bold">"$"</span>
                                    </div>
                                    <span class="px-3 py-1 text-xs font-medium bg-green-100 text-green-800 rounded-full">"Financial Tech"</span>
                                </div>
                                <h3 class="text-lg font-bold text-gray-900 mb-3 group-hover:text-blue-600 transition-colors">
                                    "High-Performance Trading Engine"
                                </h3>
                                <p class="text-gray-600 text-sm leading-relaxed mb-4">
                                    "Built ultra-low latency trading system processing 1M+ transactions/second with microsecond precision. 
                                    Implemented custom memory allocators and SIMD optimizations for 40% performance gain."
                                </p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-2 py-1 text-xs bg-orange-100 text-orange-800 rounded">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded">"SIMD"</span>
                                    <span class="px-2 py-1 text-xs bg-purple-100 text-purple-800 rounded">"Low Latency"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-500">"6 months"</span>
                                    <div class="flex items-center text-green-600">
                                        <span class="text-sm font-medium">"40% faster"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Project 2: Blockchain Consensus Protocol
                        <div class="flex-none w-80 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 group">
                            <div class="p-6">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="w-12 h-12 bg-gradient-to-r from-purple-500 to-indigo-600 rounded-xl flex items-center justify-center">
                                        <span class="text-white text-xl font-bold">"⛓"</span>
                                    </div>
                                    <span class="px-3 py-1 text-xs font-medium bg-purple-100 text-purple-800 rounded-full">"Blockchain"</span>
                                </div>
                                <h3 class="text-lg font-bold text-gray-900 mb-3 group-hover:text-blue-600 transition-colors">
                                    "Blockchain Consensus Protocol"
                                </h3>
                                <p class="text-gray-600 text-sm leading-relaxed mb-4">
                                    "Designed novel proof-of-stake consensus mechanism with 99.9% uptime and 10,000 TPS throughput. 
                                    Implemented Byzantine fault tolerance with cryptographic security guarantees."
                                </p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-2 py-1 text-xs bg-orange-100 text-orange-800 rounded">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-green-100 text-green-800 rounded">"Consensus"</span>
                                    <span class="px-2 py-1 text-xs bg-indigo-100 text-indigo-800 rounded">"Cryptography"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-500">"8 months"</span>
                                    <div class="flex items-center text-purple-600">
                                        <span class="text-sm font-medium">"10K TPS"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Project 3: Real-time Game Engine
                        <div class="flex-none w-80 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 group">
                            <div class="p-6">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="w-12 h-12 bg-gradient-to-r from-red-500 to-pink-600 rounded-xl flex items-center justify-center">
                                        <span class="text-white text-xl font-bold">"🎮"</span>
                                    </div>
                                    <span class="px-3 py-1 text-xs font-medium bg-red-100 text-red-800 rounded-full">"Game Engine"</span>
                                </div>
                                <h3 class="text-lg font-bold text-gray-900 mb-3 group-hover:text-blue-600 transition-colors">
                                    "Real-time Game Engine"
                                </h3>
                                <p class="text-gray-600 text-sm leading-relaxed mb-4">
                                    "Developed cross-platform game engine with advanced physics simulation and real-time ray tracing. 
                                    Achieved 144 FPS at 4K resolution with dynamic lighting and particle systems."
                                </p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-2 py-1 text-xs bg-orange-100 text-orange-800 rounded">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-cyan-100 text-cyan-800 rounded">"Graphics"</span>
                                    <span class="px-2 py-1 text-xs bg-pink-100 text-pink-800 rounded">"Real-time"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-500">"12 months"</span>
                                    <div class="flex items-center text-red-600">
                                        <span class="text-sm font-medium">"144 FPS"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Project 4: ML Compiler Optimization
                        <div class="flex-none w-80 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 group">
                            <div class="p-6">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="w-12 h-12 bg-gradient-to-r from-yellow-500 to-orange-600 rounded-xl flex items-center justify-center">
                                        <span class="text-white text-xl font-bold">"🧠"</span>
                                    </div>
                                    <span class="px-3 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded-full">"AI/ML"</span>
                                </div>
                                <h3 class="text-lg font-bold text-gray-900 mb-3 group-hover:text-blue-600 transition-colors">
                                    "ML Compiler Optimization"
                                </h3>
                                <p class="text-gray-600 text-sm leading-relaxed mb-4">
                                    "Created domain-specific compiler for machine learning workloads with automatic vectorization. 
                                    Reduced inference time by 60% through advanced optimization passes and memory layout improvements."
                                </p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-2 py-1 text-xs bg-orange-100 text-orange-800 rounded">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-emerald-100 text-emerald-800 rounded">"LLVM"</span>
                                    <span class="px-2 py-1 text-xs bg-yellow-100 text-yellow-800 rounded">"ML"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-500">"10 months"</span>
                                    <div class="flex items-center text-yellow-600">
                                        <span class="text-sm font-medium">"60% faster"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Project 5: Distributed Storage System
                        <div class="flex-none w-80 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 group">
                            <div class="p-6">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="w-12 h-12 bg-gradient-to-r from-teal-500 to-cyan-600 rounded-xl flex items-center justify-center">
                                        <span class="text-white text-xl font-bold">"💾"</span>
                                    </div>
                                    <span class="px-3 py-1 text-xs font-medium bg-teal-100 text-teal-800 rounded-full">"Infrastructure"</span>
                                </div>
                                <h3 class="text-lg font-bold text-gray-900 mb-3 group-hover:text-blue-600 transition-colors">
                                    "Distributed Storage System"
                                </h3>
                                <p class="text-gray-600 text-sm leading-relaxed mb-4">
                                    "Architected petabyte-scale distributed storage with automatic replication and self-healing. 
                                    Achieved 99.99% availability with sub-millisecond read latencies across global deployments."
                                </p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-2 py-1 text-xs bg-orange-100 text-orange-800 rounded">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-teal-100 text-teal-800 rounded">"Distributed"</span>
                                    <span class="px-2 py-1 text-xs bg-gray-100 text-gray-800 rounded">"Storage"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-500">"14 months"</span>
                                    <div class="flex items-center text-teal-600">
                                        <span class="text-sm font-medium">"99.99% uptime"</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Project 6: IoT Edge Computing Platform
                        <div class="flex-none w-80 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 group">
                            <div class="p-6">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="w-12 h-12 bg-gradient-to-r from-indigo-500 to-blue-600 rounded-xl flex items-center justify-center">
                                        <span class="text-white text-xl font-bold">"📡"</span>
                                    </div>
                                    <span class="px-3 py-1 text-xs font-medium bg-indigo-100 text-indigo-800 rounded-full">"IoT"</span>
                                </div>
                                <h3 class="text-lg font-bold text-gray-900 mb-3 group-hover:text-blue-600 transition-colors">
                                    "IoT Edge Computing Platform"
                                </h3>
                                <p class="text-gray-600 text-sm leading-relaxed mb-4">
                                    "Built lightweight edge computing framework for IoT devices with real-time data processing. 
                                    Optimized for ARM processors with 90% power efficiency improvement and secure communication."
                                </p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-2 py-1 text-xs bg-orange-100 text-orange-800 rounded">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-indigo-100 text-indigo-800 rounded">"IoT"</span>
                                    <span class="px-2 py-1 text-xs bg-green-100 text-green-800 rounded">"Edge"</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-500">"9 months"</span>
                                    <div class="flex items-center text-indigo-600">
                                        <span class="text-sm font-medium">"90% efficient"</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Navigation buttons
                    <button 
                        id="scroll-left"
                        class="absolute left-0 top-1/2 transform -translate-y-1/2 -translate-x-2 w-12 h-12 bg-white rounded-full shadow-lg border border-gray-200 flex items-center justify-center hover:bg-gray-50 transition-all duration-200 hover:shadow-xl z-10"
                        onclick="document.getElementById('projects-scroll').scrollBy({left: -320, behavior: 'smooth'})"
                    >
                        <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                        </svg>
                    </button>
                    
                    <button 
                        id="scroll-right"
                        class="absolute right-0 top-1/2 transform -translate-y-1/2 translate-x-2 w-12 h-12 bg-white rounded-full shadow-lg border border-gray-200 flex items-center justify-center hover:bg-gray-50 transition-all duration-200 hover:shadow-xl z-10"
                        onclick="document.getElementById('projects-scroll').scrollBy({left: 320, behavior: 'smooth'})"
                    >
                        <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                        </svg>
                    </button>
                </div>

                // Scroll indicators (dots)
                <div class="flex justify-center mt-8 space-x-2">
                    <div class="w-2 h-2 bg-blue-600 rounded-full"></div>
                    <div class="w-2 h-2 bg-gray-300 rounded-full"></div>
                </div>

                // Call to action
                <div class="text-center mt-12">
                    <p class="text-gray-600 mb-6">
                        "Want to see your project here? Let's discuss how we can help you achieve similar results."
                    </p>
                    <a href="/#contact" class="inline-flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
                        "Start Your Project"
                    </a>
                </div>
            </div>

            // Custom CSS for hiding scrollbar
            <style>
                "#projects-scroll::-webkit-scrollbar {
                    display: none;
                }"
            </style>
        </section>
    }
}