/// Measures the execution time of an expression or block.
///
/// The execution time is recorded in the global registry under the given name.
///
/// # Examples
///
/// ```
/// use clock_tower::measure;
///
/// let result = measure!("simple_addition", 1 + 1);
/// assert_eq!(result, 2);
/// ```
#[macro_export]
macro_rules! measure {
    ($name:expr, $expr:expr) => {{
        let __t0 = std::time::Instant::now();
        let __result = $expr;
        let __t1 = std::time::Instant::now();
        $crate::time_registry::record($name, __t0, __t1);
        __result
    }};

    ($name:expr, $body:block) => {{
        let __t0 = std::time::Instant::now();
        let __result = $body;
        let __t1 = std::time::Instant::now();
        $crate::time_registry::record($name, __t0, __t1);
        __result
    }};
}