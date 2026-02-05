//! Template engine for widget rendering
//! 
//! Supports variable interpolation with {variable} syntax
//! Examples:
//!   "CPU: {cpu_percent}% | Memory: {memory_percent}%"
//!   "WS {workspace_id} | {window_count} windows"

use anyhow::Result;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// Template variable source
#[derive(Debug, Clone)]
pub struct TemplateContext {
    /// Widget data variables
    pub widget_data: Value,

    /// Workspace variables
    pub workspace_id: Option<i32>,
    pub layout: Option<String>,
    pub window_count: Option<i32>,

    /// Custom variables
    pub custom: HashMap<String, String>,
}

impl TemplateContext {
    pub fn new(widget_data: Value) -> Self {
        Self {
            widget_data,
            workspace_id: None,
            layout: None,
            window_count: None,
            custom: HashMap::new(),
        }
    }

    /// Get variable value by name
    pub fn get_variable(&self, name: &str) -> Option<String> {
        // Check workspace variables first
        match name {
            "workspace_id" => self.workspace_id.map(|v| v.to_string()),
            "layout" => self.layout.clone(),
            "window_count" => self.window_count.map(|v| v.to_string()),
            _ => {
                // Check custom variables
                self.custom.get(name).cloned().or_else(|| {
                    // Check widget data
                    self.widget_data
                        .get(name)
                        .and_then(|v| match v {
                            Value::String(s) => Some(s.clone()),
                            Value::Number(n) => {
                                if let Some(f) = n.as_f64() {
                                    if f.fract() == 0.0 {
                                        Some(format!("{:.0}", f))
                                    } else {
                                        Some(format!("{:.2}", f))
                                    }
                                } else {
                                    Some(n.to_string())
                                }
                            }
                            Value::Bool(b) => Some(b.to_string()),
                            _ => None,
                        })
                })
            }
        }
    }
}

/// Template interpolator
pub struct Template {
    template_str: String,
    var_regex: Regex,
}

impl Template {
    pub fn new(template_str: &str) -> Result<Self> {
        Ok(Self {
            template_str: template_str.to_string(),
            var_regex: Regex::new(r"\{([^}]+)\}")?,
        })
    }

    /// Render template with context
    pub fn render(&self, context: &TemplateContext) -> String {
        self.var_regex
            .replace_all(&self.template_str, |caps: &regex::Captures| {
                let var_name = &caps[1];
                context
                    .get_variable(var_name)
                    .unwrap_or_else(|| format!("{{undefined:{}}}", var_name))
            })
            .to_string()
    }

    /// Check if template is valid
    pub fn is_valid(&self) -> bool {
        self.var_regex.is_match(&self.template_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_template_rendering() {
        let context = TemplateContext {
            widget_data: json!({
                "cpu_percent": 45.5,
                "memory_percent": 62.3
            }),
            workspace_id: Some(1),
            layout: Some("bsp".to_string()),
            window_count: Some(3),
            custom: HashMap::new(),
        };

        let template = Template::new("CPU: {cpu_percent}% | Memory: {memory_percent}%").unwrap();
        let result = template.render(&context);

        assert_eq!(result, "CPU: 45.50% | Memory: 62.30%");
    }

    #[test]
    fn test_workspace_variables() {
        let context = TemplateContext {
            widget_data: json!({}),
            workspace_id: Some(2),
            layout: Some("columns".to_string()),
            window_count: Some(5),
            custom: HashMap::new(),
        };

        let template =
            Template::new("WS {workspace_id} [{layout}] - {window_count} wins").unwrap();
        let result = template.render(&context);

        assert_eq!(result, "WS 2 [columns] - 5 wins");
    }

    #[test]
    fn test_undefined_variables() {
        let context = TemplateContext {
            widget_data: json!({"cpu": 50}),
            workspace_id: None,
            layout: None,
            window_count: None,
            custom: HashMap::new(),
        };

        let template = Template::new("CPU: {cpu} | Memory: {memory}").unwrap();
        let result = template.render(&context);

        assert!(result.contains("CPU: 50"));
        assert!(result.contains("undefined:memory"));
    }

    #[test]
    fn test_integer_formatting() {
        let context = TemplateContext {
            widget_data: json!({ "count": 42 }),
            workspace_id: None,
            layout: None,
            window_count: None,
            custom: HashMap::new(),
        };

        let template = Template::new("Count: {count}").unwrap();
        let result = template.render(&context);

        assert_eq!(result, "Count: 42");
    }

    #[test]
    fn test_custom_variables() {
        let mut custom = HashMap::new();
        custom.insert("user".to_string(), "admin".to_string());

        let context = TemplateContext {
            widget_data: json!({}),
            workspace_id: None,
            layout: None,
            window_count: None,
            custom,
        };

        let template = Template::new("User: {user}").unwrap();
        let result = template.render(&context);

        assert_eq!(result, "User: admin");
    }
}
