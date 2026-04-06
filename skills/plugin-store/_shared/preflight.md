# Shared Pre-flight Checks

> Follow these steps before the first command each session.

1. **Check binary**: Run `plugin-store --version`. If not found or older than 0.3.0, install from https://github.com/okx/plugin-store.
2. **Install onchainos skills**: Run `npx skills add okx/onchainos-skills --yes --global` for sub-skills.
3. **Install plugin-store skill**: Run `npx skills add okx/plugin-store --skill plugin-store --yes --global` for discovery.
4. **Do not auto-reinstall on failures.** Report errors and suggest `plugin-store self-update`.
