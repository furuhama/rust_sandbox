// assert ASCII ART "HELLO, RUST!!!"

pub fn hello_rust() {
    let s =  "\n===============================================================
  _  _ ___ _   _   __        ___ _  _   __ _____   _   _   _
 | || | __| | | | /__\\      | _ \\ || |/' _/_   _| / \\ / \\ / \\
 | >< | _|| |_| || \\/ |  _, | v / \\/ |`._`. | |   \\_/ \\_/ \\_/
 |_||_|___|___|___\\__/  [_/ |_|_\\\\__/ |___/ |_|   (_) (_) (_)
===============================================================\n";
    println!("{}", s.to_string())
}

