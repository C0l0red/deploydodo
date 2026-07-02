pub fn build_command(current_dir: &str, user_cmd: &str) -> String {
    if user_cmd.trim().is_empty() {
        return format!("cd {} && true", shell_escape(current_dir));
    }

    format!(
        "{setup} && {user_cmd}",
        setup = format!(
            "cd {} && export TERM=xterm-256color CLICOLOR_FORCE=1 && ls() {{ command ls -C --color=always \"$@\"; }}",
            shell_escape(current_dir),
        ),
        user_cmd = user_cmd,
    )
}

pub fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
