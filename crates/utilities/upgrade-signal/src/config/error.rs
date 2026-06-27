/// Error returned when CLI arguments cannot form an upgrade signal configuration.
#[derive(Debug, thiserror::Error)]
pub enum UpgradeSignalConfigError {
    /// Upgrade IDs were set without a contract address.
    #[error("upgrade ID requires --upgrade-signal.contract")]
    MissingContractAddress,
    /// The upgrade ID is empty.
    #[error("upgrade ID cannot be empty")]
    EmptyUpgradeId,
    /// The upgrade ID is not recognized.
    #[error("unknown upgrade ID `{0}`")]
    UnknownUpgradeId(String),
    /// Runtime-admin mode was selected without the Admin RPC namespace enabled.
    #[error(
        "--upgrade-signal.mode=runtime-admin requires the Admin RPC namespace; enable it with {flag_hint}"
    )]
    RuntimeAdminRequiresAdminRpc {
        /// CLI flag the operator should add to enable the Admin RPC namespace.
        flag_hint: &'static str,
    },
}
