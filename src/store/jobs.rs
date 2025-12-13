// Worker jobs
#[derive(Debug, PartialEq)]
pub enum Job {
    Set(Args),
    Get(Args),
    List(Args),
    Delete(Args),
    Clear(Args),
    Exit(Args),
}

#[derive(Debug, PartialEq)]
pub struct Args {
    key: Option<String>,
    value: Option<String>,
    multiplier: Option<i64>
}

