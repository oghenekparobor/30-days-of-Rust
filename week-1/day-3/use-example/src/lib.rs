// use std::fmt;
// use std::io;

// mod useExample {
//     fn function_1() -> fmt::Result {}

//     fn function_2() -> io::Result {}
// }

use std::fmt::Result;
use std::io::Result as IOResult;

mod useExample {
    fn function_1() -> Result {}

    fn function_2() -> IOResult {}
}
