/* Systems are special type of function that queries ECS for data and performs 
operations against it. Systems are run accoding to the Scheduler, which creates an
execution plan for each operation that systems call.

We will use multi-file modules to arranges our systems in one directory.
 */

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        // stub scheduler that returns an empty Legions schedule.
        .build()

    
}