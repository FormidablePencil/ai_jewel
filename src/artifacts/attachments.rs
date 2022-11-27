pub struct FuckedUp {
    // if reduced reliance on failure messages consciousness expanded.
    fuck_up: bool,
    fucked_up_msg: String,
}

impl FuckedUp {
    pub(crate) fn new() -> Self {
        Self {
            fuck_up: false,
            fucked_up_msg: String::from(""),
        }
    }

    // gc = garbage collection
    fn fucked_up(&mut self, msg: String) {
        self.fuck_up = true;
        self.fucked_up_msg = msg;
    }
}

// gc = garbage collection
fn randomizer_gc() {
    // our not so random randomizer. Like the garbage collection deciding
}
