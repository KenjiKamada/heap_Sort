use rand::Rng;

// https://daeudaeu.com/heap-sort/
fn main() {
    const N:usize = 100; //N個の整数数列

   // 乱数生成 
   //  let mut x0=vec![5,3,9,10,34,7,6,3,4,66,3,1,7,9,3];
   let  mut x0 = Vec::with_capacity(N);
   x0.resize(N, 0); // https://doc.rust-lang.org/std/vec/struct.Vec.html
       // -1000 以上 1000 以下のランダムな整数を100個用意する。
        for n in 0..N{
            x0[n] = rand::thread_rng().gen_range(-1000, 1000+1);
        }

        
    let _nx:f32=x0.len() as f32;
    let kf = _nx.log2()+1.0;
    let nx = _nx as usize;
    let k = kf.floor();
    let mut el = Vec::<i32>::new(); // メモリ上の階層位置

            // el = [1,2,4,8,16,....]を作る。
            for n in 0..k as i32{ 
                let m ;
                let two:i32 =2;
                
                m = two.pow(n.try_into().unwrap());
                el.push(m as i32);
            }
  
    let el_iter = el.iter();
    let _n_all:i32 = el_iter.sum();
    let n_all:usize = _n_all as usize;
    let mut x = Vec::with_capacity(n_all); 

        x.resize(n_all, 0);  // アロケーション
        x[0..N].copy_from_slice(&x0[0..N]); // https://stackoverflow.com/questions/66609964/rust-looking-for-a-c-memcpy-equivalent
            
              heap_sort(&mut x, &el, nx); // ヒープソートの関数

    // 確認  yにソート結果
    // 全部"True！"なら良い。
    for n in 0..x0.len(){
        println!("sort前={}, sort結果={}", x0[n], x[n]);
        assert!(x[n] >= x[n+1]);
    }

}

////////////////////////////////////////////////////////////////
fn heap_sort(x:&mut Vec<i32>, el:&Vec<i32>, nx:usize){
   
   let mut x_result = vec![0;nx];

   for u in 0..nx{
        for l in {1..el.len()}.rev(){
        
            for k in 0..el[l-1]{
                let m=2*k;
                let n=m+1;

                if x[(el[l]+m-1) as usize] < x[(el[l]+n-1) as usize]{ // 左が大きいにする
                    x.swap((el[l]+m-1) as usize, (el[l]+n-1) as usize);
                }

                if x[(el[l-1]+k-1) as usize] < x[(el[l]+m-1) as usize]{ // 上が大きいにする。
                    x.swap((el[l-1]+k-1) as usize, (el[l]+m-1) as usize);
                }
            }
        }

        x_result[u] = x[0];
        x[0] = std::i32::MIN; // 最小値https://qiita.com/mkimura81/items/03f2ed423905f6b39935
    }

        // x_result -> xにnx個コピーする。 
        x[0..nx].copy_from_slice(&x_result[0..nx]); // https://stackoverflow.com/questions/66609964/rust-looking-for-a-c-memcpy-equivalent  
    }



