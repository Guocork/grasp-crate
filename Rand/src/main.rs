use rand::prelude::*;

fn main() {
    // 在 1~100 范围内生成一个随机数
    let num = random::<u32>() % 100 + 1;
    println!("生成的随机数是：{}", num);

    // 从数组中随机选择一个元素
    let arr = [1, 2, 3, 4, 5];
    let elem = arr.choose(&mut thread_rng()).unwrap();
    println!("随机选择的元素是：{}", elem);
}
