use js_sys::Date;
use wasm_bindgen::prelude::*;

#[allow(long_running_const_eval)]
#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    fn dateformat(date: &Date, formatString: &str) -> String;
    fn generateUuidv4() -> String;
    fn setupSolanaAdapter();
    fn openModal();
    fn getWalletMetadata() -> String;
    fn addDate(date: &Date, duration: JsValue) -> Date;
    fn subDate(date: &Date, duration: JsValue) -> Date;
}

#[wasm_bindgen]
pub fn date_format(date: &Date, format_string: &str) -> String {
    dateformat(date, format_string)
}

#[wasm_bindgen]
pub fn generate_uuid_v4() -> String {
    generateUuidv4()
}

#[wasm_bindgen]
pub fn setup_solana_adapter() {
    setupSolanaAdapter()
}

#[wasm_bindgen]
pub fn open_modal() {
    openModal()
}

#[wasm_bindgen]
pub fn get_wallet_info() -> String {
    getWalletMetadata()
}

#[wasm_bindgen]
pub fn add_date(date: &Date, duration: JsValue) -> Date {
    addDate(date, duration)
}

#[wasm_bindgen]
pub fn sub_date(date: &Date, duration: JsValue) -> Date {
    subDate(date, duration)
}


