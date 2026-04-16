#[derive(Clone, Default)]
pub struct EmailValidationService;

impl EmailValidationService {
    pub fn validate(&self, _email: &str) -> bool {
        rand::random::<bool>()
    }
}
