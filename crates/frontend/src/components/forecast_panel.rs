use dioxus::prelude::*;
use crate::services::api_client::ForecastResponse;

/// Component props for forecast panel
#[derive(Props, Clone, PartialEq)]
pub struct ForecastPanelProps {
    /// Forecast data to display
    #[props(default)]
    forecast: Option<ForecastResponse>,
    
    /// Whether forecast is loading
    #[props(default = true)]
    is_loading: bool,
    
    /// Error message if request failed
    #[props(default)]
    error: Option<String>,
}

/// Display forecast data content
#[component]
fn ForecastContent(forecast: ForecastResponse) -> Element {
    let temp = forecast.weather.temperature;
    let pressure = forecast.weather.pressure_msl;
    let wind = forecast.weather.wind_speed;
    let wind_dir = forecast.weather.wind_direction;
    let humidity = forecast.weather.humidity;
    let water_temp = forecast.weather.water_temperature;
    let wave_height = forecast.weather.wave_height;
    let visibility = forecast.weather.visibility;
    let sunrise = forecast.weather.sunrise.clone();
    let sunset = forecast.weather.sunset.clone();
    let moon_phase = forecast.moon_phase;
    let prob = forecast.probability * 100.0;
    let best_time = forecast.best_time.clone();
    
    // Wind direction as compass
    let wind_dir_text = match wind_dir {
        d if d >= 337.5 || d < 22.5 => "Пн",
        d if d >= 22.5 && d < 67.5 => "ПнСх",
        d if d >= 67.5 && d < 112.5 => "Сх",
        d if d >= 112.5 && d < 157.5 => "ПдСх",
        d if d >= 157.5 && d < 202.5 => "Пд",
        d if d >= 202.5 && d < 247.5 => "ПдЗх",
        d if d >= 247.5 && d < 292.5 => "Зх",
        _ => "ПнЗх",
    };
    
    // Moon phase emoji
    let moon_emoji = match moon_phase {
        p if p < 0.125 => "🌑",
        p if p < 0.25 => "🌒",
        p if p < 0.375 => "🌓",
        p if p < 0.5 => "🌔",
        p if p < 0.625 => "🌕",
        p if p < 0.75 => "🌖",
        p if p < 0.875 => "🌗",
        _ => "🌘",
    };
    
    let has_bait1 = forecast.recommended_baits.len() > 0;
    let has_bait2 = forecast.recommended_baits.len() > 1;
    let has_bait3 = forecast.recommended_baits.len() > 2;
    
    let bait1_name = if has_bait1 { 
        forecast.recommended_baits[0].name.clone()
    } else { 
        String::new() 
    };
    let bait1_eff = if has_bait1 { 
        forecast.recommended_baits[0].effectiveness * 100.0 
    } else { 
        0.0 
    };
    
    let bait2_name = if has_bait2 { 
        forecast.recommended_baits[1].name.clone() 
    } else { 
        String::new() 
    };
    let bait2_eff = if has_bait2 { 
        forecast.recommended_baits[1].effectiveness * 100.0 
    } else { 
        0.0 
    };
    
    let bait3_name = if has_bait3 { 
        forecast.recommended_baits[2].name.clone() 
    } else { 
        String::new() 
    };
    let bait3_eff = if has_bait3 { 
        forecast.recommended_baits[2].effectiveness * 100.0 
    } else { 
        0.0 
    };
    
    rsx! {
        div { class: "space-y-6",
            // Main forecast
            div { class: "flex items-center justify-between bg-gradient-to-r from-blue-50 to-cyan-50 p-4 rounded-lg",
                div {
                    h3 { class: "text-sm text-slate-600 mb-1", "🎣 Прогноз клювання" }
                    p { class: "text-4xl font-bold text-blue-600", "{prob:.0}%" }
                    p { class: "text-xs text-slate-500 mt-1", "⏰ {best_time}" }
                }
                div { class: "text-5xl", "{moon_emoji}" }
            }
            
            // Weather conditions
            div { class: "bg-white rounded-lg p-4 shadow-sm",
                h4 { class: "text-sm font-semibold text-slate-700 mb-3 flex items-center gap-2",
                    span { "🌤️" }
                    "Погодні умови"
                }
                div { class: "grid grid-cols-2 gap-3 text-sm",
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "🌡️" }
                        div {
                            p { class: "text-xs text-slate-500", "Температура повітря" }
                            p { class: "font-semibold", "{temp:.1}°C" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "💧" }
                        div {
                            p { class: "text-xs text-slate-500", "Температура води" }
                            p { class: "font-semibold text-blue-600", "{water_temp:.1}°C" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "🌬️" }
                        div {
                            p { class: "text-xs text-slate-500", "Вітер" }
                            p { class: "font-semibold", "{wind:.1} м/с {wind_dir_text}" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "💨" }
                        div {
                            p { class: "text-xs text-slate-500", "Тиск" }
                            p { class: "font-semibold", "{pressure:.0} гПа" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "💦" }
                        div {
                            p { class: "text-xs text-slate-500", "Вологість" }
                            p { class: "font-semibold", "{humidity:.0}%" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "🌊" }
                        div {
                            p { class: "text-xs text-slate-500", "Висота хвиль" }
                            p { class: "font-semibold", "{wave_height:.1} м" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "👁️" }
                        div {
                            p { class: "text-xs text-slate-500", "Видимість" }
                            p { class: "font-semibold", "{visibility:.1} км" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-lg", "☀️" }
                        div {
                            p { class: "text-xs text-slate-500", "Світло" }
                            p { class: "font-semibold text-xs", "↑{sunrise} ↓{sunset}" }
                        }
                    }
                }
            }
            
            // Baits
            div { class: "bg-white rounded-lg p-4 shadow-sm",
                h4 { class: "text-sm font-semibold text-slate-700 mb-3 flex items-center gap-2",
                    span { "🪱" }
                    "Рекомендовані приманки"
                }
                div { class: "space-y-2",
                    if has_bait1 {
                        div { class: "flex justify-between items-center bg-gradient-to-r from-green-50 to-emerald-50 p-3 rounded-lg",
                            div { class: "flex items-center gap-2",
                                span { class: "text-2xl", "🥇" }
                                p { class: "font-medium", "{bait1_name}" }
                            }
                            p { class: "text-green-600 font-bold", "{bait1_eff:.0}%" }
                        }
                    }
                    if has_bait2 {
                        div { class: "flex justify-between items-center bg-slate-50 p-3 rounded-lg",
                            div { class: "flex items-center gap-2",
                                span { class: "text-2xl", "🥈" }
                                p { class: "font-medium", "{bait2_name}" }
                            }
                            p { class: "text-slate-600 font-semibold", "{bait2_eff:.0}%" }
                        }
                    }
                    if has_bait3 {
                        div { class: "flex justify-between items-center bg-slate-50 p-3 rounded-lg",
                            div { class: "flex items-center gap-2",
                                span { class: "text-2xl", "🥉" }
                                p { class: "font-medium", "{bait3_name}" }
                            }
                            p { class: "text-slate-600 font-semibold", "{bait3_eff:.0}%" }
                        }
                    }
                }
            }
        }
    }
}

/// Sliding forecast panel component
///
/// Displays bite probability, weather, and recommended baits.
/// Simple version that shows forecast data when available.
#[component]
pub fn ForecastPanel(props: ForecastPanelProps) -> Element {
    let mut is_open = use_signal(|| true);
    
    rsx! {
        div {
            class: "fixed bottom-0 left-0 right-0 z-40",
            
            div {
                class: "relative bg-white rounded-t-2xl shadow-2xl p-6 transition-all duration-300 max-h-[80vh] overflow-y-auto",
                
                // Header with close button
                div {
                    class: "flex justify-between items-center mb-4 pb-4 border-b border-slate-200",
                    
                    div {
                        class: "flex items-center gap-2",
                        span { class: "text-2xl", "🎣" }
                        h2 { class: "text-lg font-bold text-slate-800", "Прогноз клювання" }
                    }
                    
                    button {
                        class: "text-2xl hover:bg-slate-100 rounded-full p-1 transition-colors",
                        onclick: move |_| *is_open.write() = false,
                        "✕"
                    }
                }
                
                // Content
                if *is_open.read() {
                    {props.forecast.clone().map(|forecast| rsx! {
                        ForecastContent { forecast }
                    })}
                    
                    if props.is_loading {
                        div {
                            class: "text-center py-8",
                            p { class: "text-slate-500 animate-pulse", "⏳ Завантаження прогнозу..." }
                        }
                    }
                    
                    if let Some(err) = &props.error {
                        div {
                            class: "bg-red-50 border border-red-200 p-3 rounded-lg",
                            p { class: "text-red-700 text-sm", "❌ Помилка: {err}" }
                        }
                    }
                }
            }
        }
    }
}
