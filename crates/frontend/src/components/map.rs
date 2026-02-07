use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn initLeafletMap(container_id: &str, lat: f64, lon: f64, zoom: i32);
    
    #[wasm_bindgen(js_namespace = window)]
    fn updateLeafletMarker(lat: f64, lon: f64);
    
    #[wasm_bindgen(js_namespace = window)]
    fn updateUserMarker(lat: f64, lon: f64, heading: f64, wind_direction: f64);
    
    #[wasm_bindgen(js_namespace = window)]
    fn setLeafletClickHandler(callback: &Closure<dyn Fn(f64, f64)>);
}

/// Component props for the map view
#[derive(Props, Clone, PartialEq)]
pub struct MapViewProps {
    /// Callback when user clicks on map
    pub on_location_selected: EventHandler<(f64, f64)>,
    
    /// Selected location (lat, lon) for the forecast
    #[props(default)]
    pub selected_location: Option<(f64, f64)>,
}

/// Interactive map component with Leaflet integration
/// 
/// Displays an interactive OpenStreetMap with water bodies and marker for selected location.
/// Allows user to click on map to select location for forecast.
#[component]
pub fn MapView(props: MapViewProps) -> Element {
    let on_click = props.on_location_selected.clone();
    let mut map_initialized = use_signal(|| false);
    let mut user_location = use_signal(|| (50.45, 30.52)); // Kyiv default
    
    // Initialize map on first render
    use_effect(move || {
        if !*map_initialized.read() {
            let init_lat = 49.0;
            let init_lon = 31.0;
            let init_zoom = 6;
            
            web_sys::console::log_1(&"🗺️ Rust: Starting map initialization".into());
            
            // Initialize Leaflet map
            initLeafletMap("leaflet-map", init_lat, init_lon, init_zoom);
            
            // Setup click handler
            let callback = Closure::wrap(Box::new(move |lat: f64, lon: f64| {
                on_click.call((lat, lon));
            }) as Box<dyn Fn(f64, f64)>);
            
            setLeafletClickHandler(&callback);
            callback.forget(); // Keep callback alive
            
            web_sys::console::log_1(&"✅ Rust: Map initialized".into());
            
            *map_initialized.write() = true;
            
            // Let JavaScript handle geolocation and user marker
            // It will call updateUserMarker automatically
        }
    });
    
    // Update marker when location changes
    use_effect(move || {
        if let Some((lat, lon)) = props.selected_location {
            if *map_initialized.read() {
                updateLeafletMarker(lat, lon);
            }
        }
    });
    
    rsx! {
        div {
            class: "w-full h-full rounded-lg shadow-md relative",
            style: "width: 100%; height: 100%; position: relative;",
            
            // Інформаційна панель
            div {
                class: "absolute top-2 left-2 bg-white bg-opacity-95 px-3 py-2 rounded-lg shadow-lg z-[1000] border border-blue-200",
                style: "position: absolute; top: 8px; left: 8px; z-index: 1000;",
                div {
                    class: "font-bold text-sm text-blue-700",
                    "🗺️ Карта лову"
                }
                div {
                    class: "text-xs text-gray-600 mt-1",
                    "Клікніть на карту для вибору локації"
                }
                {props.selected_location.map(|(lat, lon)| rsx! {
                    div {
                        class: "text-xs text-blue-600 mt-1 font-mono",
                        "📍 {lat:.2}°, {lon:.2}°"
                    }
                })}
            }
            
            // Leaflet map container - fullscreen
            div {
                id: "leaflet-map",
                class: "w-full h-full rounded-lg",
                style: "height: 100%; width: 100%; z-index: 1;"
            }
        }
    }
}
