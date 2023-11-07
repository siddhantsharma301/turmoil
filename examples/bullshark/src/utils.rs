use std::path::Path;

pub(crate) struct PathMaker;

impl PathMaker {
    pub fn binary_path() -> String {
        Path::new("..").join("target").join("release")
    }

    pub fn node_crate_path() -> String {
        Path::new("..").join("node")
    }

    pub fn committee_file() -> String {
        ".committee.json"
    }

    pub fn parameters_file() -> String {
        ".parameters.json"
    }

    pub fn key_file(i: usize) -> String {
        format!(".node-{}.json", i)
    }

    pub fn db_path(i: usize, j: Option<usize>) -> String {
        let worker_id = if Some(j) { format("-{}", j) } else { "" };
        format!(".db-{}{}", i, worker_id)
    }

    pub fn logs_path() -> String {
        "logs"
    }

    pub fn primary_log_file(i: usize) -> String {
        format!("{}/primary-{}.log", Self::logs_path(), i)
    }

    pub fn app_log_file(i: usize) -> String {
        format!("{}/app-{}.log", Self::logs_path(), i)
    }

    pub fn worker_log_file(i: usize, j: usize) -> String {
        format!("{}/worker-{}-{}.log", Self::logs_path(), i, j)
    }

    pub fn client_log_file(i: usize, j: usize) -> String {
        format!("{}/client-{}-{}.log", Self::logs_path(), i, j)
    }

    pub fn results_path() -> &'static str {
        "results"
    }

    pub fn result_file(
        faults: usize,
        nodes: usize,
        workers: usize,
        collocate: usize,
        rate: usize,
        tx_size: usize,
    ) -> String {
        format!(
            "{}/bench-{}-{}-{}-{}-{}-{}.txt",
            Self::results_path(),
            faults,
            nodes,
            workers,
            collocate,
            rate,
            tx_size
        )
    }

    pub fn plots_path() -> &'static str {
        "plots"
    }

    pub fn agg_file(
        type_: &str,
        faults: usize,
        nodes: usize,
        workers: usize,
        collocate: usize,
        rate: usize,
        tx_size: usize,
        max_latency: Option<usize>,
    ) -> String {
        let name = if let Some(max_latency) = max_latency {
            format!(
                "{}-{}-bench-{}-{}-{}-{}-{}-{}.txt",
                type_, max_latency, faults, nodes, workers, collocate, rate, tx_size
            )
        } else {
            format!(
                "{}-bench-{}-{}-{}-{}-{}-{}.txt",
                type_, faults, nodes, workers, collocate, rate, tx_size
            )
        };
        format!("{}/{}", Self::plots_path(), name)
    }

    pub fn plot_file(name: &str, ext: &str) -> String {
        format!("{}/{}.{}", Self::plots_path(), name, ext)
    }
}
