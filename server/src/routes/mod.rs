use actix_web::web;
pub mod health_check;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::health_check);
}
