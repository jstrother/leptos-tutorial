use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/>});
}

#[component]
#[allow(non_snake_case)]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button on:click = move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        <br />
        <ProgressBar progress = count />
        <ProgressBar progress = Signal::derive(double_count) />
    }
}

/// Shows progress towards a goal.
#[component]
#[allow(non_snake_case)]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// The current progress towards the goal.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max = max
            value = progress
        />
        <br />
    }
}
