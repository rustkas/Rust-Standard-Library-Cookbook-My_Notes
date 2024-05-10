// You can implement default easily for your own types
pub enum CrustType {
  Thin,
  Thick,
}
impl Default for CrustType {
  fn default() -> CrustType {
      CrustType::Thin
  }
}
