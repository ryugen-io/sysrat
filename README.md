# sysrat - codebase

```
sysrat-rs/
├── .github/
│   ├── logs/
│   │   └── 20260111-210247-workflow.log
│   ├── skips/
│   │   └── .skip.update-readme.example
│   └── workflows/
│       ├── scripts/
│       │   ├── ci-lines.sh
│       │   ├── ci-logger.sh
│       │   └── update_readme.py
│       ├── check-skip.yml
│       └── update-readme.yml
├── .serena/
│   ├── memories/
│   │   ├── code_style_and_conventions.md
│   │   ├── codebase_structure.md
│   │   ├── project_overview.md
│   │   ├── suggested_commands.md
│   │   └── task_completion_checklist.md
│   └── project.yml
├── core/
│   ├── src/
│   │   ├── config/
│   │   │   ├── app_config.rs
│   │   │   ├── mod.rs
│   │   │   ├── models.rs
│   │   │   └── scanner.rs
│   │   ├── configs/
│   │   │   ├── actions.rs
│   │   │   ├── mod.rs
│   │   │   └── validation.rs
│   │   ├── containers/
│   │   │   ├── actions.rs
│   │   │   └── mod.rs
│   │   ├── lib.rs
│   │   └── types.rs
│   └── Cargo.toml
├── frontend/
│   ├── assets/
│   │   ├── menu-text.ascii
│   │   └── sysrat.ascii
│   ├── build_helpers/
│   │   ├── theme/
│   │   │   ├── generator.rs
│   │   │   ├── mod.rs
│   │   │   └── scanner.rs
│   │   ├── ascii.rs
│   │   ├── date.rs
│   │   ├── hash.rs
│   │   ├── keybinds.rs
│   │   ├── mod.rs
│   │   ├── statusline.rs
│   │   └── version.rs
│   ├── src/
│   │   ├── api/
│   │   │   ├── configs.rs
│   │   │   ├── containers.rs
│   │   │   ├── mod.rs
│   │   │   └── types.rs
│   │   ├── events/
│   │   │   ├── container_list/
│   │   │   │   ├── actions.rs
│   │   │   │   ├── details.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── navigation.rs
│   │   │   ├── editor/
│   │   │   │   ├── normal_mode/
│   │   │   │   │   ├── editing.rs
│   │   │   │   │   ├── insert_commands.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── navigation.rs
│   │   │   │   ├── input.rs
│   │   │   │   ├── insert_mode.rs
│   │   │   │   └── mod.rs
│   │   │   ├── file_list.rs
│   │   │   ├── menu.rs
│   │   │   └── mod.rs
│   │   ├── keybinds/
│   │   │   ├── help_text.rs
│   │   │   ├── mod.rs
│   │   │   └── types.rs
│   │   ├── state/
│   │   │   ├── refresh/
│   │   │   │   ├── cache.rs
│   │   │   │   ├── container_list.rs
│   │   │   │   ├── file_list.rs
│   │   │   │   └── mod.rs
│   │   │   ├── app.rs
│   │   │   ├── container_list.rs
│   │   │   ├── editor.rs
│   │   │   ├── file_list.rs
│   │   │   ├── menu.rs
│   │   │   ├── mod.rs
│   │   │   ├── pane.rs
│   │   │   ├── splash.rs
│   │   │   └── status_helper.rs
│   │   ├── storage/
│   │   │   ├── generic.rs
│   │   │   ├── local.rs
│   │   │   ├── mod.rs
│   │   │   └── types.rs
│   │   ├── theme/
│   │   │   ├── types/
│   │   │   │   ├── colors.rs
│   │   │   │   ├── config.rs
│   │   │   │   ├── font.rs
│   │   │   │   ├── icons.rs
│   │   │   │   └── mod.rs
│   │   │   ├── builder.rs
│   │   │   ├── container_list.rs
│   │   │   ├── editor.rs
│   │   │   ├── file_list.rs
│   │   │   ├── loader.rs
│   │   │   ├── menu.rs
│   │   │   ├── mod.rs
│   │   │   └── status_line.rs
│   │   ├── ui/
│   │   │   ├── container_details/
│   │   │   │   ├── basic.rs
│   │   │   │   ├── config.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── network.rs
│   │   │   │   └── storage.rs
│   │   │   ├── menu/
│   │   │   │   ├── center.rs
│   │   │   │   ├── keybinds.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── rat_ascii.rs
│   │   │   ├── status_line/
│   │   │   │   ├── components/
│   │   │   │   │   ├── build.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── state.rs
│   │   │   │   │   └── text.rs
│   │   │   │   ├── rendering/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── spacing.rs
│   │   │   │   ├── config.rs
│   │   │   │   └── mod.rs
│   │   │   ├── container_list.rs
│   │   │   ├── editor.rs
│   │   │   ├── file_list.rs
│   │   │   ├── mod.rs
│   │   │   └── splash.rs
│   │   ├── utils/
│   │   │   ├── error.rs
│   │   │   └── mod.rs
│   │   ├── dom.rs
│   │   ├── init.rs
│   │   └── lib.rs
│   ├── themes/
│   │   ├── cyberpunk.toml
│   │   ├── dracula.toml
│   │   ├── frappe.toml
│   │   ├── gruvbox-dark.toml
│   │   ├── gruvbox-light.toml
│   │   ├── latte.toml
│   │   ├── macchiato.toml
│   │   ├── mocha.toml
│   │   └── synthwave.toml
│   ├── build.rs
│   ├── Cargo.toml
│   ├── index.html
│   └── keybinds.toml
├── server/
│   ├── src/
│   │   ├── routes/
│   │   │   ├── configs/
│   │   │   │   ├── handlers.rs
│   │   │   │   └── mod.rs
│   │   │   ├── containers/
│   │   │   │   ├── parser/
│   │   │   │   │   ├── basic.rs
│   │   │   │   │   ├── config.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── network.rs
│   │   │   │   │   └── storage.rs
│   │   │   │   ├── actions.rs
│   │   │   │   ├── details.rs
│   │   │   │   ├── handlers.rs
│   │   │   │   └── mod.rs
│   │   │   ├── mod.rs
│   │   │   └── types.rs
│   │   ├── main.rs
│   │   └── version.rs
│   └── Cargo.toml
├── sys/
│   ├── env/
│   │   └── .env.example
│   ├── html/
│   │   ├── htmlformat.py
│   │   └── htmllint.py
│   ├── layout/
│   │   └── statusline.toml
│   ├── rust/
│   │   ├── audit.py
│   │   ├── check.py
│   │   ├── clean.py
│   │   ├── clippy.py
│   │   ├── rustfmt.py
│   │   └── test_rust.py
│   ├── theme/
│   │   ├── theme.py
│   │   └── theme.toml
│   └── utils/
│       ├── cleanup_backups.py
│       ├── fix_nerdfonts.py
│       ├── lines.py
│       ├── precommit.py
│       ├── pyclean.py
│       ├── pycompile.py
│       ├── pylint.py
│       ├── remove_emojis.py
│       ├── venv.py
│       └── xdg_paths.py
├── Cargo.lock
├── Cargo.toml
├── DEBUG.md
├── deny.toml
├── justfile
├── README.md
├── rebuild.py
├── requirements-dev.txt
├── start.py
├── status.py
├── stop.py
└── sysrat.toml
```
