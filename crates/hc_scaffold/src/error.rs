use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum ScaffoldError {
    /// std::io::Error
    #[error("IO error: {0}")]
    StdIoError(#[from] std::io::Error),

/*     #[error("ffs::IoError: {0}")]
    FfsIoError(#[from] ffs::IoError),
    /// DnaError
    #[error("DNA error: {0}")]
    DnaError(#[from] holochain_types::dna::DnaError),
    
    /// MrBundleError
    #[error(transparent)]
    MrBundleError(#[from] mr_bundle::error::MrBundleError),
    
    /// SerializedBytesError
    #[error("Internal serialization error: {0}")]
    SerializedBytesError(#[from] SerializedBytesError),
    */

    /// serde_yaml::Error
    #[error("YAML serialization error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    /// anything else
    #[error("Unknown error: {0}")]
    MiscError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// HcBundle Result type.
pub type ScaffoldResult<T> = Result<T, ScaffoldError>;