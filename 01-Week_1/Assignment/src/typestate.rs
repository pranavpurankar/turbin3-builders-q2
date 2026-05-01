use std::marker::PhantomData;

pub struct Waiting;
pub struct InProgress;
pub struct Finished;

pub struct Game<S> {
    pub secret: u32,
    pub attempts: u32,
    _state: PhantomData<S>,
}

impl Game<Waiting> {
    pub fn new() -> Self {
        Game { secret: 0, attempts: 0, _state: PhantomData }
    }

    pub fn start(self, secret: u32) -> Game<InProgress> {
        Game { secret, attempts: 0, _state: PhantomData }
    }
}

impl Game<InProgress> {
    pub fn end(self) -> Game<Finished> {
        Game { secret: self.secret, attempts: self.attempts, _state: PhantomData }
    }
}