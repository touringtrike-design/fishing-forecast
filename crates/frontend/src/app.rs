use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::MapView;
use crate::components::ForecastPanel;
use crate::components::CatchForm;
use crate::services::ApiClient;
use crate::services::api_client::{ForecastResponse, CatchRecord};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn updateUserMarker(lat: f64, lon: f64, heading: f64, wind_direction: f64);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Root application component.
/// 
/// Main app structure with fullscreen map and floating forecast panel.
#[component]
pub fn App() -> Element {
    // API client
    let api_client = ApiClient::default();
    let api_client_signal = use_signal(|| api_client.clone());
    
    // Selected location on map
    let mut selected_location = use_signal::<Option<(f64, f64)>>(|| {
        // Try to restore from URL or defaults
        None
    });
    
    // Forecast data and loading state
    let forecast = use_signal::<Option<ForecastResponse>>(|| None);
    let is_loading = use_signal(|| false);
    let error = use_signal::<Option<String>>(|| None);
    let user_location = use_signal::<Option<(f64, f64)>>(|| None);
    
    // Catch form modal state
    let mut show_catch_form = use_signal(|| false);
    let success_message = use_signal::<Option<String>>(|| None);
    
    // Show forecast panel state
    let mut show_forecast_panel = use_signal(|| false);
    
    // Restore location from localStorage on mount
    {
        let mut initialized = use_signal(|| false);
        use_effect(move || {
            if !*initialized.read() {
                // Try to load from localStorage using JavaScript
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(data)) = storage.get_item("last_location") {
                            if let Some((lat, lon)) = parse_coordinates(&data) {
                                selected_location.set(Some((lat, lon)));
                            }
                        }
                    }
                }
                *initialized.write() = true;
            }
        });
    }

    
    // Save location to localStorage whenever it changes
    use_effect(move || {
        if let Some((lat, lon)) = *selected_location.read() {
            let data = format!("{},{}", lat, lon);
            // Use localStorage to save
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("last_location", &data);
                }
            }
        }
    });
    
    // Handle location selection from map
    let on_location_selected = {
        to_owned![selected_location, show_forecast_panel, forecast, is_loading, error, api_client, user_location];
        move |(lat, lon): (f64, f64)| {
            let mut selected_location = selected_location;
            let mut show_forecast_panel = show_forecast_panel;
            selected_location.set(Some((lat, lon)));
            show_forecast_panel.set(true);

            spawn_forecast(
                api_client.clone(),
                lat,
                lon,
                forecast,
                is_loading,
                error,
                user_location,
            );
        }
    };
    
    // Handle catch form close
    let on_catch_form_close = move |_| {
        show_catch_form.set(false);
    };
    
    // Handle catch submission
    let on_catch_submit = {
        to_owned![show_catch_form, success_message];
        move |catch: CatchRecord| {
            show_catch_form.set(false);
            
            let weight_text = if let Some(w) = catch.weight_kg {
                format!(" ({} кг)", w)
            } else {
                String::new()
            };
            
            success_message.set(Some(format!(
                "✅ Улов успішно зареєстровано: {}{}",
                catch.fish_species, weight_text
            )));
        }
    };
    
    rsx! {
        div {
            class: "fixed inset-0 bg-white text-slate-900 flex flex-col w-screen h-screen",
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; width: 100vw; height: 100vh;",
            
            // Minimal header with title
            header {
                class: "bg-gradient-to-r from-blue-600 to-blue-700 text-white px-4 py-2 shadow-md z-10 flex-shrink-0",
                div {
                    class: "flex items-center justify-between",
                    h1 { class: "text-lg font-bold", "🎣 Прогноз Клювання" }
                    div { class: "flex items-center gap-2",
                        if selected_location().is_some() {
                            button {
                                class: "text-white hover:bg-blue-800 px-3 py-1 rounded text-sm",
                                onclick: move |_| show_forecast_panel.set(!show_forecast_panel()),
                                if show_forecast_panel() { "▼ Сховати" } else { "▲ Показати" }
                            }
                        }
                    }
                }
            }
            
            // Fullscreen map
            main {
                class: "flex-1 relative overflow-hidden w-full",
                style: "flex: 1; position: relative; overflow: hidden; width: 100%;",
                
                MapView {
                    on_location_selected: on_location_selected.clone(),
                    selected_location: selected_location(),
                }
            }
            
            // Floating forecast panel at bottom
            if show_forecast_panel() {
                div {
                    class: "absolute inset-x-0 bottom-24 max-h-[50vh] bg-white border-t border-slate-300 shadow-2xl rounded-t-xl overflow-y-auto z-30",
                    
                    div {
                        class: "p-4",
                        ForecastPanel {
                            forecast: forecast(),
                            is_loading: is_loading(),
                            error: error(),
                        }
                    }
                }
            }
            
            // Success message notification
            if let Some(msg) = success_message() {
                div {
                    class: "fixed top-4 right-4 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg z-50 animate-pulse",
                    "{msg}"
                }
            }
            
            // Floating Action Button for adding catch
            button {
                class: "fixed bottom-6 right-6 bg-blue-600 hover:bg-blue-700 text-white rounded-full w-16 h-16 shadow-lg flex items-center justify-center z-40 transition-all hover:scale-110 active:scale-95 p-3",
                onclick: move |_| show_catch_form.set(true),
                title: "Зареєструвати улов",
                dangerous_inner_html: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 90 110" width="100%" height="100%" style="display: block;">
                    <g>
                        <circle cx="30" cy="22" r="11" fill="white"/>
                        <path d="M 20 32 Q 20 30, 22 28 L 38 28 Q 40 30, 40 32 L 40 55 Q 40 58, 37 58 L 23 58 Q 20 58, 20 55 Z" fill="white"/>
                        <path d="M 22 32 Q 18 35, 17 42 Q 16 45, 18 47 L 25 50 Q 26 49, 25 47 L 22 38 Z" fill="white"/>
                        <path d="M 38 32 Q 42 35, 45 42 L 48 45 Q 49 46, 48 47 L 41 50 Q 40 49, 41 47 L 38 38 Z" fill="white"/>
                        <rect x="20" y="56" width="20" height="38" rx="3" fill="white" opacity="0.9"/>
                        <path d="M 22 94 L 22 102 Q 22 104, 24 104 L 28 104 Q 30 104, 30 102 L 30 94 Z" fill="white" opacity="0.9"/>
                        <path d="M 30 94 L 30 102 Q 30 104, 32 104 L 36 104 Q 38 104, 38 102 L 38 94 Z" fill="white" opacity="0.9"/>
                        <line x1="48" y1="45" x2="75" y2="18" stroke="white" stroke-width="3" stroke-linecap="round"/>
                        <line x1="48" y1="45" x2="80" y2="8" stroke="white" stroke-width="2.5" stroke-linecap="round"/>
                        <line x1="80" y1="8" x2="85" y2="35" stroke="white" stroke-width="0.8" opacity="0.7"/>
                    </g>
                </svg>"#
            }
            
            // Catch form modal
            if show_catch_form() {
                div {
                    class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4",
                    
                    div {
                        class: "bg-white rounded-lg max-w-lg w-full max-h-[90vh] overflow-y-auto",
                        
                        CatchForm {
                            api_client: api_client_signal,
                            user_location: user_location,
                            on_close: on_catch_form_close,
                            on_submit: on_catch_submit,
                        }
                    }
                }
            }
        }
    }
}

/// Parse coordinates from "lat,lon" format
fn parse_coordinates(data: &str) -> Option<(f64, f64)> {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() == 2 {
        if let (Ok(lat), Ok(lon)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            return Some((lat, lon));
        }
    }
    None
}

fn spawn_forecast(
    api_client: ApiClient,
    lat: f64,
    lon: f64,
    forecast: Signal<Option<ForecastResponse>>,
    is_loading: Signal<bool>,
    error: Signal<Option<String>>,
    user_location: Signal<Option<(f64, f64)>>,
) {
    let mut is_loading = is_loading;
    let mut error = error;
    let mut forecast = forecast;

    is_loading.set(true);
    error.set(None);

    spawn(async move {
        match api_client.get_forecast(lat, lon, None).await {
            Ok(result) => {
                let wind_dir = result.weather.wind_direction;

                // Update user marker with wind direction if we have user location
                if let Some((ulat, ulon)) = *user_location.read() {
                    updateUserMarker(ulat, ulon, 0.0, wind_dir);
                }

                forecast.set(Some(result));
                is_loading.set(false);
            }
            Err(e) => {
                error.set(Some(e.to_string()));
                is_loading.set(false);
            }
        }
    });
}


