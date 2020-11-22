use probe_rs::MemoryInterface;
use probe_rs::Session;

fn main() {
    println!("SAME54P20A demo");

    let mut session = Session::auto_attach("ATSAME54P20A").unwrap();
    let mut core = session.core(0).unwrap();

    // Read a single 32 bit word.
    let word = core.read_word_32(0x2000_0000).unwrap();

    // Writing is just as simple.
    let buff = [0xdeadbeefu32; 50];
    core.write_32(0x2000_0000, &buff).unwrap();

    // Read a single 32 bit word.
    let word = core.read_word_32(0x2000_0000).unwrap();

    println!("word {:#X}", word);

    // of course we can also write 8bit words.
    let buff = [0x66; 50];
    core.write_8(0x2000_0000, &buff).unwrap();

    // Read a single 32 bit word.
    let word = core.read_word_32(0x2000_0000).unwrap();

    // Read a block of 50 32 bit words.
    let mut buff = [0u32; 50];
    core.read_32(0x2000_0000, &mut buff).unwrap();

    for i in buff.into_iter() {
        println!("buff {:#X}", i);
    }
}
