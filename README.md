# AtomicIncrement Landing Page

A modern landing page built with Rust and Leptos featuring a responsive design inspired by the Fledgio website.

## Features

- **Modern Navigation**: Responsive menu bar with mobile support
- **Hero Section**: Engaging introduction with call-to-action buttons
- **Features Grid**: Highlighting key differentiators and capabilities
- **Testimonials**: Customer feedback and success stories
- **Contact Section**: Multiple ways to get in touch
- **Responsive Design**: Works perfectly on desktop and mobile devices

## Technology Stack

- **Rust**: System programming language for performance and safety
- **Leptos**: Modern reactive web framework for Rust
- **Tailwind CSS**: Utility-first CSS framework for styling
- **WebAssembly**: For running Rust code in the browser

## Development Setup

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Install `wasm-pack`:
   ```bash
   cargo install wasm-pack
   ```
3. Install a local development server (like `basic-http-server`):
   ```bash
   cargo install basic-http-server
   ```

### Build and Run

1. Build the WebAssembly package:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

2. Start a local development server:
   ```bash
   basic-http-server .
   ```

3. Open your browser and navigate to `http://localhost:4000`

## Project Structure

```
├── src/
│   ├── lib.rs              # Main application entry point
│   └── components/
│       ├── mod.rs          # Component module exports
│       ├── header.rs       # Navigation header component
│       ├── hero.rs         # Hero section component
│       ├── features.rs     # Features grid component
│       ├── testimonials.rs # Testimonials section component
│       ├── contact.rs      # Contact section component
│       └── footer.rs       # Footer component
├── index.html              # Main HTML file
├── Cargo.toml             # Rust project configuration
└── README.md              # This file
```

## Customization

The landing page is designed to be easily customizable:

1. **Company Information**: Update company name, logo, and contact details in the components
2. **Content**: Modify text content in each component file
3. **Styling**: Adjust Tailwind CSS classes for different colors, layouts, or animations
4. **Features**: Add or remove feature cards in the features section
5. **Testimonials**: Update customer testimonials with real feedback

## Deployment

For production deployment:

1. Build the optimized WebAssembly package:
   ```bash
   wasm-pack build --target web --out-dir pkg --release
   ```

2. Deploy the generated files (index.html, pkg/ directory) to your web server

## License

This project is open source and available under the MIT License.