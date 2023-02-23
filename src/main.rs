extern crate ansi_term;
use ansi_term::Colour::{Blue, Yellow, Red, Green};

use std::{env};
use std::str::FromStr;
use parity_scale_codec::{Compact, Decode, Encode}; //  CompactLen // CompactLen,

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("args: {}
Перевод чисел в разные системы.
Examles:
    # перевод числа в scale
        trl 12345
        > строка: [12345] число: 12345 scale-hex (2): e5c0

    # перевод из scale в число
        trl 0xe5c0 
        > scale-число [e5c0] = 12345

        trl 0xe5c0FF00EE
        > scale-число [e5c0] = 12345 остаток 3: [ff00ee]

    # перевод из hex в число
        trl 0Xe5c0
        > hex-число [e5c0] = 49381

", args.len() );    
        //    dbg!(args);
        std::process::exit(0);
    }

    let str = &args[1];
    if str.starts_with("0x") { // если это SCALE HEX

        let str = &str[2..]; // убираем '0x'
        let mut bytes: &[u8] = &hex::decode(str).unwrap(); // раскодируем из HEX
        let scale = Compact::<u128>::decode(&mut bytes).unwrap(); // переведем из SCALE
        let r = bytes.len(); // сколько осталось после выбора первого scale
        if r == 0 {
            println!("scale-число {} = {}",
                Yellow.on(Blue).paint( format!(" {} ",&str) ),
                Green.paint(format!("{:?}",&scale))
            );
        } else {
            let ost = hex::encode(bytes); // остаток неиспользованных байт

            println!("scale-число {} = {} {}",
                Yellow.on(Blue).paint( format!(" {} ",&str[0..(str.len()-ost.len())]) ),
                Green.paint(format!("{:?}",&scale)),
                Red.bold().paint( format!("остаток {}: {ost}",ost.len()/2) )
            );
        }

    } else if str.starts_with("0X") { // если это просто HEX без всякого SCALE

        let str = &str[2..]; // убираем 0x
        let bytes: &[u8] = &hex::decode(str).unwrap(); // раскодируем из HEX
        let mut bytes128: [u8; 16] = Default::default(); // готовим массив для u128
        bytes128[0..bytes.len()].copy_from_slice(&bytes); // копируем в него из нашего массива сколько надо
        let value = u128::from_le_bytes(bytes128); // переводим в число u128

        println!("hex-число {} = {}",
            Yellow.on(Blue).paint( format!(" {} ",&str) ),
            Green.paint(format!("{value}"))
        );

    } else {

        let value = u128::from_str(str).unwrap(); // переводим в число u128
        let scale = Compact(value).encode(); // переводим в Compact
        let hex = hex::encode(scale); // кодируем в HEX
        
        println!("число: {} = {} scale-hex ({}): {}", 
            Yellow.on(Blue).paint( format!(" {} ",&str) ),
            Green.paint(format!("{value}")),
            hex.len()/2,
            Green.paint(format!("0x{hex}"))
        );
    }  
}
