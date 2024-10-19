use std::io;

fn main() {
    println!("数を予想してください");

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

    println!("あなたの予想: {}", guess);
}
