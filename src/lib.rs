#![feature(async_await)]
use std::future::Future;

pub trait Broadcast {
    type Future1: Future<Output = Vec<()>>;
    type Future2: Future<Output = String>;

    fn broadcast1() -> Self::Future1;
    fn broadcast2() -> Self::Future2;
}
pub async fn propose<B>() where B: Broadcast {
    B::broadcast1().await;
    B::broadcast2().await;
}