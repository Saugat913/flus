use crate::error::Result;
use serde::Serialize;
use tera::{Context, Tera};

pub mod templates {
    pub const MAKEFILE: &str = "Makefile.tera";

    pub mod root {
        pub const MAIN_COMMON: &str = "root/main_common.dart.tera";
        pub const MAIN_DEVELOPMENT: &str = "root/main_development.dart.tera";
        pub const MAIN_PRODUCTION: &str = "root/main_production.dart.tera";
    }

    pub mod core {
        pub const APP_TEXT: &str = "core/app/app_text.dart.tera";
        pub const CONSTANTS: &str = "core/app/constants.dart.tera";
        pub const DI: &str = "core/app/di.dart.tera";
        pub const ENUM: &str = "core/app/enum.dart.tera";
        
        pub const AUTH_NOTIFIER: &str = "core/auth/auth_notifier.dart.tera";
        pub const FLAVOR: &str = "core/config/flavor.dart.tera";
        pub const LOGGER: &str = "core/development/logger.dart.tera";
    }

    pub mod route {
        pub const NOT_FOUND: &str = "core/route/not_found_screen.dart.tera";
        pub const CONFIG: &str = "core/route/route_config.dart.tera";
        pub const GENERATOR: &str = "core/route/route_generator.dart.tera";
        pub const NAVIGATION: &str = "core/route/route_navigation.dart.tera";
    }

    pub mod theme {
        pub const COLORS: &str = "core/theme/app_colors.dart.tera";
        pub const TEXT_STYLE: &str = "core/theme/app_text_style.dart.tera";
        pub const DIMENSION: &str = "core/theme/dimension.dart.tera";
    }

    pub const PREDEFINED_FEATURES_DIR: &str = "features/predefined";
}



pub struct TemplateEngine {
    tera: Tera,
}

pub struct TemplateContext {
    context: tera::Context,
}

impl TemplateContext {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }

    pub fn insert<V: Serialize>(&mut self, key: &str, value: &V) {
        self.context.insert(key, value);
    }
}
impl TemplateEngine {
    pub fn new(template_path: &str) -> Result<Self> {
        let tera = Tera::new(template_path)?;
        return Ok(Self { tera: tera });
    }

    pub fn render<F>(&self, template_name: &str, context_builder: F) -> Result<String>
    where
        F: FnOnce(&mut TemplateContext),
    {
        let mut context = TemplateContext::new();
        context_builder(&mut context);
        let rendered_text = self.tera.render(template_name, &context.context)?;
        return Ok(rendered_text);
    }
}
