
/// Identify a staking provider by its name and ibc status
pub trait Identify {
    fn over_ibc(&self) -> bool;
    fn name(&self) -> &'static str;
}
