use crate::error::Result;
use crate::generator::{Generator, GeneratorContext};
use crate::template::{TemplateContext, TemplateEngine, templates};
use crate::utils::{
    ExecuterConfig, FSAction, execute_command, inject_git_dependency, run_with_spinner,
};

pub struct ProjectScaffolder;
type DataInjector = Box<dyn Fn(&mut TemplateContext)>;

impl Generator for ProjectScaffolder {
    fn run(&self, context: &GeneratorContext) -> Result<()> {
        let lib_path = context.base_path.join("lib");
        let engine = TemplateEngine::new("templates/**/*")?;

        let template_mappings: Vec<(&str, &str, DataInjector)> = vec![
            (
                templates::root::MAIN_COMMON,
                "main_common.dart",
                Box::new(|_| {}),
            ),
            (
                templates::root::MAIN_DEVELOPMENT,
                "main_development.dart",
                Box::new(|_| {}),
            ),
            (
                templates::root::MAIN_PRODUCTION,
                "main_production.dart",
                Box::new(|_| {}),
            ),
            (
                templates::core::APP_TEXT,
                "core/app/app_text.dart",
                Box::new(|_| {}),
            ),
            (
                templates::core::CONSTANTS,
                "core/app/constants.dart",
                Box::new(|_| {}),
            ),
            (templates::core::DI, "core/app/di.dart", Box::new(|_| {})),
            (
                templates::core::ENUM,
                "core/app/enum.dart",
                Box::new(|_| {}),
            ),
            (
                templates::core::AUTH_NOTIFIER,
                "core/auth/auth_notifier.dart",
                Box::new(|_| {}),
            ),
            (
                templates::core::FLAVOR,
                "core/config/flavor.dart",
                Box::new(|_| {}),
            ),
            (
                templates::core::LOGGER,
                "core/development/logger.dart",
                Box::new(|_| {}),
            ),
            (
                templates::route::NOT_FOUND,
                "core/route/not_found_screen.dart",
                Box::new(|_| {}),
            ),
            (
                templates::route::CONFIG,
                "core/route/route_config.dart",
                Box::new(|_| {}),
            ),
            (
                templates::route::GENERATOR,
                "core/route/route_generator.dart",
                Box::new(|_| {}),
            ),
            (
                templates::route::NAVIGATION,
                "core/route/route_navigation.dart",
                Box::new(|_| {}),
            ),
            (
                templates::theme::COLORS,
                "core/theme/app_colors.dart",
                Box::new(|ctx| {
                    ctx.insert("primary_hex", &"#4287f5");
                }),
            ),
            (
                templates::theme::TEXT_STYLE,
                "core/theme/app_text_style.dart",
                Box::new(|_| {}),
            ),
            (
                templates::theme::DIMENSION,
                "core/theme/dimension.dart",
                Box::new(|_| {}),
            ),
        ];
        for (template, output, data_injector) in template_mappings {
            let content = engine.render(template, |ctx| {
                ctx.insert("project_name", &context.project_name);
                data_injector(ctx);
            })?;

            FSAction::create_file(output, Some(&content)).execute(&lib_path)?;
        }

        let makefile_content = engine.render(templates::MAKEFILE, |ctx| {
            ctx.insert("project_name", &context.project_name);
        })?;
        FSAction::create_file("Makefile", Some(&makefile_content)).execute(&context.base_path)?;

        let dirs = vec![
            "features",
            "features/shared/model",
            "features/shared/widgets",
        ];

        for dir in dirs {
            FSAction::create_dir(dir).execute(&lib_path)?;
        }

        FSAction::remove_file("main.dart").execute(&lib_path)?;

        let template_base = "templates";
        let predefined_root = format!("{}/{}", template_base, templates::PREDEFINED_FEATURES_DIR);
        if std::path::Path::new(&predefined_root).exists() {
            for entry in walkdir::WalkDir::new(&predefined_root)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();

                if path.is_file() {
        
                    let relative = path.strip_prefix(&predefined_root).unwrap();
                    let template_key = format!(
                        "{}/{}",
                        templates::PREDEFINED_FEATURES_DIR,
                        relative.display()
                    );

                    let mut output_path = relative.to_path_buf();
                    output_path.set_extension(""); 
                    let final_output = format!("features/{}", output_path.display());

                    let content = engine.render(&template_key, |ctx| {
                        ctx.insert("project_name", &context.project_name);
                    })?;

                    FSAction::create_file(&final_output, Some(&content)).execute(&lib_path)?;
                }
            }
        }

        let mut executer_config = ExecuterConfig::default();
        executer_config.base_path = context.base_path.clone();
        run_with_spinner("Installing dependendy", || {
            execute_command(
                "flutter",
                &["pub", "add", "go_router", "get_it"],
                executer_config,
            )?;
            inject_git_dependency(
                &context.base_path,
                "zren",
                "https://github.com/Saugat913/zren.git",
            )?;

            let mut executer_config = ExecuterConfig::default();
            executer_config.base_path = context.base_path.clone();

            execute_command("flutter", &["pub", "get"], executer_config)?;
            return anyhow::Ok(());
        })?;

        println!("Installed dependency.");
        println!("Now follow below steps:");
        println!("  cd {}", &context.project_name);
        println!("  make");

        Ok(())
    }
}
