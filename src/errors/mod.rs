pub type SimutronResult<T> = Result<T, Box<SimutronError>>;

#[derive(Debug)]
pub enum SimutronError {
    // A specific non-critical error that prevents operation.
    Runtime(String),
    // Generic error as a catch-all
    Generic(std::io::Error),
}

// 1. Implement Display (for user-facing messages)
impl std::fmt::Display for SimutronError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SimutronError::Runtime(msg) => write!(f, "Runtime Error: {}", msg),
            SimutronError::Generic(e) => write!(f, "Uncaught Error: {}", e),
        }
    }
}

// 2. Implement the Error trait (for compatibility with standard error mechanisms)
impl std::error::Error for SimutronError {
    // This method is optional, but often useful for logging the original source of the error
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SimutronError::Runtime(_) => None,
            SimutronError::Generic(e) => Some(e),
        }
    }
}

// 3. Implement the 'From' trait to convert external errors into your custom error
impl From<std::io::Error> for SimutronError {
    fn from(error: std::io::Error) -> Self {
        SimutronError::Generic(error)
    }
}

#[macro_export]
macro_rules! runtime_error {
    ($($arg:tt)*) => {
        Err(Box::new(SimutronError::Runtime(format!($($arg)*))))
    };
}
