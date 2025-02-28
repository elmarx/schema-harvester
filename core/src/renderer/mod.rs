mod json_schema_renderer;
mod string;

pub use json_schema_renderer::render_schema;

trait Render {
    fn render(&self) -> serde_json::Value;
}
