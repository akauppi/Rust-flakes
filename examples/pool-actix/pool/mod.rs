#[allow(non_snake_case)]
mod poolActor::PoolActor;
mod fish;

use std::time::Duration;
use fish::Fish;

use actix::prelude::*;

#[actix_rt::main]
async fn main() {
    let poolAddr = PoolActor::new().start();
    let streamAddr = StreamActor::new(poolAddr).start();

    let fut1 = pool.fish_for(Fish::Ahven, Duration::from_secs(3));
    let fut2 = pool.fish_for(Fish::Hauki, Duration::from_secs(10));

    futures_util::join(fut1, fut2) .await;
}
