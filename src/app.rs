use js_sys::Date;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::Serialize;
use wasm_bindgen::JsValue;
use crate::bindgen::{add_date, date_format, generate_uuid_v4, setup_solana_adapter, sub_date};

#[derive(Serialize)]
struct Duration {
    days: Option<u32>,
    hours: Option<u32>,
    minutes: Option<u32>,
    months: Option<u32>,
    seconds: Option<u32>,
    weeks: Option<u32>,
    years: Option<u32>,
}


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fpco-leptos.css"/>

        // sets the document title
        <Title text="Leptos Sample App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (today, set_today) = create_signal(String::new());
    let (yesterday, set_yesterday) = create_signal(String::new());
    let (tomorrow, set_tomorrow) = create_signal(String::new());
    let (uuid, set_uuid) = create_signal(String::new());
    let (is_visible, set_visible) = create_signal(false);
    
    // // Use create_effect to run a function on component load
    // create_effect(move |_| {
    //     setup_solana_adapter();
    // });

    let on_click_call_uuid = move |_| {
        set_uuid.update(|uuid| *uuid = generate_uuid_v4());
    };

    let on_click_call_date_fns = move |_| {
        // Initialize Data with today's date
        let now = Date::new_0();

        let duration = Duration {
            days: Some(1),
            hours: None,
            minutes: None,
            months: None,
            seconds: None,
            weeks: None,
            years: None,
        };

        let durationJsValue: JsValue = serde_wasm_bindgen::to_value(&duration).unwrap(); // Convert to JsValue
        let tomorrow_date = add_date(&now, durationJsValue);
        let tomorrow_formatted_date: String = date_format(&tomorrow_date, "'Tomorrow will be' eeee");
        set_tomorrow.update(|tomorrow| *tomorrow = tomorrow_formatted_date);

        let duration = Duration {
            days: Some(1),
            hours: None,
            minutes: None,
            months: None,
            seconds: None,
            weeks: None,
            years: None,
        };

        let durationJsValue: JsValue = serde_wasm_bindgen::to_value(&duration).unwrap(); // Convert to JsValue
        let yesterday_date = sub_date(&now, durationJsValue);
        let yesterday_formatted_date: String = date_format(&yesterday_date, "'Yesterday was' eeee");
        set_yesterday.update(|yesterday| *yesterday = yesterday_formatted_date);

        // Call NPM Function
        let formatted_date: String = date_format(&now, "'Today is' eeee");

        // Update the signal with the new input value
        set_today.update(|today| *today = formatted_date);
    };

    let on_click_call_reown = move |_| {
        setup_solana_adapter();
        set_visible.update(|visible| *visible = true);
    };

    view! {
        <h1>"Sample Leptos App for consuming NPM Modules"</h1>

        <div style="text-align: left;">
            <h3>"Date FNS NPM Module"</h3>
            <button on:click=on_click_call_date_fns>"Call"</button>
            <p>{yesterday}</p>
            <p>{today}</p>
            <p>{tomorrow}</p>
        </div>
        <hr></hr>
        <div style="text-align: left;">
            <h3>"UUID NPM Module"</h3>
            <button on:click=on_click_call_uuid>"Generate UUID"</button>
            <p>{uuid}</p>
        </div>
        <hr></hr>
        <div style="text-align: left;">
            <h3>"Reown NPM Module"</h3>
            <button on:click=on_click_call_reown>"Initialize Solana Adapter"</button>
            <br></br>
            <Show when=move || is_visible.get()>
                <appkit-button balance="show"></appkit-button>
            </Show>
        </div>
        <hr></hr>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server(TestServerFunction, "/api")]
pub async fn test_server_function() -> Result<(), ServerFnError> {
    Ok(())
}
