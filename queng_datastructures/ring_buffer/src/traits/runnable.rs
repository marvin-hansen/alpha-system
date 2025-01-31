/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

pub trait Runnable: Send {
    fn run(self: Box<Self>);
}
