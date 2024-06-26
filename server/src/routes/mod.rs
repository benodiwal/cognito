use actix_web::web;

mod health_check;
mod process_prompt;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::health_check);
    cfg.service(process_prompt::process_prompt);
}
