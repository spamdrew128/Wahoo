use crate::board_representation::{Bitboard, Square, NUM_SQUARES};

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
}

#[derive(Debug)]
struct MagicLookup {
    rook_entries: [MagicEntry; NUM_SQUARES as usize],
    bishop_entries: [MagicEntry; NUM_SQUARES as usize],
    hash_table: [Bitboard; NUM_HASH_ENTRIES],
}

impl MagicLookup {
    const fn new() -> Self {
        Self {
            rook_entries: [MagicEntry::new(); NUM_SQUARES as usize],
            bishop_entries: [MagicEntry::new(); NUM_SQUARES as usize],
            hash_table: [Bitboard::new(0); NUM_HASH_ENTRIES],
        }
    }
}

const fn blocker_mask_incomplete(d1: Bitboard, d2: Bitboard, d3: Bitboard, d4: Bitboard) -> bool {
    d1.union(d2).union(d3).union(d4).is_not_empty()
}

#[rustfmt::skip]
const fn rook_blocker_mask(sq: Square) -> Bitboard {
    let mut north = sq.as_bitboard();
    let mut east = sq.as_bitboard();
    let mut south = sq.as_bitboard();
    let mut west = sq.as_bitboard();

    let mut result = Bitboard::new(0);

    while blocker_mask_incomplete(north, east, south, west) {
        north = north
            .north_one()
            .intersection(Bitboard::RANK_8.complement());
        east = east
            .east_one()
            .intersection(Bitboard::H_FILE.complement());
        south = south
            .south_one()
            .intersection(Bitboard::RANK_1.complement());
        west = west
            .west_one()
            .intersection(Bitboard::A_FILE.complement());

        result = result
            .union(north)
            .union(east)
            .union(south)
            .union(west);
    }

    result
}

#[rustfmt::skip]
const fn bishop_blocker_mask(sq: Square) -> Bitboard {
    let mut northeast = sq.as_bitboard();
    let mut southeast = sq.as_bitboard();
    let mut southwest = sq.as_bitboard();
    let mut northwest = sq.as_bitboard();

    let mut result = Bitboard::new(0);

    while blocker_mask_incomplete(northeast, southeast, southwest, northwest) {
        northeast = northeast
            .northeast_one()
            .intersection(Bitboard::RANK_8.union(Bitboard::H_FILE).complement());
        southeast = southeast
            .southeast_one()
            .intersection(Bitboard::RANK_1.union(Bitboard::H_FILE).complement());
        southwest = southwest
            .southwest_one()
            .intersection(Bitboard::RANK_1.union(Bitboard::A_FILE).complement());
        northwest = northwest
            .northwest_one()
            .intersection(Bitboard::RANK_8.union(Bitboard::A_FILE).complement());

        result = result
            .union(northeast)
            .union(southeast)
            .union(southwest)
            .union(northwest);
    }

    result
}

#[rustfmt::skip]
const fn rook_attacks_from_blockers(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut north = sq.as_bitboard();
    let mut east = sq.as_bitboard();
    let mut south = sq.as_bitboard();
    let mut west = sq.as_bitboard();

    let mut result = Bitboard::new(0);

    let non_blocked = blockers.complement();
    while blocker_mask_incomplete(north, east, south, west) {
        north = north.north_one();
        east = east.east_one();
        south = south.south_one();
        west = west.west_one();

        result = result
            .union(north)
            .union(east)
            .union(south)
            .union(west);

        north = north.intersection(non_blocked);
        east = east.intersection(non_blocked);
        south = south.intersection(non_blocked);
        west = west.intersection(non_blocked);
    }

    result
}

#[rustfmt::skip]
const fn bishop_attacks_from_blockers(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut northeast = sq.as_bitboard();
    let mut southeast = sq.as_bitboard();
    let mut southwest = sq.as_bitboard();
    let mut northwest = sq.as_bitboard();

    let mut result = Bitboard::new(0);

    let non_blocked = blockers.complement();
    while blocker_mask_incomplete(northeast, southeast, southwest, northwest) {
        northeast = northeast.northeast_one();
        southeast = southeast.southeast_one();
        southwest = southwest.southwest_one();
        northwest = northwest.northwest_one();

        result = result
            .union(northeast)
            .union(southeast)
            .union(southwest)
            .union(northwest);

        northeast = northeast.intersection(non_blocked);
        southeast = southeast.intersection(non_blocked);
        southwest = southwest.intersection(non_blocked);
        northwest = northwest.intersection(non_blocked);
    }

    result
}

const fn offset_from_mask(mask: Bitboard) -> usize {
    let base: u32 = 2;
    base.pow(mask.popcount()) as usize
}

const fn init_magic_lookup() {
    let mut lookup = MagicLookup::new();

    let mut prev_offset = 0;
    let mut i = 0;
    while i < NUM_SQUARES {
        let sq = Square::new(i);
        let index = i as usize;
        let rook_mask = rook_blocker_mask(sq);
        let bishop_mask = bishop_blocker_mask(sq);

        lookup.rook_entries[index].mask = rook_mask;
        lookup.rook_entries[index].magic = ROOK_MAGICS[index];
        lookup.rook_entries[index].offset = ((NUM_SQUARES as u32) - rook_mask.popcount()) as usize;
        lookup.rook_entries[index].offset = offset_from_mask(rook_mask) + prev_offset;
        prev_offset = lookup.rook_entries[index].offset;

        lookup.bishop_entries[index].mask = bishop_mask;
        lookup.bishop_entries[index].magic = BISHOP_MAGICS[index];
        lookup.bishop_entries[index].offset =
            ((NUM_SQUARES as u32) - bishop_mask.popcount()) as usize;
        lookup.bishop_entries[index].offset = offset_from_mask(bishop_mask) + prev_offset;
        prev_offset = lookup.bishop_entries[index].offset;

        i += 1;
    }
}
