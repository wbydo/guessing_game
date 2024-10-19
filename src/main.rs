use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数を予想してください");

    let secret_number = rand::thread_rng()
        // 下限値は含みますが上限値は含みません。
        // これと同等の1..=100という範囲を渡すこともできます。
        //
        // https://bit.ly/4dRtcSP
        .gen_range(1..101);

    // Debug用
    // println!("秘密の数字: {}", secret_number);

    loop {
        println!("予想を入力してください");

        let mut guess = String::new();
        io::stdin()
            // とりあえず知っておいてほしいのは、変数のように参照もデフォルトで不変であることです。
            // したがって、&guessではなく&mut guessと書いて可変にする必要があります。
            //
            // https://bit.ly/4eP5NTo
            //
            // 感想: 参照の不変性がイマイチピンと来ない。安全っぽくはある。
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        // Rustではguessの前の値を新しい値で覆い隠す（shadowする）ことが許されているのです。
        // この機能はある型から別の型に値を変換するときによく使われる
        // https://bit.ly/3YvlY2v
        //
        // 感想: キモ。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい！"),
            Ordering::Greater => println!("大きい！"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}
