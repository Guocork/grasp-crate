use futures::executor::block_on;

fn main() {
    // let result = async_hi();
    // block_on(result);
    // block_on(async_hello());

    async fn one_add_one() -> i32 {
        1 + 1
    }

    async fn two_add_two() -> i32 {
        2 + 2
    }

    async fn part_mult_part() -> i32 {
        let f1 = one_add_one();
        let f2 = two_add_two();

        let res = futures::join!(f1,f2);
        res.0 * res.1
    }

    let result = block_on(part_mult_part());
    println!("(1 + 1) * (2 + 2) = {result}");
}

// async fn async_hi() {
//     println!("hello world from async");
// }

// async fn async_hello() {
//     println!("hello");
//     async_world().await;
// }


// async fn async_world() {
//     println!("world")
// }