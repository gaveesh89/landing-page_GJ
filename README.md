# AtomicIncrement Landing Page

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/gaveesh89/landing-page_GJ)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Leptos](https://img.shields.io/badge/leptos-0.6+-blue.svg)](https://leptos.dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A modern, high-performance landing page and blog system built with Rust and Leptos, featuring a complete case studies section, responsive design, and professional styling.

## 🚀 Live Demo

- **Website**: [http://127.0.0.1:8080](http://127.0.0.1:8080) (development)
- **Case Studies**: [http://127.0.0.1:8080/blog](http://127.0.0.1:8080/blog)
- **About Page**: [http://127.0.0.1:8080/about](http://127.0.0.1:8080/about)

## ✨ Features

### 🌐 **Landing Page**
- **Modern Navigation**: Responsive menu bar with mobile hamburger menu
- **Hero Section**: Engaging introduction with technical visualization
- **Features Grid**: Highlighting key differentiators and capabilities
- **Core Capabilities**: Technical expertise showcase with visual indicators
- **Testimonials**: Customer feedback and success stories
- **Recent Projects**: Horizontal scrolling project showcase
- **About Founder**: Dedicated about page with professional information
- **Contact Section**: Professional contact form and information
- **Responsive Design**: Optimized for desktop, tablet, and mobile devices

### 🧭 **Navigation Structure**
- **Home**: Main landing page with all sections
- **Services**: Quick navigation to services section
- **About**: Dedicated founder and company information page
- **Case Studies**: Internal blog system with technical posts and tutorials
- **Contact**: Direct link to contact section

### 📝 **Blog System (Case Studies)**
- **Markdown Support**: Full markdown parsing with frontmatter
- **Professional Styling**: Dark theme with purple accents and hover effects
- **Technical Content**: 4 comprehensive case studies and tutorials
- **SEO Optimized**: Proper meta tags, excerpts, and structured content
- **Responsive Reading**: Optimized typography and spacing for readability
- **Code Highlighting**: Syntax highlighting for code blocks
- **Navigation Integration**: Seamless integration with main site navigation

### 🎨 **Design & Performance**
- **Dark Theme**: Professional dark design with gradient backgrounds
- **Smooth Animations**: Hover effects, transitions, and micro-interactions
- **Performance Optimized**: WebAssembly for near-native performance
- **Accessibility**: WCAG compliant with proper ARIA labels and keyboard navigation
- **Mobile First**: Responsive design that works perfectly on all devices

## 🛠️ Technology Stack

- **[Rust](https://www.rust-lang.org/)**: Systems programming language for performance and safety
- **[Leptos](https://leptos.dev/)**: Modern reactive web framework for Rust
- **[Trunk](https://trunkrs.dev/)**: Rust web application bundler
- **[Tailwind CSS](https://tailwindcss.com/)**: Utility-first CSS framework
- **[WebAssembly](https://webassembly.org/)**: For running Rust code in the browser
- **[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)**: Markdown parser for blog content
- **[serde](https://serde.rs/)**: Serialization framework for data handling
- **[chrono](https://github.com/chronotope/chrono)**: Date and time library

## 📋 Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Trunk**: Web application bundler for Rust
  ```bash
  cargo install trunk
  ```
- **wasm32 target**: WebAssembly compilation target
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## 🚀 Quick Start

1. **Clone the repository**:
   ```bash
   git clone https://github.com/gaveesh89/landing-page_GJ.git
   cd landing-page_GJ
   ```

2. **Install dependencies and build**:
   ```bash
   cargo build
   ```

3. **Start development server**:
   ```bash
   trunk serve
   ```

4. **Open in browser**: Navigate to [http://localhost:8080](http://localhost:8080)

## 📁 Project Structure

```
landing-page_GJ/
├── 📁 src/
│   ├── 📄 lib.rs                    # Main application entry point
│   ├── 📄 blog.rs                   # Blog system with markdown parsing
│   ├── 📁 components/               # Reusable UI components
│   │   ├── 📄 mod.rs               # Component module exports
│   │   ├── 📄 header.rs            # Navigation header with case studies link
│   │   ├── 📄 hero.rs              # Hero section with tech visualization
│   │   ├── 📄 features.rs          # Areas of expertise grid
│   │   ├── 📄 testimonials.rs      # Customer testimonials
│   │   ├── 📄 contact.rs           # Professional contact form
│   │   └── 📄 footer.rs            # Footer with social links
│   └── 📁 pages/                   # Page components
│       ├── 📄 mod.rs               # Page module exports
│       ├── 📄 about.rs             # About founder page
│       └── 📄 blog.rs              # Blog list and post pages
├── 📁 content/
│   └── 📁 posts/                   # Blog post markdown files
│       ├── 📄 hello-world.md       # Welcome post
│       ├── 📄 getting-started.md   # Systems programming guide
│       ├── 📄 rust-tips.md         # Performance optimization tips
│       └── 📄 What is an atomic increment.md  # Company story
├── 📄 index.html                   # Main HTML with blog CSS styling
├── 📄 Cargo.toml                   # Rust dependencies and configuration
├── 📄 README.md                    # This documentation
└── 📁 dist/                        # Built WebAssembly and assets
```

## 📝 Blog Content Management

### Adding New Blog Posts

1. **Create a new markdown file** in `content/posts/`:
   ```markdown
   ---
   title: "Your Post Title"
   date: "2024-11-08"
   excerpt: "Brief description of your post"
   ---
   
   # Your Post Content
   
   Write your content here using markdown...
   ```

2. **Update `src/blog.rs`** to include the new post:
   ```rust
   // Add to get_all_posts() function
   let your_post_content = include_str!("../content/posts/your-post.md");
   posts.push(parse_post("your-post-slug", your_post_content));
   ```

3. **The post will be available** at `/blog/your-post-slug`

### Blog Features
- **Frontmatter Support**: YAML frontmatter for metadata
- **Markdown Parsing**: Full CommonMark support with code highlighting
- **Automatic Sorting**: Posts sorted by date (newest first)
- **SEO Friendly**: Proper excerpts and meta information
- **Responsive Design**: Optimized reading experience

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

Key test coverage:
- ✅ Blog post parsing and frontmatter extraction
- ✅ Markdown to HTML conversion
- ✅ Post retrieval by slug
- ✅ All posts loading (currently 4 posts)

## 🏗️ Building for Production

1. **Optimize build**:
   ```bash
   trunk build --release
   ```

2. **Deploy the `dist/` folder** to your web server

3. **Environment variables** (optional):
   ```bash
   export RUSTFLAGS="-C target-cpu=native"
   ```

## 🎨 Customization Guide

### **Styling**
- **Colors**: Modify CSS variables in `index.html`
- **Layout**: Adjust Tailwind classes in components
- **Blog Styling**: Update blog-specific CSS classes

### **Content**
- **Company Info**: Update in `src/components/`
- **Blog Posts**: Add new markdown files in `content/posts/`
- **Navigation**: Modify links in `header.rs` and `footer.rs`

### **Features**
- **Add Components**: Create new components in `src/components/`
- **Routing**: Add new routes in `src/lib.rs`
- **Pages**: Create new pages in `src/pages/`

## 🌐 Deployment Options

### **Static Hosting**
- GitHub Pages
- Netlify
- Vercel
- AWS S3 + CloudFront

### **Server Deployment**
- Any static file server (nginx, Apache)
- CDN for optimal performance

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## � Recent Updates

### **v1.1.0** - November 8, 2025
- ✅ **Navigation Simplified**: Removed external "Blog" link from header navigation
- ✅ **Case Studies Focus**: "Case Studies" tab now exclusively points to internal blog system
- ✅ **Mobile Navigation**: Updated mobile menu to match desktop navigation changes
- ✅ **Documentation**: Updated README with current navigation structure
- ✅ **Performance**: Cleaner navigation reduces external dependencies

### **v1.0.0** - November 8, 2025
- ✅ **Complete Blog System**: Full markdown parsing with frontmatter support
- ✅ **Professional Styling**: Dark theme with responsive design
- ✅ **4 Technical Posts**: Comprehensive case studies and tutorials
- ✅ **Routing System**: Integrated blog routes with main application
- ✅ **SEO Optimization**: Proper meta tags and structured content

## �📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Company**: [Atomic Increment Ltd](https://atomicincrement.com)
- **Blog**: [Technical Blog](https://atomicincrement.github.io/)
- **LinkedIn**: [Company Page](https://www.linkedin.com/company/atomic-increment-limited/)
- **GitHub**: [@atomicincrement](https://github.com/atomicincrement)
- **Twitter/X**: [@quaternioso](https://x.com/quaternioso)

---

Built with ❤️ using Rust and Leptos by [Atomic Increment](https://atomicincrement.com)