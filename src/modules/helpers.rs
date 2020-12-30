pub mod run {

    use std::process::Command;

    pub fn shell_with_output(command: (&str, [&str; 1])) -> Vec<u8>{
        let output = Command::new(command.0)
            .args(&command.1)
            .output()
            .expect("Failed to run command");
        return output.stdout;
    }

    pub fn with_shell(command: (&str, [&str; 2])) {
        let _output = Command::new(command.0)
            .args(&command.1)
            .output()
            .expect("Failed to run command");
    }

}

pub mod trim {

    fn output_to_string(stdout: Vec<u8>) -> String {
        let encoded = String::from_utf8_lossy(stdout.as_slice());
        return String::from(encoded);
    }

    pub fn interfaces(output: Vec<u8>) -> Vec<String> {
        let mut interfaces: Vec<String> = Vec::new();
        let trimmed = output_to_string(output);
        let split = trimmed.split("\n");
        for s in split {
            if s.starts_with("interface") {
                let parts: Vec<&str> = s.split(" ").collect();
                let interface_name: &str = parts[1];
                interfaces.push(String::from(interface_name));
            }
        }
        return interfaces;
    }

}

pub mod find {

    pub fn index_of(vector: &Vec<String>, of: &String) -> i32 {
        return vector.iter().position(|r| r == of).unwrap() as i32;
    } 

}