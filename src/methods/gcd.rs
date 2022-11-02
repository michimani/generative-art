use crate::images;
use proconio::input;
use std::time::SystemTime;

static OUT_DIR: &str = "out/1_gcd";

pub fn render() {
    println!("two numbers for render image >)");
    input! {
      m: u64,
      n: u64,
    }

    render_image_by_gcd(m, n);
}

const COLORS: [[u8; 3]; 8] = [
    [247, 96, 96],
    [247, 207, 96],
    [164, 247, 96],
    [96, 247, 234],
    [96, 141, 247],
    [139, 96, 247],
    [212, 96, 247],
    [247, 96, 161],
];

fn render_image_by_gcd(m: u64, n: u64) {
    let scaler: u64;

    if m < 20 {
        scaler = 100;
    } else if m < 100 {
        scaler = 10;
    } else if m < 200 {
        scaler = 5;
    } else {
        scaler = 1;
    }

    let img_x: usize = (m * scaler) as usize;
    let img_y: usize = (n * scaler) as usize;

    let mut pixels: Vec<[u8; 3]> = vec![[255, 255, 255]; img_x * img_y];

    let mut width = n;
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut loop_cnt = 0;
    let mut sq_count = 1;

    while width > 0 {
        loop_cnt += 1;

        if loop_cnt % 2 == 0 {
            while x_pos + width <= m {
                for y in y_pos * scaler..(y_pos + width) * scaler {
                    for x in x_pos * scaler..(x_pos + width) * scaler {
                        let idx = y as usize * img_x + x as usize;
                        pixels[idx] = COLORS[sq_count % COLORS.len()];
                    }
                }

                println!(
                    "({x_pos},{y_pos}) {sq_count} {:?}",
                    COLORS[sq_count % COLORS.len()]
                );

                sq_count += 1;
                x_pos += width;
            }
            width = m - x_pos;
        } else {
            while y_pos + width < n {
                for y in y_pos * scaler..(y_pos + width) * scaler {
                    for x in x_pos * scaler..(x_pos + width) * scaler {
                        let idx = y as usize * img_x + x as usize;
                        pixels[idx] = COLORS[sq_count % 5];
                    }
                }

                println!(
                    "({x_pos},{y_pos}) {sq_count} {:?}",
                    COLORS[sq_count % COLORS.len()]
                );

                sq_count += 1;
                y_pos += width;
            }
            width = n - y_pos;
        }
    }

    images::render::render(
        &format!(
            "{}/{}.jpg",
            OUT_DIR,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ),
        pixels,
        (img_x, img_y),
    )
    .unwrap();
}
