#[derive(PartialEq, Debug)]
pub struct CommandCollection {
    _storage: Vec<Command>
}

impl CommandCollection {
    pub fn new(stack_lines: Vec<String>) -> CommandCollection {
        CommandCollection { _storage: Vec::new() }
    }
}

#[derive(PartialEq, Debug)]
struct Command {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_command_collection() {
    }
}