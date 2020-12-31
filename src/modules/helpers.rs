pub mod run {

    pub fn with_shell(command: (&str, &[&str])) -> Vec<u8> {
        let output = std::process::Command::new(command.0)
            .args(command.1)
            .output()
            .expect("Failed to run command");
        output.stdout
    }

}

pub mod trim {

    use crate::modules::config::Config;

    fn shell_output_to_string(stdout: Vec<u8>) -> String {
        String::from(String::from_utf8_lossy(stdout.as_slice()))
    }

    pub fn interfaces(output: Vec<u8>) -> Vec<String> {
        let mut interfaces: Vec<String> = Vec::new();
        let trimmed = shell_output_to_string(output);
        let split = trimmed.split("\n");
        for s in split {
            if s.starts_with("interface") {
                let parts: Vec<&str> = s.split(" ").collect();
                let interface_name: &str = parts[1];
                interfaces.push(String::from(interface_name));
            }
        }
        interfaces
    }

    pub fn user_home(output: Vec<u8>) -> String {
        let trimmed = shell_output_to_string(output);
        let parts: Vec<&str> = trimmed.split(":").collect();
        String::from(parts[5])
    }

    pub fn config(content: String) -> Config {
        let mut config = Config {
            defaults: true,
            systemd: true
        };
        let lines = content.split("\n");
        for line in lines {
            // Skip commented lines
            if line.starts_with("#") {
                continue;
            }
            if line == "nosystemd" {
                config.defaults = false;
                config.systemd = false;
            }
        }
        config
    }

}

pub mod find {

    pub fn index_of_string(vector: &Vec<String>, of: &String) -> i32 {
        vector.iter().position(|r| r == of).unwrap() as i32
    } 

}
