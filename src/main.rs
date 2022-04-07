//use std::collections::BinaryHeap;
//use std::mem;
//use std::cmp::Ordering;
//use rand::Rng;
//use std::f64;

// https://daeudaeu.com/heap-sort/
fn main() {
//    const N:usize = 5; //N個の整数数列
   // let mut x: [i32; N]=[0;N]; //乱数
        // -1000 以上 1000 以下のランダムな整数を100個用意する。
        // for n in 1..x.len(){
        //     x[n] = rand::thread_rng().gen_range(-1000, 1000+1);
        // }





    let mut x0=vec![5,3,9,10,34,7,6,3,4,66,3,1,7,9,3];
    let mut x=vec![0;100];

    for n in 0..x0.len(){
            x[n] = x0[n];

    }
   // x=[5,0,9,7,1,6,3,8,4];

    let _nx:f32=x0.len() as f32;
    let nx = _nx as usize;
    let kf = _nx.log2()+1.0;
    let k = kf.floor();
    let mut L = Vec::<i32>::new(); // メモリ上の階層位置

    //    L[2] = 1;

    
             for n in 0..k as i32{ 
                 let mut m ;
                 let two:i32 =2;
                 
                 m = two.pow(n.try_into().unwrap());
                 L.push(m as i32);
             }

            for n in 0..x0.len(){
                println!("ソート前={}",x[n]);
            }
    
           // x.swap(1, 2);
         
           heap_sort(&mut x, &L, nx);

            for n in 0..x0.len(){
                println!("後={}",x[n]);
            }
    

       // let mut heap:[i32];



}

fn heap_sort(x:&mut Vec<i32>, L:&Vec<i32>, nx:usize){
   // x.swap(1, 2);
   
   let mut x_result = vec![0;nx];

   for u in 0..nx{
        for l in {1..L.len()}.rev(){
        
            for k in {0..L[l-1]}{
                let m=2*k;
                let n=m+1;

                if x[(L[l]+m-1) as usize] < x[(L[l]+n-1) as usize]{ // 左が大きいにする
                    x.swap((L[l]+m-1) as usize, (L[l]+n-1) as usize);
                }

                if x[(L[l-1]+k-1) as usize] < x[(L[l]+m-1) as usize]{ // 上が大きいにする。
                    x.swap((L[l-1]+k-1) as usize, (L[l]+m-1) as usize);
                }
            }
        }

        x_result[u] = x[0];
        x[0] = -1;
        println!("x_result={}",x_result[u]);
    }

        for i in 0..nx{
            x[i] = x_result[i];
        }

    }



