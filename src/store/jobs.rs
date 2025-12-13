// Worker jobs
#[derive(Debug, PartialEq)]
pub enum Job {
    Set,
    Get,
    List,
    Delete,
    Clear,
    Exit,
}

impl Clone for Job {
    fn clone(&self) -> Job {
        match self {
            &Job::Set => Job::Set,
            _ => Job::Exit,
        }
    }
}

impl Job {
    fn execute(&self) {
        match self {
            Job::Set => println!("Doing set job"),
            _ => (),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Args {
    pub command: Job,
    pub key: Option<String>,
    pub value: Option<String>,
    pub multiplier: Option<i64>,
}

impl Clone for Args {
    fn clone(&self) -> Args {
        Args {
            command: self.command.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            multiplier: self.multiplier,
        }
    }
}

impl Args {
    pub fn execute(&self) -> Result<(), String> {
        match self.command {
            Job::Set => self.command.execute(),
            _ => (),
        }

        Ok(())
    }
}
