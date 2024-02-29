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
//nightly
//trait MyF = Fn() -> () + Unpin + 'static;       // experimental!

// stable
trait MyF: Fn() -> () + Unpin {}

// Rust note: generic on 'F' required (even though we just want this for one concrete type),
//      because having a trait object as member would need 'dyn MyF'.
//
struct StreamActor /*<F>*/ {
    call_me: dyn MyF
}

impl StreamActor {
    fn new(/*move*/ f: impl MyF) -> StreamActor {
        StreamActor{ call_me: f }
    }
}

impl Actor for StreamActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct RandomTick;

impl Handler<RandomTick> for StreamActor {
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
