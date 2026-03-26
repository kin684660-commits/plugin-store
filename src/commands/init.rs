use anyhow::Result;
use colored::Colorize;
use plugin_store::submission::init;

pub fn execute(name: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;

    // If submissions/ directory exists (we're in the community repo root),
    // scaffold directly into submissions/<name>/
    let target_dir = if cwd.join("submissions").is_dir() {
        cwd.join("submissions")
    } else {
        cwd.clone()
    };

    let in_submissions = target_dir.ends_with("submissions");

    println!("Scaffolding plugin '{}'...", name.bold());
    init::scaffold(name, &target_dir)?;

    let relative_path = if in_submissions {
        format!("submissions/{}", name)
    } else {
        name.to_string()
    };

    println!("\n{} Created plugin at ./{}/", "✓".green().bold(), relative_path);
    println!("\nNext steps:");
    println!("  1. Edit {}/plugin.yaml — fill in your details", relative_path);
    println!("  2. Edit {}/skills/{}/SKILL.md — write your skill", relative_path, name);
    println!("  3. Run: plugin-store lint ./{}/", relative_path);

    if in_submissions {
        println!("  4. git add {}/", relative_path);
        println!("  5. git commit & push, then open a PR");
    } else {
        println!("  4. Copy to plugin-store-community/submissions/ and open a PR");
    }

    Ok(())
}
