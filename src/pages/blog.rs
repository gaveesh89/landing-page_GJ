use leptos::*;
use leptos_router::*;

use crate::blog::{get_all_posts, get_post_by_slug};
use crate::components::{Header, Footer};

#[component]
pub fn BlogListPage() -> impl IntoView {
    let posts = get_all_posts();
    
    view! {
        <div class="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-black">
            <Header />
            
            {/* Hero Section */}
            <div class="bg-gradient-to-r from-purple-900/20 to-blue-900/20 py-20">
                <div class="container mx-auto px-4 text-center">
                    <h1 class="text-5xl font-bold text-white mb-4">
                        "Technical Blog"
                    </h1>
                    <p class="text-xl text-gray-300 max-w-2xl mx-auto">
                        "Insights, tutorials, and deep dives into systems programming, Rust, and software engineering excellence."
                    </p>
                </div>
            </div>
            
            {/* Blog Posts */}
            <div class="blog-container">
                {if posts.is_empty() {
                    view! {
                        <div class="text-center py-12">
                            <p class="text-gray-400 text-lg">"No blog posts available yet."</p>
                        </div>
                    }
                } else {
                    view! {
                        <div class="space-y-6">
                            {posts.into_iter().map(|post| {
                                view! {
                                    <article class="blog-card">
                                        <h2>
                                            <a href={format!("/blog/{}", post.slug)} class="blog-title">
                                                {post.title}
                                            </a>
                                        </h2>
                                        <div class="blog-date">
                                            {post.date}
                                        </div>
                                        <p class="blog-excerpt">
                                            {post.excerpt}
                                        </p>
                                        <a href={format!("/blog/{}", post.slug)} class="blog-read-more">
                                            "Read More"
                                        </a>
                                    </article>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }
                }}
            </div>
            
            <Footer />
        </div>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();
    
    let post = create_memo(move |_| {
        get_post_by_slug(&slug())
    });
    
    view! {
        <div class="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-black">
            <Header />
            
            <div class="blog-post-container">
                {move || {
                    match post.get() {
                        Some(post) => {
                            view! {
                                <article>
                                    <a href="/blog" class="blog-back-link">
                                        "Back to Blog"
                                    </a>
                                    
                                    <header class="blog-post-header">
                                        <h1 class="blog-post-title">
                                            {post.title}
                                        </h1>
                                        <div class="blog-post-meta">
                                            <span>{post.date}</span>
                                        </div>
                                    </header>
                                    
                                    <div class="blog-content" inner_html={post.content}></div>
                                </article>
                            }
                        }
                        None => {
                            view! {
                                <article>
                                    <div class="text-center py-12">
                                        <h1 class="text-3xl font-bold text-white mb-4">"Post Not Found"</h1>
                                        <p class="text-gray-400 mb-6">"The blog post you're looking for doesn't exist."</p>
                                        <a href="/blog" class="blog-back-link">
                                            "Back to Blog"
                                        </a>
                                    </div>
                                </article>
                            }
                        }
                    }
                }}
            </div>
            
            <Footer />
        </div>
    }
}