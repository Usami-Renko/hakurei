
extern crate hakurei;

use hakurei::preinclude::*;

const WINDOW_TITLE: &'static str = "Trangle Example";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct TriangleProcedure {

}

impl ProgramProc for TriangleProcedure {

}

fn main() {

    let procecure = TriangleProcedure {};
    let mut program = ProgramBuilder::new(procecure)
        .title(WINDOW_TITLE)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .build().unwrap();

    program.launch();
}