use rand::Rng;

// https://daeudaeu.com/heap-sort/
fn main() {
    const N:usize = 100; //N個の整数数列

   // 乱数生成 
   //    let mut x0=vec![-5,3,-9,10];
    let  mut x0 = Vec::with_capacity(N);
        x0.resize(N, 0); // https://doc.rust-lang.org/std/vec/struct.Vec.html
        // -1000 以上 1000 以下のランダムな整数を100個用意する。
        for n in 0..N{
            x0[n] = rand::thread_rng().gen_range(-1000, 1000+1);
        }
    
    // 乱数：x0 -> x としてｘ:Vec<i32>の中身をソートする。
    let mut x = Vec::with_capacity(N); 
        x.resize(N, std::i32::MIN);  // アロケーション
        x[0..N].copy_from_slice(&x0[0..N]); // https://stackoverflow.com/questions/66609964/rust-looking-for-a-c-memcpy-equivalent

        /* ヒープソートの関数  */
        heap_sort(&mut x);       

    /*****    確かめ *****/
    // 確認  yにソート結果
    for n in 0..x0.len(){
        println!("sort前={}, sort結果={}", x0[n], x[n]);
    }
    // Rustのソートと比較する。x
    x0.sort_by(|a, b| b.cmp(a));
    assert_eq!(x0,x);

} // end of main
////////////////////////////////////////////////////////////////
fn heap_sort(x:&mut Vec<i32>){
    
    let nx = x.len();
    let nxf:f32=(nx) as f32;
    let kf = nxf.log2()+1.0;
    let k = kf.floor();
    let mut el= Vec::<usize>::new(); // メモリ上の階層位置
            
        for n in 0..k as usize{   // el = [1,2,4,8,16,....]を作る。
            let m:i32 ;
            let two:i32 =2;
                m = two.pow(n.try_into().unwrap());
                el.push(m as usize);
        }

   x.push(std::i32::MIN); //アルゴリズム的に１つ増やす
   let mut x_result = vec![0;nx];
   for u in 0..nx{
        for l in {1..el.len()}.rev(){
            for k in 0..el[l-1] as usize{
              let m=2*k;
              let n=m+1;
                
                if (el[l] as usize+n-1) > nx {break;} // xの個数以上はやらない

                if x[el[l]+m-1] < x[el[l]+n-1]{ // 左が大きいにする
                    x.swap(el[l]+m-1, el[l]+n-1);
                }
                if x[el[l-1]+k-1] < x[el[l]+m-1]{ // 上が大きいにする。
                    x.swap(el[l-1]+k-1, el[l]+m-1);
                }
            }
        }
        x_result[u] = x[0];
        x[0] = std::i32::MIN; // 最小値https://qiita.com/mkimura81/items/03f2ed423905f6b39935
    }
        // x_result -> xにnx個コピーする。 
        x.pop(); //１つ増やした分、減らす。
        x[0..nx].copy_from_slice(&x_result[0..nx]); // https://stackoverflow.com/questions/66609964/rust-looking-for-a-c-memcpy-equivalent  

} // end of heap_sort()



