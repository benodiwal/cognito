use actix_web::middleware::Logger;

pub fn logger_middleware() -> Logger {
    let formatted_string = "%t: [%r] %s";
    Logger::new(formatted_string)
}
