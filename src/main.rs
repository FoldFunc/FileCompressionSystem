use dioxus::prelude::*;
fn main() {
    // Launch the top-level App component
    dioxus::launch(|| {
        rsx! {
            Router::<Route> {}
        }
    });
}

// 1. Define all your routes here
#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Navbar{},

    #[route("/login")]
    Login {},

    #[route("/register")]
    Register {},

    #[route("/about")]
    About {},
}

// 2. App component: wrap Navbar + Outlet in Router
#[component]
fn App() -> Element {
    rsx! {
        Navbar {}
    }
}

// 3. Navbar with Links
#[component]
fn Navbar() -> Element {
    static CSS: Asset = asset!("/assets/main.css");
    rsx! {
        // Include your CSS
        document::Stylesheet { href: CSS }
        nav {
            ul {
                class: "navbar",
                li { Link { to: Route::Navbar{},    "MainPage" } }
                li { Link { to: Route::Login {},   "Login" } }
                li { Link { to: Route::Register {}, "Register" } }
                li { Link { to: Route::About {},   "About" } }
            }
        }
    }
}

// 4. One component per route:

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "main_page_text", h1 { "Welcome to FES!" } }
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        div {
            Navbar {}
        }
        div {
            class: "register-container",
            h1 { "Login" }
            p { "Please fill in the form to login into an account." }
            form {
                class: "register-form",
                input {
                    class: "form-input",
                    r#type: "text",
                    placeholder: "Enter your username",
                    required: "true",
                }
                input {
                    class: "form-input",
                    r#type: "email",
                    placeholder: "Enter your email",
                    required: "true",
                }
                input {
                    class: "form-input",
                    r#type: "password",
                    placeholder: "Create a password",
                    required: "true",
                }
                button {
                    class: "submit-btn",
                    r#type: "submit",
                    "Login"
                }
            }
        }
    }
}


#[component]
fn Register() -> Element {
    rsx! {
        div {
            Navbar {}
        }
        div {
            class: "register-container",
            h1 { "Register" }
            p { "Please fill in the form to create an account." }
            form {
                class: "register-form",
                input {
                    class: "form-input",
                    r#type: "text",
                    placeholder: "Enter your username",
                    required: "true",
                }
                input {
                    class: "form-input",
                    r#type: "email",
                    placeholder: "Enter your email",
                    required: "true",
                }
                input {
                    class: "form-input",
                    r#type: "password",
                    placeholder: "Create a password",
                    required: "true",
                }
                input {
                    class: "form-input",
                    r#type: "password",
                    placeholder: "Confirm your password",
                    required: "true",
                }
                button {
                    class: "submit-btn",
                    r#type: "submit",
                    "Register"
                }
            }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div {
            Navbar {}
        }
        div {
            class: "main_page_text",
            h1 { "About FES" }
            p { "FES is a secure file encryption system developed to protect your data." }
        }
    }
}

