pub enum CdAction {
    Change { new_dir: String },
    NoOp,
}

pub fn resolve_cd(current_dir: &str, cmd_trimmed: &str) -> CdAction {
    match cmd_trimmed {
        "cd" | "cd ~" => {
            CdAction::Change {
                new_dir: String::from("/root"),
            }
        }
        "cd /" => {
            CdAction::Change {
                new_dir: String::from("/"),
            }
        }
        "cd .." => resolve_parent_dir(current_dir),
        _ if cmd_trimmed.starts_with("cd ") => {
            resolve_cd_target(current_dir, cmd_trimmed)
        }
        _ => CdAction::NoOp,
    }
}

fn resolve_parent_dir(current_dir: &str) -> CdAction {
    let mut new_dir = String::from("/");
    if let Some(parent) = std::path::Path::new(current_dir).parent() {
        let parent_str = parent.to_string_lossy().to_string();
        if !parent_str.is_empty() {
            new_dir = parent_str;
        }
    }
    CdAction::Change { new_dir }
}

fn resolve_cd_target(current_dir: &str, cmd_trimmed: &str) -> CdAction {
    let target = cmd_trimmed
        .strip_prefix("cd ")
        .unwrap()
        .trim()
        .trim_matches('"')
        .trim_matches('\'');

    if target == ".." {
        return resolve_parent_dir(current_dir);
    }

    let new_dir = if target.starts_with('/') {
        target.to_string()
    } else {
        format!(
            "{}/{}",
            current_dir.trim_end_matches('/'),
            target
        )
    };

    CdAction::Change { new_dir }
}
