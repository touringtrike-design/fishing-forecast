use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::cell::RefCell;

/// Initialize MapLibre GL map with click handlers
/// Returns Ok on success, Err with message on failure
pub async fn init_map(
    container_id: &str,
    on_click: Box<dyn Fn(f64, f64)>,
) -> Result<(), String> {
    // Get window and document
    let window = web_sys::window().ok_or("No window object")?;
    let document = window.document().ok_or("No document object")?;
    
    // Get container element
    let container = document
        .get_element_by_id(container_id)
        .ok_or(format!("Element with id {} not found", container_id))?;
    
    // Initialize simple map using HTML canvas or div-based approach
    // For now, we'll use a simple approach with data attributes
    // In production, you'd load MapLibre GL via CDN or bundler
    
    init_simple_map(&container, on_click).await
}

/// Initialize a simple map with basic tile layer
/// This is a simplified version without MapLibre GL library
/// In production, you'd include MapLibre GL and use its API
async fn init_simple_map(
    container: &web_sys::Element,
    on_click: Box<dyn Fn(f64, f64)>,
) -> Result<(), String> {
    // Cast to HtmlElement
    let html_element = container
        .dyn_ref::<HtmlElement>()
        .ok_or("Container is not an HTML element")?;
    
    // Set up basic map styling
    html_element.set_attribute("data-map-initialized", "true")
        .map_err(|_| "Failed to set attribute".to_string())?;
    
    // Set innerHTML to show a basic map placeholder with grid
    let map_html = r#"
        <div style="width: 100%; height: 100%; position: relative; background: linear-gradient(45deg, #e8f4f8 25%, transparent 25%, transparent 75%, #e8f4f8 75%, #e8f4f8), 
                    linear-gradient(45deg, #e8f4f8 25%, transparent 25%, transparent 75%, #e8f4f8 75%, #e8f4f8);
                    background-size: 40px 40px; background-position: 0 0, 20px 20px; background-color: #f0f8ff;">
            <div style="position: absolute; top: 10px; left: 10px; background: white; padding: 10px; border-radius: 5px; font-size: 12px; z-index: 10;">
                <div style="margin-bottom: 5px;"><strong>Клік на карті</strong></div>
                <div style="font-size: 11px; color: #666;">Натисніть для вибору локації</div>
            </div>
            <div style="position: absolute; bottom: 10px; left: 10px; background: white; padding: 8px; border-radius: 5px; font-size: 11px; z-index: 10;">
                © OpenStreetMap
            </div>
        </div>
    "#;
    
    container.set_inner_html(map_html);
    
    // Add click handler to container
    add_map_click_handler(container, on_click).await?;
    
    Ok(())
}

/// Add click event listener to map container
async fn add_map_click_handler(
    container: &web_sys::Element,
    on_click: Box<dyn Fn(f64, f64)>,
) -> Result<(), String> {
    use wasm_bindgen::prelude::*;
    use web_sys::MouseEvent;
    
    let container_copy = container.clone();
    let on_click_rc = Rc::new(RefCell::new(on_click));
    let on_click_copy = on_click_rc.clone();
    
    // Create closure for click handler
    let on_click_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        // Get click position relative to container
        if let Some(html_elem) = container_copy.dyn_ref::<HtmlElement>() {
            let rect = html_elem.get_bounding_client_rect();
            
            let x = (event.client_x() as f64 - rect.left()) / rect.width();
            let y = (event.client_y() as f64 - rect.top()) / rect.height();
            
            // Convert pixel position to lat/lon
            // This is a simple mercator projection approximation
            // Centered on Europe/Africa region
            let lat = 20.0 + (1.0 - y) * 40.0;  // Range: -20 to 60
            let lon = -20.0 + x * 80.0;          // Range: -20 to 60
            
            let on_click_fn = on_click_copy.borrow();
            on_click_fn(lat, lon);
        }
    }) as Box<dyn FnMut(MouseEvent)>);
    
    // Add event listener
    container
        .add_event_listener_with_callback("click", on_click_closure.as_ref().unchecked_ref())
        .map_err(|_| "Failed to add event listener".to_string())?;
    
    // Leak the closure to keep it alive
    on_click_closure.forget();
    
    Ok(())
}

/// Update marker position on map
pub fn update_marker(lat: f64, lon: f64) -> Result<(), String> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    
    let container = document
        .get_element_by_id("map-container")
        .ok_or("Map container not found")?;
    
    // Get or create marker element
    let marker = if let Some(existing) = document.get_element_by_id("map-marker") {
        existing
    } else {
        let marker = document
            .create_element("div")
            .map_err(|_| "Failed to create marker")?;
        marker.set_id("map-marker");
        container
            .append_child(&marker)
            .map_err(|_| "Failed to append marker")?;
        marker
    };
    
    // Position marker based on lat/lon
    let x_percent = ((lon + 20.0) / 80.0) * 100.0;
    let y_percent = ((60.0 - lat) / 40.0) * 100.0;
    
    if let Some(html_elem) = marker.dyn_ref::<HtmlElement>() {
        html_elem.set_attribute("style", 
            &format!(
                "position: absolute; left: {}%; top: {}%; width: 24px; height: 24px; \
                 transform: translate(-50%, -50%); z-index: 20; \
                 background: #ff4444; border: 3px solid white; border-radius: 50%; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.3); cursor: pointer;",
                x_percent, y_percent
            ))
            .map_err(|_| "Failed to set marker style".to_string())?;
    }
    
    Ok(())
}
