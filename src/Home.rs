use clap::{ Arg, Command};

#[derive(Debug, Serialize, Deserialize)]
struct Task{
    description: String,
    completed: bool,
}

impl Task{
    fn new(description: String) -> Task{
        Task{
            description,
            completed: false,
        }
    }
}

fn build_cli() -> Command<'static>{



