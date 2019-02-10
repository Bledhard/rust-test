pub mod unstructured_api;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Organise our modules to be more user-friendly public api with less nesting levels
pub use documentation::unstructured_api::kinds::PrimaryColor;
pub use documentation::unstructured_api::kinds::SecondaryColor;
pub use documentation::unstructured_api::utils::mix;