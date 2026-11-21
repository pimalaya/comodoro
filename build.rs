fn main() {
    use pimalaya_cli::build::{features_env, git_envs, target_envs};

    features_env(include_str!("./Cargo.toml"));
    target_envs();
    git_envs();
}
