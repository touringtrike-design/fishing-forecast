use dioxus::prelude::*;
use crate::services::api_client::{ApiClient, CatchRecord};
use chrono::Utc;

#[derive(Clone, Debug, PartialEq)]
pub struct FishSpecies {
    pub id: String,
    pub name: String,
    pub scientific_name: String,
}

#[component]
pub fn CatchForm(
    api_client: Signal<ApiClient>,
    user_location: Signal<Option<(f64, f64)>>,
    on_close: EventHandler<()>,
    on_submit: EventHandler<CatchRecord>,
) -> Element {
    let mut fish_species = use_signal(|| Vec::<FishSpecies>::new());
    let mut selected_fish = use_signal(|| String::new());
    let mut weight = use_signal(|| String::new());
    let mut length = use_signal(|| String::new());
    let mut bait = use_signal(|| String::new());
    let mut bite_intensity = use_signal(|| 3u8);
    let mut notes = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    // Load fish species on mount
    use_effect(move || {
        spawn(async move {
            match api_client.read().get_fish_species("UA", "uk").await {
                Ok(species_list) => {
                    let species: Vec<FishSpecies> = species_list
                        .into_iter()
                        .map(|s| FishSpecies {
                            id: s.id,
                            name: s.name,
                            scientific_name: s.scientific_name.unwrap_or_default(),
                        })
                        .collect();
                    fish_species.set(species);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to load fish species: {}", e)));
                }
            }
        });
    });

    let handle_submit = move |_| {
        let location = match *user_location.read() {
            Some((lat, lon)) => (lat, lon),
            None => {
                error.set(Some("Location not available. Please enable geolocation.".to_string()));
                return;
            }
        };

        let fish = selected_fish.read().clone();
        if fish.is_empty() {
            error.set(Some("Please select a fish species".to_string()));
            return;
        }

        let weight_val = weight.read().parse::<f64>().ok();
        let length_val = length.read().parse::<f64>().ok();
        let bait_val = bait.read().clone();
        
        if bait_val.is_empty() {
            error.set(Some("Please enter the bait used".to_string()));
            return;
        }

        is_loading.set(true);

        let catch_record = CatchRecord {
            id: String::new(), // Will be set by backend
            user_id: "default_user".to_string(), // TODO: Replace with actual user ID after auth
            lat: location.0,
            lon: location.1,
            caught_at: Utc::now().to_rfc3339(),
            fish_species: fish,
            weight_kg: weight_val,
            length_cm: length_val,
            bait_used: Some(bait_val),
            weather_temp: None, // TODO: Add from forecast
            weather_pressure: None, // TODO: Add from forecast
            moon_phase: None, // TODO: Add from forecast
            notes: if notes.read().is_empty() {
                None
            } else {
                Some(notes.read().clone())
            },
            photo_url: None,
        };

        spawn(async move {
            match api_client.read().save_catch(&catch_record).await {
                Ok(_) => {
                    on_submit.call(catch_record.clone());
                    is_loading.set(false);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to save catch: {}", e)));
                    is_loading.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            class: "bg-white rounded-lg shadow-xl max-w-md w-full",

            // Header
            div {
                class: "flex justify-between items-center p-4 border-b",
                h2 {
                    class: "text-xl font-bold text-gray-800",
                    "🎣 Зареєструвати улов"
                }
                button {
                    class: "text-gray-500 hover:text-gray-700 text-2xl",
                    onclick: move |_| on_close.call(()),
                    "×"
                }
            }

            // Form
            div {
                class: "p-4 space-y-4",

                    // Error message
                    if let Some(err) = error.read().as_ref() {
                        div {
                            class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
                            "{err}"
                        }
                    }

                    // Fish species dropdown
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Вид риби *"
                        }
                        select {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            value: "{selected_fish}",
                            onchange: move |e| selected_fish.set(e.value()),
                            
                            option {
                                value: "",
                                "Виберіть вид..."
                            }
                            for species in fish_species.read().iter() {
                                option {
                                    value: "{species.id}",
                                    "{species.name} ({species.scientific_name})"
                                }
                            }
                        }
                    }

                    // Weight
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Вага (кг)"
                        }
                        input {
                            r#type: "number",
                            step: "0.1",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "0.5",
                            value: "{weight}",
                            oninput: move |e| weight.set(e.value()),
                        }
                    }

                    // Length
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Довжина (см)"
                        }
                        input {
                            r#type: "number",
                            step: "1",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "25",
                            value: "{length}",
                            oninput: move |e| length.set(e.value()),
                        }
                    }

                    // Bait
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Наживка *"
                        }
                        input {
                            r#type: "text",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "Черв'як, кукурудза...",
                            value: "{bait}",
                            oninput: move |e| bait.set(e.value()),
                        }
                    }

                    // Bite intensity
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Інтенсивність клювання: {bite_intensity}"
                        }
                        div {
                            class: "flex items-center space-x-2",
                            input {
                                r#type: "range",
                                min: "1",
                                max: "5",
                                class: "w-full",
                                value: "{bite_intensity}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<u8>() {
                                        bite_intensity.set(val);
                                    }
                                },
                            }
                            span {
                                class: "text-2xl",
                                match *bite_intensity.read() {
                                    1 => "😴",
                                    2 => "😐",
                                    3 => "🙂",
                                    4 => "😊",
                                    5 => "🤩",
                                    _ => "🙂"
                                }
                            }
                        }
                    }

                    // Notes
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Нотатки"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            rows: "3",
                            placeholder: "Додаткова інформація...",
                            value: "{notes}",
                            oninput: move |e| notes.set(e.value()),
                        }
                    }

                    // Location info
                    if let Some((lat, lon)) = *user_location.read() {
                        div {
                            class: "text-sm text-gray-600 bg-gray-50 p-2 rounded",
                            "📍 Локація: {lat:.4}°, {lon:.4}°"
                        }
                    }

                    // Submit button
                    div {
                        class: "flex space-x-3",
                        button {
                            class: "flex-1 bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed",
                            disabled: *is_loading.read(),
                            onclick: handle_submit,
                            if *is_loading.read() {
                                "Збереження..."
                            } else {
                                "💾 Зберегти улов"
                            }
                        }
                        button {
                            class: "px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50",
                            onclick: move |_| on_close.call(()),
                            "Скасувати"
                        }
                    }
            }
        }
    }
}