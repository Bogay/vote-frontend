use leptos::*;

#[component]
pub fn ErrorList(
    cx: Scope,
    children: Children,
    #[prop(default = "Error Occurred!".to_string())] error_title: String,
) -> impl IntoView {
    let (error_title, _) = create_signal(cx, error_title);

    view! { cx,
        <ErrorBoundary
            // ref: https://leptos-rs.github.io/leptos/view/07_errors.html?highlight=error%20handling#errorboundary
            fallback=move |cx, errors| view! { cx,
                <div class="alert alert-error p-4">
                    <div>
                        <h3>{error_title()}</h3>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                                .collect_view(cx)
                            }
                        </ul>
                    </div>
                </div>
            }
        >
            {children(cx)}
        </ErrorBoundary>
    }
}
