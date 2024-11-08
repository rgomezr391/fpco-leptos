use js_sys::Date;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::Serialize;
use wasm_bindgen::JsValue;
// use wasm_bindgen::prelude::*;
// use std::process::Command;
// use serde_wasm_bindgen::{to_value, from_value};
use crate::bindgen::{add_date, date_format, generate_uuid_v4, get_wallet_info, open_modal, setup_solana_adapter, sub_date};

/////////////////////////////////////////////////////////////////////////////////////

// #[wasm_bindgen(module = "/src/wrapper/wrapper.js")]
// extern "C" {
//     fn getFirstElement(array: JsValue) -> JsValue;
// }

// #[cfg(feature = "ssr")]
// #[wasm_bindgen(module = "/src/package.js")]
// extern "C" {
//     fn generatedUuid2() -> String;
// }

// #[cfg(feature = "ssr")]
// #[wasm_bindgen]
// pub fn my_generateUUid2() -> String {
//     generatedUuid2()
// }

// // #[wasm_bindgen(module = "/src/wrapper/wrapper.js")]
// // extern "C" {
// //     fn generatedUuid() -> String;
// //     // fn name() -> String;
// // }

// #[cfg(feature = "ssr")]
// #[wasm_bindgen]
// pub fn call_js_function(function: &str, param: &str) -> String {
//     // let output = Command::new("node")
//     //     .arg("/src/wrapper/wrapper.js")
//     //     .arg(param)
//     //     .output()
//     //     .expect("Failed to execute command");
//     // println!("Function: {}, Param: {}", function, param);

//     let output = Command::new("npx")
//         .arg("run-func")
//         .arg("src/wrapper/wrapper.js")
//         .arg(function)
//         .arg(param)
//         .output()
//         .expect("Failed to execute command");

//     String::from_utf8_lossy(&output.stdout).to_string()
// }

// #[cfg(feature = "ssr")]
// #[wasm_bindgen]
// pub fn call_js_function_no_params(function: &str) -> String {
//     // let output = Command::new("node")
//     //     .arg("/src/wrapper/wrapper.js")
//     //     .arg(param)
//     //     .output()
//     //     .expect("Failed to execute command");
//     // println!("Function: {}, Param: {}", function, param);

//     let output = Command::new("npx")
//         .arg("run-func")
//         .arg("src/wrapper/wrapper.js")
//         .arg(function)
//         .output()
//         .expect("Failed to execute command");

//     String::from_utf8_lossy(&output.stdout).to_string()
// }


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
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let (today, set_today) = create_signal(String::new());
    let (yesterday, set_yesterday) = create_signal(String::new());
    let (tomorrow, set_tomorrow) = create_signal(String::new());
    let (uuid, set_uuid) = create_signal(String::new());

    // Use create_effect to run a function on component load
    create_effect(move |_| {
        // This function will run when the component is first rendered
        setup_solana_adapter();
        // logging::log!("RESULT {}", result3);
        // my_open_modal();
    });

    // let on_click = move |_| {
    //     set_count.update(|count| *count += 1);
    //     spawn_local(async {
    //         let _ = test_server_function().await;
    //     });
    // };

    // let on_click_2 = move |_| {

    //     // Calling date-fns function
    //     let now = Date::new_0();
    //     let formatted_date: String = date_format(&now, "'Today is a' eeee");
    //     // Update the signal with the new input value

    //     set_today.update(|today| *today = formatted_date);

    //     logging::log!("Formatted Date: {}", today.get());
        
    //     let my_uuid = generate_uuid_v4();
    //     logging::log!("UUID {}", my_uuid);
    // };
    

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

    // let get_wallet_info = move |_| {
    //     let result = get_wallet_info();
    //     logging::log!("WALLET NAME {}", result);
    // };

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
            <appkit-button balance="show"></appkit-button>
        </div>
        <hr></hr>

        // <button on:click=on_click>"Click Me: " {count}</button>
        // <button on:click=on_click_2>"Click Me 2"</button>
        // <button on:click=get_wallet_info>"Get Wallet Info"</button>
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

    // logging::log!("Init");
    // let array = vec!["apple", "banana", "cherry"];
    // let js_value = to_value(&array).unwrap();
    // let first_element = getFirstElement(js_value);
    // let first: String = from_value(first_element).unwrap();  // Convert back to Rust String
    // logging::log!("First: {}", first);

    // let result2 = my_generateUUid2();
    // logging::log!("Result: {}", result2);
    
    // logging::log!("I'm running on the server");
    // let result = call_js_function("myFunction", "Hello World");
    // logging::log!("{}", result);
    // let result_no_params = call_js_function_no_params("generatedUuid");
    // logging::log!("UUID {}", result_no_params);

    Ok(())
}
