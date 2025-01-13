pub fn print_banner() {
    let p = [
        ["┌─┐ "], 
        ["│─┘ "], 
        ["┴   "], 
    ];

    let a = [
        ["┌─┐ "],
        ["├─┤ "],
        ["┴ ┴ "],
    ];

    let i = [
        ["┬ "], 
        ["│ "], 
        ["┴ "]
    ];

    let l = [
        ["┬   "],
        ["│   "],
        ["┴─┘ "]
    ];

    let o = [
        ["┌─┐ "],
        ["│ │ "],
        ["└─┘ "],
    ];

    let n = [
        ["┌┐┌ "],
        ["│││ "],
        ["┘└┘ "],
    ];

    let banner = [p, a, p, i, l, l, o, n];
    let init_color = 91; // inc: 36
    let mut cnt = 0;

    for row in 0..=2 {
        let mut text_color = init_color;
        for char in banner {
            let color_code = format!("\x1b[38;5;{}m", text_color);
            print!("{}{}\x1b[m", color_code, char[row][0]);
            cnt += 1;
            if cnt % 2 == 0 {
                text_color += 36;
            }
        }
        println!("");
    }
}
