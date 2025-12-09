use std::path::PathBuf;
use zed::serde_json::{self, Value};
use zed_extension_api::{self as zed, LanguageServerId, Worktree};

struct MoZukuExtension;

impl zed::Extension for MoZukuExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        let server_path = self.binary_path(worktree)?;

        Ok(zed::Command {
            command: server_path.to_string_lossy().to_string(),
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        // The `for_worktree` method returns a Result, so we handle it with `?`.
        let settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        // The VSCode extension wraps all settings under a "mozuku" key in initializationOptions.
        // We replicate that behavior here for compatibility with the existing LSP server.
        let init_opts = settings
            .initialization_options
            .clone()
            .unwrap_or_else(|| serde_json::json!({}));

        let mozuku_init_opts = serde_json::json!({
            "mozuku": init_opts
        });

        Ok(Some(mozuku_init_opts))
    }
}

impl MoZukuExtension {
    fn binary_path(&mut self, worktree: &Worktree) -> zed::Result<PathBuf> {
        // The Zed extension sandbox doesn't allow direct filesystem access like `Path::exists()`.
        // We must rely on `worktree.which` to find the binary in the system's PATH.
        // For a production-ready extension, this function would download the binary
        // from a release and return the path to the downloaded file.
        if let Some(path_str) = worktree.which("mozuku-lsp") {
            return Ok(PathBuf::from(path_str));
        }

        Err("Could not find `mozuku-lsp` binary in your PATH. Please build it and ensure its location is added to your system's PATH.".into())
    }
}

zed::register_extension!(MoZukuExtension);
