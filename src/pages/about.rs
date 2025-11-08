use leptos::*;
use leptos_meta::*;
use crate::components::{Header, Footer};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About - Atomic Increment Ltd."/>
        <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
            <Header/>
            <AboutContent/>
            <Footer/>
        </div>
    }
}

#[component]
fn AboutContent() -> impl IntoView {
    view! {
        <div class="pt-16 pb-20">
            // Hero section for About page
            <section class="relative py-20 bg-gradient-to-r from-slate-900 via-purple-900 to-slate-900">
                <div class="absolute inset-0 bg-black/20"></div>
                <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h1 class="text-4xl md:text-6xl font-bold text-white mb-6">
                        "About " <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-400">"Atomic Increment"</span>
                    </h1>
                    <p class="text-xl text-gray-300 max-w-3xl mx-auto">
                        "Pioneering systems programming excellence through three decades of innovation, 
                        education, and cutting-edge technology development."
                    </p>
                </div>
            </section>

            // Company Story Section
            <section class="py-20 bg-white">
                <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="grid lg:grid-cols-2 gap-12 items-center">
                        <div>
                            <h2 class="text-3xl font-bold text-gray-900 mb-6">
                                "Our " <span class="text-blue-600">"Story"</span>
                            </h2>
                            <div class="space-y-4 text-gray-600 leading-relaxed">
                                <p>
                                    "Founded on the principle that exceptional software requires exceptional expertise, 
                                    Atomic Increment represents the culmination of decades of systems programming mastery."
                                </p>
                                <p>
                                    "Our journey began in the academic halls of Oxford University, where foundational 
                                    computer science principles met real-world application. From teaching the next 
                                    generation of programmers to architecting mission-critical systems, we've maintained 
                                    an unwavering commitment to excellence."
                                </p>
                                <p>
                                    "Today, we bridge the gap between cutting-edge research and practical implementation, 
                                    delivering solutions that push the boundaries of what's possible in systems programming."
                                </p>
                            </div>
                        </div>
                        <div class="relative">
                            <div class="bg-gradient-to-r from-blue-500 to-purple-600 rounded-2xl p-8 text-white">
                                <div class="grid grid-cols-2 gap-6 text-center">
                                    <div>
                                        <div class="text-3xl font-bold">"30+"</div>
                                        <div class="text-blue-100">"Years Experience"</div>
                                    </div>
                                    <div>
                                        <div class="text-3xl font-bold">"1000+"</div>
                                        <div class="text-blue-100">"Students Trained"</div>
                                    </div>
                                    <div>
                                        <div class="text-3xl font-bold">"50+"</div>
                                        <div class="text-blue-100">"Enterprise Projects"</div>
                                    </div>
                                    <div>
                                        <div class="text-3xl font-bold">"15+"</div>
                                        <div class="text-blue-100">"Technologies Mastered"</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Andy's Detailed Biography
            <section class="py-20 bg-gray-50">
                <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl font-bold text-gray-900 mb-4">
                            "Meet " <span class="text-blue-600">"Andy Thomason"</span>
                        </h2>
                        <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                            "Founder & Principal Systems Architect"
                        </p>
                    </div>

                    <div class="grid lg:grid-cols-3 gap-12">
                        // Profile column
                        <div class="lg:col-span-1">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <div class="w-32 h-32 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full mx-auto mb-6 flex items-center justify-center">
                                    <span class="text-white text-4xl font-bold">"AT"</span>
                                </div>
                                <h3 class="text-xl font-bold text-center mb-4">"Andy Thomason"</h3>
                                <div class="space-y-3 text-sm text-gray-600">
                                    <div class="flex items-center">
                                        <span class="w-2 h-2 bg-blue-500 rounded-full mr-3"></span>
                                        "Oxford Computer Science"
                                    </div>
                                    <div class="flex items-center">
                                        <span class="w-2 h-2 bg-blue-500 rounded-full mr-3"></span>
                                        "30+ Years in Systems Programming"
                                    </div>
                                    <div class="flex items-center">
                                        <span class="w-2 h-2 bg-blue-500 rounded-full mr-3"></span>
                                        "Former University Lecturer"
                                    </div>
                                    <div class="flex items-center">
                                        <span class="w-2 h-2 bg-blue-500 rounded-full mr-3"></span>
                                        "Blockchain Pioneer"
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Biography content
                        <div class="lg:col-span-2">
                            <div class="space-y-6 text-gray-600 leading-relaxed">
                                <p>
                                    "Andy Thomason brings over three decades of systems programming expertise to every project. 
                                    His journey began with a Computer Science degree from Oxford University, where he developed 
                                    a deep understanding of computational theory and practical implementation."
                                </p>
                                <p>
                                    "As a university lecturer, Andy shaped the minds of over 1,000 students, teaching everything 
                                    from fundamental algorithms to advanced systems architecture. His teaching philosophy emphasizes 
                                    both theoretical rigor and practical application – a balance that defines Atomic Increment's 
                                    approach to consulting."
                                </p>
                                <p>
                                    "Andy's technical expertise spans the full spectrum of systems programming. He's architected 
                                    high-performance game engines, built mission-critical compilers, and pioneered blockchain 
                                    implementations when the technology was still in its infancy. His work in SIMD optimization 
                                    has delivered performance improvements that seemed impossible to achieve."
                                </p>
                                <p>
                                    "What sets Andy apart is his ability to see the bigger picture while never losing sight 
                                    of implementation details. Whether it's designing a memory-safe architecture or optimizing 
                                    critical performance bottlenecks, he brings both strategic vision and hands-on expertise 
                                    to every challenge."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Technical Expertise Deep Dive
            <section class="py-20 bg-white">
                <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl font-bold text-gray-900 mb-4">
                            "Technical " <span class="text-blue-600">"Expertise"</span>
                        </h2>
                        <p class="text-xl text-gray-600">
                            "Deep specialization across the systems programming spectrum"
                        </p>
                    </div>

                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Rust & Memory Safety
                        <div class="bg-gradient-to-br from-orange-50 to-red-50 rounded-xl p-6 border border-orange-100">
                            <div class="w-12 h-12 bg-gradient-to-r from-orange-500 to-red-500 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-white font-bold">"R"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-3">"Rust & Memory Safety"</h3>
                            <p class="text-gray-600 text-sm leading-relaxed">
                                "Advanced Rust programming with deep understanding of ownership, borrowing, 
                                and zero-cost abstractions. Expert in building memory-safe systems without 
                                performance compromise."
                            </p>
                        </div>

                        // Game Engine Architecture
                        <div class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-xl p-6 border border-blue-100">
                            <div class="w-12 h-12 bg-gradient-to-r from-blue-500 to-indigo-500 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-white font-bold">"G"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-3">"Game Engine Architecture"</h3>
                            <p class="text-gray-600 text-sm leading-relaxed">
                                "Complete game engine development from graphics pipelines to physics simulation. 
                                Expertise in real-time rendering, asset management, and performance optimization."
                            </p>
                        </div>

                        // Compiler Construction
                        <div class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl p-6 border border-purple-100">
                            <div class="w-12 h-12 bg-gradient-to-r from-purple-500 to-pink-500 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-white font-bold">"C"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-3">"Compiler Construction"</h3>
                            <p class="text-gray-600 text-sm leading-relaxed">
                                "Deep expertise in lexical analysis, parsing, optimization, and code generation. 
                                Built production compilers with advanced optimization techniques."
                            </p>
                        </div>

                        // Blockchain & Distributed Systems
                        <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-green-100">
                            <div class="w-12 h-12 bg-gradient-to-r from-green-500 to-emerald-500 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-white font-bold">"B"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-3">"Blockchain & Distributed Systems"</h3>
                            <p class="text-gray-600 text-sm leading-relaxed">
                                "Early blockchain adoption with deep understanding of consensus mechanisms, 
                                cryptographic primitives, and distributed system design patterns."
                            </p>
                        </div>

                        // SIMD & Performance Optimization
                        <div class="bg-gradient-to-br from-yellow-50 to-orange-50 rounded-xl p-6 border border-yellow-100">
                            <div class="w-12 h-12 bg-gradient-to-r from-yellow-500 to-orange-500 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-white font-bold">"S"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-3">"SIMD & Performance Optimization"</h3>
                            <p class="text-gray-600 text-sm leading-relaxed">
                                "Vectorization expertise with SIMD instruction sets. Micro-optimization 
                                techniques that deliver measurable performance improvements."
                            </p>
                        </div>

                        // Systems Architecture
                        <div class="bg-gradient-to-br from-teal-50 to-cyan-50 rounded-xl p-6 border border-teal-100">
                            <div class="w-12 h-12 bg-gradient-to-r from-teal-500 to-cyan-500 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-white font-bold">"A"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-3">"Systems Architecture"</h3>
                            <p class="text-gray-600 text-sm leading-relaxed">
                                "Large-scale system design with focus on scalability, reliability, and 
                                maintainability. Expert in designing architectures that evolve gracefully."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Values & Philosophy
            <section class="py-20 bg-gradient-to-r from-slate-900 via-purple-900 to-slate-900">
                <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl font-bold text-white mb-4">
                            "Our " <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-400">"Philosophy"</span>
                        </h2>
                        <p class="text-xl text-gray-300">
                            "The principles that guide everything we do"
                        </p>
                    </div>

                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div class="text-center">
                            <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center mx-auto mb-4">
                                <span class="text-white text-2xl">"⚡"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-white mb-3">"Performance First"</h3>
                            <p class="text-gray-300 text-sm leading-relaxed">
                                "Every line of code is written with performance in mind. We believe that 
                                efficiency is not just a feature – it's a fundamental requirement."
                            </p>
                        </div>

                        <div class="text-center">
                            <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center mx-auto mb-4">
                                <span class="text-white text-2xl">"🛡️"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-white mb-3">"Safety & Reliability"</h3>
                            <p class="text-gray-300 text-sm leading-relaxed">
                                "Memory safety and type safety are non-negotiable. We build systems 
                                that fail gracefully and recover automatically."
                            </p>
                        </div>

                        <div class="text-center">
                            <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center mx-auto mb-4">
                                <span class="text-white text-2xl">"🎯"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-white mb-3">"Precision & Clarity"</h3>
                            <p class="text-gray-300 text-sm leading-relaxed">
                                "Clean, maintainable code that expresses intent clearly. Every abstraction 
                                serves a purpose, every optimization is measured."
                            </p>
                        </div>

                        <div class="text-center">
                            <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center mx-auto mb-4">
                                <span class="text-white text-2xl">"🔬"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-white mb-3">"Scientific Approach"</h3>
                            <p class="text-gray-300 text-sm leading-relaxed">
                                "Data-driven decisions backed by rigorous testing and measurement. 
                                We prove our optimizations work before deploying them."
                            </p>
                        </div>

                        <div class="text-center">
                            <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center mx-auto mb-4">
                                <span class="text-white text-2xl">"🚀"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-white mb-3">"Innovation & Excellence"</h3>
                            <p class="text-gray-300 text-sm leading-relaxed">
                                "Continuous learning and adaptation to emerging technologies while 
                                maintaining the highest standards of craftsmanship."
                            </p>
                        </div>

                        <div class="text-center">
                            <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center mx-auto mb-4">
                                <span class="text-white text-2xl">"🤝"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-white mb-3">"Partnership & Growth"</h3>
                            <p class="text-gray-300 text-sm leading-relaxed">
                                "We don't just deliver solutions – we transfer knowledge and empower 
                                teams to maintain and evolve their systems independently."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // CTA Section
            <section class="py-20 bg-white">
                <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h2 class="text-3xl font-bold text-gray-900 mb-6">
                        "Ready to Work " <span class="text-blue-600">"Together?"</span>
                    </h2>
                    <p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                        "Let's discuss how our expertise can accelerate your next systems programming project."
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a href="/#contact" class="inline-flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
                            "Start a Conversation"
                        </a>
                        <a href="/#services" class="inline-flex items-center justify-center px-8 py-3 border border-gray-300 text-base font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors">
                            "View Our Services"
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}