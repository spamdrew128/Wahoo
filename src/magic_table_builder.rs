use crate::board_representation::{Bitboard, Square, NUM_SQUARES};
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

type Magic = u64;
const NUM_HASH_ENTRIES: usize = 107648;

const ROOK_MAGICS: [Magic; NUM_SQUARES as usize] = [
    0xc08000802330400a,
    0x140084060001000,
    0x80082000803000,
    0x2900242010000900,
    0x2200040910202200,
    0x200020064010830,
    0x2900008402000100,
    0x100002042108100,
    0x800a81400020,
    0x400400020100240,
    0x8001002002490010,
    0x800805001814800,
    0x1223000d00280032,
    0x44a001042000804,
    0x21b000100020004,
    0x608000c4800100,
    0x2400318000804004,
    0x810094001200041,
    0x1088818010002000,
    0x1200210049001000,
    0x100808004013800,
    0x1801010004000208,
    0x280c440002100188,
    0x4010020014408104,
    0x2048401280008061,
    0x430004040122000,
    0x5002074200102080,
    0xc080100080800800,
    0x420980100110004,
    0x18020080800400,
    0x9008010080800200,
    0xf000140200004081,
    0x2100400220800a81,
    0x20a0c8842002100,
    0x801004802000,
    0xa80100084800800,
    0x10040080800800,
    0x412008400800680,
    0x108048020c001001,
    0x2002900a60004c4,
    0x4021088000410020,
    0x1810500820004002,
    0xc10012008808010,
    0x80621001001000c,
    0x400100501801002c,
    0x8220004008080,
    0x60040200010100,
    0x25030040860004,
    0x5028204c02a00,
    0x400460008180,
    0x10200100421100,
    0xc900008018080,
    0x4c000802c4008080,
    0x900c002010088401,
    0x1010002000c4100,
    0x80040109804200,
    0x1080801064420302,
    0x4080400020110083,
    0x800200141090111,
    0x8002a04805009001,
    0x1000410480083,
    0x18a001004210802,
    0x8010010200c814,
    0x481011004c2c0082,
];

const BISHOP_MAGICS: [Magic; NUM_SQUARES as usize] = [
    0x104209c0c008610,
    0x40b0020850c28848,
    0x1004482200480100,
    0x820c040039078,
    0x32040d0408080000,
    0x2000822022044000,
    0x4040084105a4004,
    0x20ca010088040288,
    0x8404842442044,
    0x2006610242044504,
    0x900440544040,
    0x2040420800000,
    0x802b0a0210000000,
    0x1420210048010,
    0x200018801082002,
    0x2000411101101200,
    0x4040812005010204,
    0x4002008022048,
    0xc010000200224100,
    0x204002804301000,
    0x2044000220a00141,
    0x80090452008a0304,
    0xe219305012004,
    0x808040404c5000,
    0x200e1000403002a1,
    0x68014800203e0c00,
    0x2ac80090008075,
    0x480000820040,
    0x24410010d3004002,
    0x8000408104100402,
    0x41c0100849405,
    0x2884100004d0804,
    0x8012100400410800,
    0xb42014d00200800,
    0x4020228800104800,
    0x20081880080,
    0x32500082004c2200,
    0x4004080041000,
    0x40122002400a0,
    0x1042040040203200,
    0x8140208202120,
    0xc040108100404,
    0x1008510801000800,
    0x4120030431010800,
    0x2008a4000082,
    0xc010141000202,
    0x402040404000090,
    0x3410021c80201100,
    0x2840420140004,
    0x188c838040020,
    0x20044208d02000,
    0x601048084042000,
    0x8030012124240124,
    0x2030202002118001,
    0x2481a1002420010,
    0x483a320644090000,
    0x8000854800900800,
    0x410904048080800,
    0x8200204044022100,
    0x10a0a800,
    0x82418e1010202200,
    0x21402a50010200,
    0xa200b01001c80080,
    0xc1021202020450,
];

#[derive(Debug, Copy, Clone)]
struct MagicEntry {
    mask: Bitboard,
    magic: Magic,
    shift: u8,
    offset: usize,
}

impl MagicEntry {
    const fn new() -> Self {
        Self {
            mask: Bitboard::new(0),
            magic: 0,
            shift: 0,
            offset: 0,
        }
    }

    const fn hash_index(self, blockers: Bitboard) -> usize {
        ((blockers.as_u64().wrapping_mul(self.magic)) >> self.shift) as usize
    }

    fn as_string(self) -> String {
        format!(
            "MagicEntry {{ mask: Bitboard::new({:#x}), magic: {:#x}, shift: {}, offset: {} }}",
            self.mask.as_u64(),
            self.magic,
            self.shift,
            self.offset
        )
    }
}

#[derive(Debug)]
pub struct MagicLookupBuilder {
    rook_entries: [MagicEntry; NUM_SQUARES as usize],
    bishop_entries: [MagicEntry; NUM_SQUARES as usize],
    hash_table: Box<[Bitboard]>,
}

impl MagicLookupBuilder {
    fn new() -> Self {
        Self {
            rook_entries: [MagicEntry::new(); NUM_SQUARES as usize],
            bishop_entries: [MagicEntry::new(); NUM_SQUARES as usize],
            hash_table: vec![Bitboard::new(0); NUM_HASH_ENTRIES].into_boxed_slice(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn as_init_string(self) -> String {
        let mut rook_str = String::new();
        let mut bishop_str = String::new();
        let mut table_str = String::new();

        for i in 0..NUM_SQUARES {
            let rook_entry = self.rook_entries[i as usize];
            let bishop_entry = self.bishop_entries[i as usize];

            rook_str.push_str(format!("{},\n", rook_entry.as_string()).as_str());
            bishop_str.push_str(format!("{},\n", bishop_entry.as_string()).as_str());
        }

        for i in 0..NUM_HASH_ENTRIES {
            table_str
                .push_str(format!("Bitboard::new({:#x}), ", self.hash_table[i].as_u64()).as_str());
        }

        format!("pub const MAGIC_LOOKUP: MagicLookup = MagicLookup {{ rook_entries: [{rook_str}],\nbishop_entries[{bishop_str}],\nhash_table: [{table_str}],\n}};")
    }
}

const fn offset_from_mask(mask: Bitboard) -> usize {
    let base: u32 = 2;
    base.pow(mask.popcount()) as usize
}

type Shifter = fn(Bitboard) -> Bitboard;
const ROOK_SHIFTERS: [Shifter; 4] = [
    Bitboard::north_one,
    Bitboard::east_one,
    Bitboard::south_one,
    Bitboard::west_one,
];
const BISHOP_SHIFTERS: [Shifter; 4] = [
    Bitboard::northeast_one,
    Bitboard::southeast_one,
    Bitboard::southwest_one,
    Bitboard::northwest_one,
];

fn calc_blocker_mask(sq: Square, shifters: [Shifter; 4]) -> Bitboard {
    let mut result = Bitboard::default();

    for f in shifters {
        let mut bitset = f(sq.as_bitboard());

        while f(bitset).is_not_empty() {
            result |= bitset;
            bitset = f(bitset);
        }
    }

    result
}

fn attacks_from_blockers(sq: Square, blockers: Bitboard, shifters: [Shifter; 4]) -> Bitboard {
    let mut result = Bitboard::default();
    let availible = !blockers;

    for f in shifters {
        let mut bitset = sq.as_bitboard();

        while bitset.overlaps(availible) {
            bitset = f(bitset);
            result |= bitset;
        }
    }

    result
}

fn fill_single_entry(
    sq: Square,
    entry: MagicEntry,
    table: &mut [Bitboard],
    offset: &mut usize,
    shifters: [Shifter; 4],
) {
    let set = entry.mask.as_u64();
    let mut subset: u64 = 0;
    let mut largest_index = 0;

    loop {
        let blockers = Bitboard::new(subset);
        let attack_set = attacks_from_blockers(sq, blockers, shifters);
        let index = entry.hash_index(blockers);
        largest_index = max(largest_index, index);

        assert!(
            (table[index + *offset] == Bitboard::new(0)) || (table[index + *offset] == attack_set)
        );

        table[index + *offset] = attack_set;

        subset = subset.wrapping_sub(set) & set;
        if subset == 0 {
            break;
        }
    }

    *offset += largest_index + 1;
}

fn init_hash_table(lookup: &mut MagicLookupBuilder) {
    let mut offset = 0;
    for i in 0..NUM_SQUARES {
        let sq = Square::new(i);
        let rook_entry = lookup.rook_entries[i as usize];
        let bishop_entry = lookup.bishop_entries[i as usize];

        lookup.rook_entries[i as usize].offset = offset;
        fill_single_entry(
            sq,
            rook_entry,
            &mut lookup.hash_table,
            &mut offset,
            ROOK_SHIFTERS,
        );

        lookup.bishop_entries[i as usize].offset = offset;
        fill_single_entry(
            sq,
            bishop_entry,
            &mut lookup.hash_table,
            &mut offset,
            BISHOP_SHIFTERS,
        );
    }
}

fn generate_magic_table() -> String{
    let mut lookup = MagicLookupBuilder::new();

    for i in 0..NUM_SQUARES {
        let sq = Square::new(i);
        let index = i as usize;
        let rook_mask = calc_blocker_mask(sq, ROOK_SHIFTERS);
        let bishop_mask = calc_blocker_mask(sq, BISHOP_SHIFTERS);

        lookup.rook_entries[index].mask = rook_mask;
        lookup.rook_entries[index].magic = ROOK_MAGICS[index];
        lookup.rook_entries[index].shift = NUM_SQUARES - (rook_mask.popcount() as u8);

        lookup.bishop_entries[index].mask = bishop_mask;
        lookup.bishop_entries[index].magic = BISHOP_MAGICS[index];
        lookup.bishop_entries[index].shift = NUM_SQUARES - (bishop_mask.popcount() as u8);
    }

    init_hash_table(&mut lookup);
    lookup.as_init_string()
}

pub fn build_file() {
    let mut file = File::create("magic_table.rs").expect("couldn't create file");
    let contents = generate_magic_table();
    let include = "use crate::magic::*;";
    write!(&mut file, "{include}\n\n{contents}").unwrap();
}