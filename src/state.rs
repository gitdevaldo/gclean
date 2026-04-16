use crate::service::EmailValidationService;

#[derive(Clone)]
pub struct AppState {
    pub validator: EmailValidationService,
}

impl AppState {
    pub fn new(validator: EmailValidationService) -> Self {
        Self { validator }
    }
}
