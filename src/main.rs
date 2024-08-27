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
        <ProgressBar progress = double_count />
    }
}

#[component]
#[allow(non_snake_case)]
fn ProgressBar(
    #[prop(default = 100)]
    max: u16,
    progress: impl Fn() -> i32 + 'static,
) -> impl IntoView {
    view! {
        <progress
            max = max
            value = progress
        />
        <br />
    }
}
