#![feature(async_await)]
use std::future::Future;

pub trait T {
    type Future1: Future<Output = Vec<()>>;
    type Future2: Future<Output = String>;

    fn call1() -> Self::Future1;
    fn call2() -> Self::Future2;
}
pub async fn await_two_different_return_types<S>() where S: T {
    S::call1().await;
    S::call2().await;
}