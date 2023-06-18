pub mod comment;
pub mod error;
pub mod topic;

pub use comment::*;
pub use error::*;
pub use topic::*;

use crate::api::get_me;
use crate::state::GlobalState;
use leptos::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    let state = expect_context::<RwSignal<GlobalState>>(cx);
    let user = create_resource(cx, state.read_only(), |state| async move {
        match state.token() {
            Some(token) => Some(get_me(token.to_string()).await),
            None => None,
        }
    });
    let username = move || user.read(cx).and_then(|u| u).map(|u| u.map(|u| u.username));
    let avatar = move || {
        view! { cx,
            <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
                <ErrorBoundary
                    // FIXME: error handling
                    fallback=move |cx, errors| view! { cx,
                        <a href="/login" class="btn btn-ghost">"Login"</a>
                    }
                >
                    {move || match username() {
                        Some(username) => {
                            view! { cx,
                                <div class="flex items-center">
                                    <span class="text-gray-300 text-sm pr-2">{username}</span>
                                </div>
                            }.into_view(cx)
                        }
                        None => {
                            view! { cx,
                                <a href="/login" class="btn btn-ghost">"Login"</a>
                            }.into_view(cx)
                        }
                    }}
                </ErrorBoundary>
            </Transition>
        }
    };

    view! { cx,
        <nav class="navbar bg-gray-800 px-2">
            <div class="flex-1">
                <a href="/" class="btn btn-ghost text-xl">"Voting System"</a>
                <div class="hidden md:block">
                    <div class="ml-4 flex items-center space-x-4">
                        <a href="/topics" class="btn btn-ghost">"Topics"</a>
                        <a href="/about" class="btn btn-ghost">"About"</a>
                    </div>
                </div>
            </div>
            <div class="flex-none">
                {avatar}
            </div>
        </nav>
    }
}
