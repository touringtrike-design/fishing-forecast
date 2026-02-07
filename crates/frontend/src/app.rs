use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::MapView;
use crate::components::ForecastPanel;
use crate::services::ApiClient;
use crate::services::api_client::ForecastResponse;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn updateUserMarker(lat: f64, lon: f64, heading: f64, wind_direction: f64);
}

/// Root application component.
/// 
/// Main app structure with map and forecast panel.
#[component]
pub fn App() -> Element {
    // API client
    let api_client = ApiClient::default();
    
    // Selected location on map
    let selected_location = use_signal::<Option<(f64, f64)>>(|| None);
    
    // Forecast data and loading state
    let forecast = use_signal::<Option<ForecastResponse>>(|| None);
    let is_loading = use_signal(|| false);
    let error = use_signal::<Option<String>>(|| None);
    let user_location = use_signal::<Option<(f64, f64)>>(|| None);
    
    // Handle location selection from map
    let on_location_selected = {
        to_owned![selected_location, forecast, is_loading, error, api_client, user_location];
        move |(lat, lon): (f64, f64)| {
            selected_location.set(Some((lat, lon)));
            is_loading.set(true);
            error.set(None);
            
            // Fetch forecast
            spawn({
                to_owned![forecast, is_loading, error, api_client, user_location];
                async move {
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
                }
            });
        }
    };
    
    let (lat, lon) = selected_location().unwrap_or((0.0, 0.0));
    let show_coords = selected_location().is_some();
    
    rsx! {
        div {
            class: "min-h-screen bg-slate-50 text-slate-900 flex flex-col",
            
            // Header
            header {
                class: "bg-white border-b border-slate-200 shadow-sm",
                div {
                    class: "max-w-7xl mx-auto px-4 py-3 flex items-center justify-between",
                    div {
                        class: "flex items-center gap-3",
                        h1 { class: "text-xl font-bold", "Прогноз Клювання 🎣" }
                    }
                }
            }
            
            // Main content
            main {
                class: "flex-1 max-w-7xl w-full mx-auto px-4 py-6",
                
                // Map
                div {
                    class: "mb-8",
                    MapView {
                        on_location_selected: on_location_selected.clone(),
                        selected_location: selected_location(),
                    }
                }
                
                // Info cards (when location selected)
                if show_coords {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-8",
                        
                        div {
                            class: "bg-white rounded-lg shadow p-4 border-l-4 border-blue-500",
                            p { class: "text-sm text-slate-600 mb-2", "Вибрана локація" }
                            p { class: "text-lg font-semibold font-mono", "{lat:.4}° N, {lon:.4}° E" }
                        }
                    }
                }
            }
            
            // Forecast panel
            ForecastPanel {
                forecast: forecast(),
                is_loading: is_loading(),
                error: error(),
            }
        }
    }
}

