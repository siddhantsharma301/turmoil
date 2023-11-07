use crate::utils::PathMaker;

const NODE: &str = "./../target/debug/node";
pub struct CommandMaker;

impl CommandMaker {
    pub fn cleanup() -> String {
        format!(
            "rm -r .db-* ; rm .*.json; mkdir -p {}",
            PathMaker::logs_path()
        )
    }

    pub fn clean_logs() -> String {
        format!(
            "rm -r {logs} ; mkdir -p {logs}",
            logs = PathMaker::logs_path()
        )
    }

    pub fn compile() -> String {
        "cargo build".to_string()
    }

    pub fn generate_key(filename: &str) -> String {
        format!("{} generate_keys --filename {}", NODE, filename)
    }

    pub fn run_primary(
        keys: &str,
        committee: &str,
        store: &str,
        parameters: &str,
        app_api: &str,
        abci_api: &str,
        debug: bool,
    ) -> String {
        let v = if debug { "-vvv" } else { "-vv" };
        format!("{} {} run --keys {} --committee {} --store {} --parameters {} primary --app-api {} --abci-api {}", NODE, v, keys, committee, store, parameters, app_api, abci_api)
    }

    pub fn run_worker(
        keys: &str,
        committee: &str,
        store: &str,
        parameters: &str,
        id: &str,
        debug: bool,
    ) -> String {
        let v = if debug { "-vvv" } else { "-vv" };
        format!(
            "{} {} run --keys {} --committee {} --store {} --parameters {} worker --id {}",
            NODE, v, keys, committee, store, parameters, id
        )
    }

    pub fn run_client(address: &str, size: usize, rate: usize, nodes: Vec<&str>) -> String {
        let nodes = if !nodes.is_empty() {
            format!("--nodes {}", nodes.join(" "))
        } else {
            String::new()
        };
        format!(
            "./../target/debug/benchmark_client {} --size {} --rate {} {}",
            address, size, rate, nodes
        )
    }

    pub fn run_app(listen_on: &str) -> String {
        format!("../target/debug/evm-app --demo {}", listen_on)
    }

    pub fn kill() -> String {
        if std::env::var("TMUX").is_ok() {
            "tmux kill-session -a".to_string()
        } else {
            "tmux kill-server".to_string()
        }
    }

    pub fn alias_binaries(origin: &str) -> String {
        let node = format!("{}/node", origin);
        let client = format!("{}/benchmark_client", origin);
        format!(
            "rm node ; rm benchmark_client ; ln -s {} . ; ln -s {} .",
            node, client
        )
    }
}
