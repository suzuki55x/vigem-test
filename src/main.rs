use std::{thread, time};

fn main() {
    println!("Hello, world!");

    let client = vigem_client::Client::connect().unwrap();

    // コントローラーの種類を指定
    let id = vigem_client::TargetId::XBOX360_WIRED;
    // コントローラーのインスタンスを作成
    let mut target = vigem_client::Xbox360Wired::new(client, id);

    // コントローラーを接続
    target.plugin().unwrap();

    // コントローラーが接続されるまで待機
    target.wait_ready().unwrap();

    let mut gamepad = vigem_client::XGamepad {
        // 使うボタンを指定
        buttons: vigem_client::XButtons!(UP | RIGHT | LEFT | LB | A | X),
        ..Default::default()
    };

    // 時間カウント
    let start = time::Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs_f64();

        // 10秒経過したら終了
        if elapsed > 10.0 {
            break;
        }

        gamepad.thumb_lx = (elapsed.cos() * 30000.0) as i16;

        let _ = target.update(&gamepad);

        thread::sleep(time::Duration::from_millis(10));
    }
}
