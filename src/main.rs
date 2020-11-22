use probe_rs::MemoryInterface;
use probe_rs::Session;

fn main() {
    println!("SAME54P20A demo");

    let mut session = Session::auto_attach("ATSAME54P20A").unwrap();
    let mut core = session.core(0).unwrap();

    // Write a block of 50 bit words.
    let buff = [0xdeadbeefu32; 50];
    core.write_32(0x2000_0000, &buff).unwrap();

    // Read a single 32 bit word.
    let word = core.read_word_32(0x2000_0000).unwrap();

    println!("word {:#X}", word);

    // Write a block of 50 8bit words.
    let buff = [0x66; 50];
    core.write_8(0x2000_0000, &buff).unwrap();

    // Read a block of 50 32 bit words.
    let mut buff = [0u32; 50];
    core.read_32(0x2000_0000, &mut buff).unwrap();

    for i in buff.iter() {
        println!("buff {:#X}", i);
    }
}
