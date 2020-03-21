use thiserror::Error;

/// VMware backdoor errors.
#[derive(Error, Debug)]
#[error("vmware backdoor error: {0}")]
pub struct VmwError(pub(crate) String);

impl From<&str> for VmwError {
    fn from(arg: &str) -> Self {
        VmwError(arg.to_string())
    }
}

impl From<String> for VmwError {
    fn from(arg: String) -> Self {
        VmwError(arg)
    }
}
