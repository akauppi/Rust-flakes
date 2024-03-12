#[allow(non_snake_case)]
//?mod poolActor;

use std::time::Duration;

use log::{debug};

use crate::Fish;

use actix::prelude::*;
use rand::Rng;

pub struct Pool {
    poolAddr: Addr
}

impl Pool {
    pub fn new() -> Pool {
        //? let poolAddr = PoolActor::new().start();
        let streamAddr = StreamActor::new(move || {
            //R debug!("Random tick");
            let fish = rand::random::gen_range(0..3).map(Fish::genByIndex);
            poolAddr.do_send( PA_New(fish) )
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
*   - New(Fish)
*
* If there is a 'Rod', fishing for the incoming fish, it is immediately caught. Fishes are not
* collected in the pool (i.e. placing a Rod doesn't pick existing fishes).
*/
struct PoolActor {
    rods: Vec<RodActor>
}

impl PoolActor {
    fn new() -> PoolActor {
        PoolActor{ rods: Vec::new() }
    }
}

impl Actor for PoolActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct PA_New(Fish);

impl Handler<PA_New> for PoolActor {
    type Result = ();

    fn handle(&mut self, msg: PA_New, _ctx: &mut Context<Self>) -> Self::Result {
        let PA_New(fish) = msg;

        let x = for rod in self.rods {
            if rod.catch(fish) {
                debug!("Rod {rod} given a fish.");
                return ();
            }
        };
        debug!("No rod wanted {fish} (presented to {}).", self.rods.len());
            // Rust note: no expressions in format string curly braces, only variables.

        ()
    }
}

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
trait MyF: Fn() -> () + Unpin + 'static {
    fn call(&self);
}
impl<F: Fn() -> () + Unpin + 'static> MyF for F {
    fn call(&self) {
        self();
    }
}
// /stable

// Rust note: generic on 'F' required (even though we just want this for one concrete type),
//      because having a trait object as member would need 'dyn MyF'.
//
struct StreamActor {
    call_me: Box<dyn MyF>       // tbd. would like here to be 'MyF' (with a known size; no boxing)
}

impl StreamActor {
    fn new(/*move*/ f: impl MyF) -> StreamActor {
        StreamActor{ call_me: Box::new(f) }
    }
}

impl Actor for StreamActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct SA_RandomTick;

impl Handler<SA_RandomTick> for StreamActor {
    type Result = ();

    fn handle(&mut self, msg: SA_RandomTick, ctx: &mut Context<Self>) -> Self::Result {
        (self.call_me)();

        // Note: They warn it's "slow", but making a commonly used (static?) random-number-generator
        //      in Rust is an Advanced Feat!!
        //
        //      hmm.. ideally, we could get one from the 'Context'?
        //
        let mut rng = rand::thread_rng();

        let d: Duration = Duration::from_millis(rng.gen_range(1000..5000));
        ctx.notify_later(SA_RandomTick, d);

        ()
    }
}

/*??
* RodActor
*
* Receives:
*   - RA_New(Fish)
*
* If there is a 'Rod', fishing for the incoming fish, it is immediately caught. Fishes are not
* collected in the pool (i.e. placing a Rod doesn't pick existing fishes).
*_/
struct RodActor {
    rods: Vec<RodActor>
}

impl crate::pool::PoolActor {
    fn new() -> crate::pool::PoolActor {
        crate::pool::PoolActor { rods: Vec::new() }
    }
}

impl Actor for crate::pool::PoolActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct PA_New(Fish);
**/