//! Widget export formats for Zebar integration

use crate::WidgetRuntime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Export format
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    JSON,
    HTML,
    CSS,
    Zebar,
}

impl ExportFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::JSON => "json",
            ExportFormat::HTML => "html",
            ExportFormat::CSS => "css",
            ExportFormat::Zebar => "json",
        }
    }
}

/// Zebar widget definition (compatible with Zebar renderer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZebarWidget {
    pub id: String,
    pub r#type: String,
    pub template: String,
    pub data: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<HashMap<String, String>>,
}

/// Zebar export payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZebarExport {
    pub version: String,
    pub timestamp: String,
    pub widgets: Vec<ZebarWidget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<String>,
}

/// HTML export structure
#[derive(Debug)]
pub struct HtmlExport {
    pub html: String,
    pub css: String,
}

/// Export engine
pub struct Exporter;

impl Exporter {
    /// Export widgets to JSON format
    pub fn to_json(widgets: &[WidgetRuntime]) -> Result<String, serde_json::Error> {
        let data: Vec<Value> = widgets
            .iter()
            .map(|w| {
                json!({
                    "id": w.config.id,
                    "type": w.config.widget_type,
                    "template": w.config.template,
                    "data": w.last_data,
                    "last_update": w.last_update,
                    "error": w.error,
                })
            })
            .collect();

        serde_json::to_string_pretty(&data)
    }

    /// Export widgets to Zebar format
    pub fn to_zebar(widgets: &[WidgetRuntime]) -> Result<String, serde_json::Error> {
        let zebar_widgets: Vec<ZebarWidget> = widgets
            .iter()
            .filter_map(|w| {
                // Only export supported widget types
                let widget_type = match w.config.widget_type.as_str() {
                    "System" => "system-metrics",
                    "Process" => "process-monitor",
                    _ => return None,
                };

                Some(ZebarWidget {
                    id: w.config.id.clone(),
                    r#type: widget_type.to_string(),
                    template: w.config.template.clone(),
                    data: w.last_data.clone(),
                    styles: None,
                })
            })
            .collect();

        let export = ZebarExport {
            version: "1.0".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            widgets: zebar_widgets,
            styles: Some(Self::default_css()),
        };

        serde_json::to_string_pretty(&export)
    }

    /// Export widgets to HTML
    pub fn to_html(widgets: &[WidgetRuntime]) -> HtmlExport {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>YASB Widgets Export</title>
    <style>
"#,
        );

        html.push_str(&Self::default_css());
        html.push_str(
            r#"    </style>
</head>
<body>
    <div class="widgets-container">
"#,
        );

        for widget in widgets {
            html.push_str(&format!(
                r#"        <div class="widget" id="widget-{}">
            <h3 class="widget-title">{}</h3>
            <div class="widget-content">{}</div>
        </div>
"#,
                widget.config.id,
                widget.config.id,
                Self::render_widget_html(&widget.last_data)
            ));
        }

        html.push_str(
            r#"    </div>
</body>
</html>"#,
        );

        HtmlExport {
            html: html.clone(),
            css: Self::default_css(),
        }
    }

    /// Export to CSS snippet
    pub fn to_css() -> String {
        Self::default_css()
    }

    /// Render widget data to HTML
    fn render_widget_html(data: &Value) -> String {
        if let Some(obj) = data.as_object() {
            let mut html = String::from("<dl>");

            for (key, value) in obj {
                let formatted = match value {
                    Value::Number(n) => {
                        if let Some(f) = n.as_f64() {
                            if f.fract() == 0.0 {
                                format!("{:.0}", f)
                            } else {
                                format!("{:.2}", f)
                            }
                        } else {
                            n.to_string()
                        }
                    }
                    Value::Bool(b) => b.to_string(),
                    Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };

                html.push_str(&format!(
                    "<dt>{}</dt><dd>{}</dd>",
                    key.replace('_', " "),
                    formatted
                ));
            }

            html.push_str("</dl>");
            html
        } else {
            String::new()
        }
    }

    /// Default CSS for exported widgets
    pub fn default_css() -> String {
        String::from(
            r#":root {
    --bg-color: rgba(10, 10, 15, 0.95);
    --text-color: #ffffff;
    --accent-color: #00ff00;
    --border-color: rgba(0, 255, 0, 0.3);
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', sans-serif;
    background: var(--bg-color);
    color: var(--text-color);
    padding: 12px;
}

.widgets-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.widget {
    padding: 10px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    transition: all 0.2s ease;
}

.widget:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: var(--accent-color);
}

.widget-title {
    color: var(--accent-color);
    margin-bottom: 6px;
    font-size: 13px;
    font-weight: bold;
}

.widget-content {
    font-size: 11px;
}

.widget-content dl {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4px;
}

.widget-content dt {
    color: #aaa;
    font-size: 10px;
    text-transform: uppercase;
}

.widget-content dd {
    font-weight: bold;
    color: var(--accent-color);
}
"#,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::JSON.extension(), "json");
        assert_eq!(ExportFormat::Zebar.extension(), "json");
        assert_eq!(ExportFormat::HTML.extension(), "html");
    }

    #[test]
    fn test_html_render_with_metrics() {
        let data = json!({
            "cpu_percent": 45.5,
            "memory_percent": 62.3,
        });

        let html = Exporter::render_widget_html(&data);
        assert!(html.contains("cpu percent"));
        assert!(html.contains("45.50"));
        assert!(html.contains("memory percent"));
    }

    #[test]
    fn test_zebar_widget_serialization() {
        let widget = ZebarWidget {
            id: "test-widget".to_string(),
            r#type: "system-metrics".to_string(),
            template: "CPU: {cpu}%".to_string(),
            data: json!({"cpu": 45}),
            styles: None,
        };

        let json_str = serde_json::to_string(&widget).unwrap();
        let parsed: ZebarWidget = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed.id, "test-widget");
        assert_eq!(parsed.r#type, "system-metrics");
    }

    #[test]
    fn test_css_export() {
        let css = Exporter::to_css();
        assert!(css.contains("--accent-color"));
        assert!(css.contains(".widget"));
        assert!(css.len() > 100);
    }
}
