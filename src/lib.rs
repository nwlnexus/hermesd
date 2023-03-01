pub mod agent;
pub mod cli;

extern crate os_type;

pub fn generate_service_agent() {
    let os = os_type::current_platform();
    eprintln!("Detected OS: {:?}", os.os_type);
}
