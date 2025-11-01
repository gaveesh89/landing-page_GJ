use leptos::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <div class="relative isolate overflow-hidden">
            <div class="mx-auto max-w-7xl px-6 pb-24 pt-10 sm:pb-32 lg:flex lg:px-8 lg:py-40">
                <div class="mx-auto max-w-2xl flex-shrink-0 lg:mx-0 lg:max-w-xl lg:pt-8">
                    <h1 class="mt-10 text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">
                        "ACCELERATE SYSTEMS ENGINEERS."
                    </h1>
                    <p class="mt-6 text-lg leading-8 text-gray-600">
                        "IMPLEMENT ROBUST, PERFORMANCE-CRITICAL INFRASTRUCTURE. NAVIGATE RUST'S OWNERSHIP MODEL WITH SPECIALIZED GUIDANCE."
                    </p>
                    <div class="mt-10 flex items-center gap-x-6">
                        <a
                            href="#contact"
                            class="rounded-md bg-blue-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
                        >
                            "INITIATE CONSULTATION"
                        </a>
                        <a href="#services" class="text-sm font-semibold leading-6 text-gray-900">
                            "REVIEW CAPABILITIES"
                            <span aria-hidden="true">"→"</span>
                        </a>
                    </div>
                </div>
                <div class="mx-auto mt-16 flex max-w-2xl sm:mt-24 lg:ml-10 lg:mt-0 lg:mr-0 lg:max-w-none lg:flex-none xl:ml-32">
                    <div class="max-w-3xl flex-none sm:max-w-5xl lg:max-w-none">
                        <div class="relative w-[40rem] h-[30rem] sm:w-[57rem] md:-ml-4 lg:-ml-0">
                            <div class="h-full w-full rounded-xl bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 p-8 shadow-2xl ring-1 ring-slate-700">
                                <div class="flex flex-col h-full justify-between">
                                    <div>
                                        <div class="flex items-center space-x-2 mb-6">
                                            <div class="w-3 h-3 rounded-full bg-red-500"></div>
                                            <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                                            <div class="w-3 h-3 rounded-full bg-green-500"></div>
                                            <span class="ml-4 text-slate-400 text-sm font-mono">"systems/core.rs"</span>
                                        </div>
                                        <pre class="text-xs sm:text-sm text-slate-300 font-mono leading-relaxed overflow-x-auto">
                                            <code>
<span class="text-purple-400">"use"</span>" std::sync::Arc;\n"
<span class="text-purple-400">"use"</span>" tokio::sync::RwLock;\n\n"
<span class="text-blue-400">"#[derive(Debug, Clone)]"</span>"\n"
<span class="text-purple-400">"pub struct"</span>" "<span class="text-yellow-300">"SystemCore"</span>"<T: Send + Sync> {\n"
"    state: Arc<RwLock<T>>,\n"
"    metrics: "<span class="text-yellow-300">"PerformanceMetrics"</span>",\n"
"}\n\n"
<span class="text-purple-400">"impl"</span>"<T: Send + Sync> "<span class="text-yellow-300">"SystemCore"</span>"<T> {\n"
"    "<span class="text-purple-400">"pub async fn"</span>" "<span class="text-green-400">"process"</span>"(&self) -> "<span class="text-yellow-300">"Result"</span>"<(), Error> {\n"
"        "<span class="text-purple-400">"let"</span>" guard = self.state."<span class="text-green-400">"read"</span>"()."<span class="text-purple-400">"await"</span>";\n"
"        "<span class="text-slate-500">"// Zero-cost abstraction"</span>"\n"
"        "<span class="text-purple-400">"Ok"</span>"(())\n"
"    }\n"
"}"
                                            </code>
                                        </pre>
                                    </div>
                                    <div class="mt-4 pt-4 border-t border-slate-700">
                                        <div class="flex justify-between items-center text-xs text-slate-400 font-mono">
                                            <span>"⚡ ZERO-COST ABSTRACTIONS"</span>
                                            <span>"🔒 MEMORY SAFE"</span>
                                            <span>"⚙️ THREAD SAFE"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
