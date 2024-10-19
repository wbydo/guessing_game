use std::io;

fn main() {
    println!("数を予想してください");

    println!("予想を入力してください");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("読み込みに失敗しました");


    println!("あなたの予想: {}", guess);
}
