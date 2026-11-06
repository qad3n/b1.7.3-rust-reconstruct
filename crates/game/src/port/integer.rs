use jvm::string::JString;
use jvm::throwable::Throwable;

// Integer.parseInt over text the port holds as a rust str
pub fn parseInt(string: &str) -> Result<i32, Throwable> { jvm::integer::parseInt(&JString::from(string)) }
