use crate::component::*;
use crate::page::*;
use crate::state::GlobalState;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    provide_context(cx, create_rw_signal(cx, GlobalState::new(cx)));

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        // sets the document title
        <Title text="NTNU CSIE Online Voting System" />
        // Load tailwind css
        <link href="https://cdn.jsdelivr.net/npm/daisyui@3.1.1/dist/full.css" rel="stylesheet" type="text/css" />
        <Script src="https://cdn.tailwindcss.com"></Script>

        // content for this welcome page
        <Router>
            <main>
                <NavBar />
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/topic/create" view=|cx| view! { cx, <CreateTopicPage/> }/>
                    <Route path="/topic/:id" view=|cx| view! { cx, <TopicPage/> }/>
                    <Route path="/login" view=|cx| view! { cx, <LoginPage/> }/>
                    <Route path="/signup" view=|cx| view! { cx, <SignupPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}
