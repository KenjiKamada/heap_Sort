use std::collections::BinaryHeap;

use rand::Rng;
// https://daeudaeu.com/heap-sort/
fn main() {
//    const N:usize = 5; //N個の整数数列
   // let mut x: [i32; N]=[0;N]; //乱数
        // -1000 以上 1000 以下のランダムな整数を100個用意する。
        // for n in 1..x.len(){
        //     x[n] = rand::thread_rng().gen_range(-1000, 1000+1);
        // }
        let x; //乱数
            x = [5,0,9,7,1,6,3,8,4];
        

        for n in 0..x.len(){
            println!("ソート前={}",x[n]);
        }


        let mut heap:[i32];



}
