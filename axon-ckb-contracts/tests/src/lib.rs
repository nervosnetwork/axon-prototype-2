use ckb_tool::ckb_types::bytes::Bytes;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg(test)]
#[allow(dead_code)]
mod common;
#[cfg(test)]
mod environment_builder;
#[cfg(test)]
#[allow(dead_code)]
mod secp256k1;
#[cfg(test)]
mod test_always_success;
#[cfg(test)]
mod test_checker_vote;
#[cfg(test)]
mod test_checker_withdraw;
#[cfg(test)]
mod test_collator_publish_task;
#[cfg(test)]
mod test_collator_submit_faild_challenge;
#[cfg(test)]
mod test_collator_submit_success_challenge;
#[cfg(test)]
mod test_collator_submit_tasks;
#[cfg(test)]
mod test_collator_unlock;
#[cfg(test)]
mod test_join_sidechain;
#[cfg(test)]
mod test_publish_challenge;
#[cfg(test)]
mod test_quit_sidechain;
#[cfg(test)]
mod test_refresh_task;
#[cfg(test)]
mod test_take_beneficiary;

const TEST_ENV_VAR: &str = "CAPSULE_TEST_ENV";

pub enum TestEnv {
    Debug,
    Release,
}

impl FromStr for TestEnv {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(TestEnv::Debug),
            "release" => Ok(TestEnv::Release),
            _ => Err("no match"),
        }
    }
}

pub struct Loader(PathBuf);

impl Default for Loader {
    fn default() -> Self {
        let test_env = match env::var(TEST_ENV_VAR) {
            Ok(val) => val.parse().expect("test env"),
            Err(_) => TestEnv::Debug,
        };
        Self::with_test_env(test_env)
    }
}

impl Loader {
    fn with_test_env(env: TestEnv) -> Self {
        let load_prefix = match env {
            TestEnv::Debug => "debug",
            TestEnv::Release => "release",
        };
        let dir = env::current_dir().unwrap();
        let mut base_path = PathBuf::new();
        base_path.push(dir);
        base_path.push("..");
        base_path.push("build");
        base_path.push(load_prefix);
        Loader(base_path)
    }

    pub fn load_binary(&self, name: &str) -> Bytes {
        let mut path = self.0.clone();
        path.push(name);
        fs::read(path).expect("binary").into()
    }
}
