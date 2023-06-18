use crate::api::{Topic, VoteOption};
use leptos::*;
use leptos_router::*;

#[component]
pub fn TopicCard(
    cx: Scope,
    #[prop(into)] topic: Signal<Topic>,
    #[prop(optional)] show_action: bool,
) -> impl IntoView {
    let topic = topic();
    let goto = use_navigate(cx);
    let open_topic = move |_| {
        // FIXME: error handling
        let _ = goto(&format!("/topic/{}", &topic.id), NavigateOptions::default());
    };
    let action = show_action.then(|| {
        view! { cx,
            <div class="card-actions justify-end">
            <button on:click=open_topic class="btn btn-primary">"Detail"</button>
            </div>
        }
    });
    view! { cx,
        <div class="card w-96 bg-base-200 mb-4 shadow-xl">
            <div class="card-body">
                <div class="text-3xl font-semibold">{topic.description}</div>
                <p>
                    "Starts at: "{topic.starts_at} <br />
                    "Ends at: "{topic.ends_at} <br />
                    "Updated at: "{topic.updated_at} <br />
                    "Stage: "{topic.stage} <br />
                    {action}
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn OptionCard<F, IV>(
    cx: Scope,
    #[prop(into)] option: Signal<VoteOption>,
    #[prop(default = None)] action: Option<F>,
    #[prop(optional)] extra_class: String,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    view! { cx,
        <div class=format!("card card-compact w-96 m-4 {extra_class}")>
            <div class="card-body">
                <div class="card-title">
                    {option().label}
                </div>
                <p>{option().description}</p>
            </div>
            {action.map(|action| {
                view! { cx,
                    <div class="card-actions justify-end">
                        {action}
                    </div>
                }
            })}
        </div>
    }
}
