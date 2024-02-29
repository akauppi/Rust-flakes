#[allow(non_snake_case)]
//?mod poolActor;

use std::time::Duration;

use log::{debug};

use crate::Fish;

use actix::prelude::*;
use rand::Rng;

pub struct Pool {
    //? poolAddr: Addr
}

impl Pool {
    pub fn new() -> Pool {
        //? let poolAddr = PoolActor::new().start();
        let streamAddr = StreamActor::new(move || {
            debug!("Random tick");
        }).start();

        Pool {
            //? poolAddr = PoolActor::new().start(),
        }
    }

    pub async fn fish_for(&self, fish: Fish, _timeout: Duration) -> Option<Fish> {
        debug!("Fishing for: {fish}...");
        None        // todo!
    }
}

/*
* PoolActor
*
* Receives:
*
*/

/*
* StreamActor
*
* Receives:
*   - RandomTick
*
* Provides a steady, but random stream of fish to the pond.
*/
// tbd. By making the type 'F' in a 'type', we may be able to reduce the repetition?
struct StreamActor<F: Fn() -> () + Unpin> {
    call_me: F
}

impl<F: Fn() -> () + Unpin> StreamActor<F> {
    fn new(/*move*/ f: F) -> StreamActor<F> {
        StreamActor{ call_me: f }
    }
}

impl<F: Fn() -> () + Unpin + 'static> Actor for StreamActor<F> {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct RandomTick;

impl<F: Fn() -> () + Unpin + 'static> Handler<RandomTick> for StreamActor<F> {
    type Result = ();

    fn handle(&mut self, msg: RandomTick, ctx: &mut Context<Self>) -> Self::Result {
        (self.call_me)();

        // Note: They warn it's "slow", but making a commonly used (static?) random-number-generator
        //      in Rust is an Advanced Feat!!
        //
        //      hmm.. ideally, we could get one from the 'Context'?
        //
        let mut rng = rand::thread_rng();

        let d: Duration = Duration::from_millis(rng.gen_range(1000..5000));
        ctx.notify_later(RandomTick, d);

        ()
    }
}
