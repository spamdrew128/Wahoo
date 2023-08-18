#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{eval::{evaluation::ScoreTuple, piece_tables::{Pst, Prt}}, board::board_representation::NUM_PIECES};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(169, 288), s(210, 337), s(269, 365), s(320, 342), s(328, 359), s(225, 337), s(194, 340), s(223, 250), 
  s(289, 321), s(321, 336), s(347, 325), s(332, 340), s(355, 322), s(349, 323), s(306, 331), s(311, 294), 
  s(308, 319), s(352, 327), s(374, 350), s(370, 351), s(354, 351), s(394, 333), s(345, 319), s(331, 296), 
  s(339, 325), s(352, 345), s(367, 367), s(399, 368), s(373, 367), s(389, 361), s(351, 345), s(364, 311), 
  s(333, 331), s(348, 339), s(360, 368), s(374, 367), s(371, 370), s(369, 359), s(367, 340), s(340, 335), 
  s(322, 311), s(334, 332), s(351, 341), s(351, 357), s(367, 357), s(356, 332), s(352, 326), s(328, 325), 
  s(304, 290), s(314, 309), s(324, 323), s(343, 322), s(345, 324), s(339, 322), s(325, 304), s(325, 308), 
  s(269, 278), s(302, 308), s(303, 299), s(313, 308), s(320, 306), s(321, 295), s(300, 320), s(309, 280), 
]),
// Bishop PST
Pst::new([
  s(338, 384), s(314, 378), s(266, 383), s(242, 389), s(262, 388), s(256, 384), s(315, 385), s(325, 365), 
  s(345, 351), s(356, 364), s(357, 356), s(313, 362), s(330, 358), s(348, 368), s(321, 372), s(311, 356), 
  s(360, 365), s(385, 353), s(376, 361), s(377, 351), s(359, 358), s(382, 370), s(356, 368), s(367, 357), 
  s(352, 354), s(373, 361), s(378, 363), s(409, 375), s(384, 374), s(390, 369), s(362, 363), s(357, 357), 
  s(359, 344), s(370, 358), s(376, 371), s(395, 373), s(390, 370), s(368, 369), s(354, 362), s(366, 338), 
  s(364, 343), s(385, 355), s(389, 361), s(380, 366), s(379, 372), s(381, 363), s(378, 354), s(373, 339), 
  s(383, 341), s(383, 330), s(389, 338), s(371, 349), s(374, 351), s(386, 345), s(392, 343), s(378, 328), 
  s(355, 330), s(384, 350), s(358, 346), s(352, 342), s(360, 342), s(352, 354), s(370, 334), s(374, 320), 
]),
// Rook PST
Pst::new([
  s(562, 642), s(585, 634), s(556, 652), s(576, 639), s(594, 629), s(555, 641), s(563, 639), s(592, 627), 
  s(541, 639), s(544, 643), s(556, 647), s(571, 635), s(559, 631), s(563, 635), s(559, 628), s(561, 628), 
  s(525, 632), s(545, 632), s(541, 630), s(544, 630), s(562, 623), s(548, 622), s(599, 615), s(546, 616), 
  s(502, 635), s(529, 624), s(522, 640), s(527, 631), s(535, 620), s(521, 625), s(525, 623), s(502, 624), 
  s(492, 619), s(501, 623), s(509, 626), s(523, 619), s(525, 619), s(489, 628), s(518, 618), s(489, 617), 
  s(493, 608), s(501, 612), s(512, 602), s(514, 607), s(518, 606), s(513, 603), s(530, 592), s(500, 592), 
  s(482, 601), s(502, 601), s(513, 603), s(517, 601), s(521, 593), s(508, 599), s(531, 584), s(465, 601), 
  s(497, 604), s(501, 600), s(505, 602), s(513, 596), s(512, 593), s(503, 596), s(491, 601), s(488, 591), 
]),
// Queen PST
Pst::new([
  s(919, 1236), s(952, 1227), s(956, 1258), s(986, 1239), s(977, 1252), s(985, 1229), s(1055, 1154), s(985, 1213), 
  s(957, 1076), s(935, 1105), s(956, 1115), s(935, 1149), s(915, 1176), s(958, 1122), s(983, 1080), s(1016, 1073), 
  s(959, 1060), s(943, 1077), s(961, 1088), s(949, 1119), s(960, 1112), s(978, 1087), s(992, 1040), s(987, 1057), 
  s(948, 1068), s(945, 1076), s(952, 1077), s(948, 1100), s(965, 1091), s(968, 1075), s(972, 1079), s(971, 1061), 
  s(953, 1037), s(946, 1075), s(949, 1087), s(948, 1112), s(957, 1097), s(951, 1077), s(968, 1056), s(967, 1049), 
  s(953, 1033), s(962, 1048), s(957, 1072), s(954, 1080), s(952, 1090), s(970, 1054), s(979, 1043), s(970, 1033), 
  s(957, 1018), s(958, 1027), s(968, 1018), s(971, 1028), s(969, 1039), s(978, 1000), s(974, 1001), s(979, 976), 
  s(946, 1018), s(948, 1012), s(951, 1008), s(962, 994), s(958, 1002), s(942, 1000), s(957, 972), s(945, 1006), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(153, 239), s(153, 230), s(153, 231), s(174, 213), s(136, 224), s(158, 212), s(95, 241), s(80, 256), 
  s(86, 166), s(80, 156), s(114, 148), s(114, 140), s(126, 121), s(171, 131), s(155, 150), s(120, 150), 
  s(68, 150), s(77, 131), s(86, 125), s(96, 106), s(111, 113), s(106, 113), s(88, 120), s(79, 123), 
  s(65, 123), s(67, 119), s(83, 113), s(100, 105), s(99, 106), s(100, 103), s(78, 101), s(77, 104), 
  s(68, 120), s(78, 113), s(85, 114), s(91, 120), s(93, 122), s(98, 112), s(95, 98), s(82, 101), 
  s(66, 128), s(74, 119), s(72, 126), s(82, 128), s(78, 137), s(111, 116), s(98, 103), s(76, 103), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-20, 6), s(-55, 78), s(29, 84), s(-76, 119), s(-45, 87), s(-91, 97), s(-49, 92), s(79, -6), 
  s(-57, 118), s(-25, 160), s(-91, 186), s(55, 171), s(-31, 168), s(-38, 178), s(7, 153), s(-68, 134), 
  s(-116, 89), s(-14, 102), s(-40, 125), s(-69, 135), s(-28, 127), s(69, 109), s(46, 105), s(-29, 74), 
  s(-56, 22), s(-29, 52), s(-41, 62), s(-83, 69), s(-89, 64), s(-58, 54), s(-76, 54), s(-142, 30), 
  s(-84, -17), s(-33, 6), s(-44, 24), s(-65, 33), s(-60, 29), s(-33, 10), s(-72, 7), s(-144, -2), 
  s(-39, -37), s(-2, -22), s(-23, -7), s(-20, 0), s(-20, -2), s(-14, -15), s(-14, -24), s(-68, -34), 
  s(18, -62), s(6, -37), s(-9, -26), s(-38, -16), s(-34, -16), s(-20, -27), s(17, -42), s(12, -63), 
  s(33, -103), s(65, -85), s(38, -58), s(-40, -37), s(5, -52), s(-14, -47), s(40, -74), s(36, -107), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(53, 139), s(53, 130), s(53, 131), s(74, 113), s(36, 124), s(58, 112), s(-4, 141), s(-19, 156), 
  s(48, 131), s(59, 138), s(42, 112), s(40, 94), s(29, 116), s(9, 112), s(-38, 127), s(-48, 150), 
  s(36, 53), s(32, 60), s(31, 53), s(15, 63), s(1, 55), s(26, 52), s(-15, 79), s(-10, 78), 
  s(21, 21), s(10, 33), s(-10, 35), s(-4, 34), s(-19, 31), s(-1, 28), s(3, 49), s(-2, 44), 
  s(10, -4), s(-9, 21), s(-25, 19), s(-18, 14), s(-21, 8), s(0, 7), s(-21, 36), s(12, 16), 
  s(-2, 9), s(-3, 15), s(-11, 5), s(-10, 3), s(-6, -2), s(-10, 4), s(-11, 13), s(-4, 18), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(-30, -183),
  s(2, -112),
  s(-3, -48),
  s(-11, -18),
  s(-10, 5),
  s(-10, 4),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(56, -29),
  s(11, -32),
  s(0, -28),
  s(-14, -16),
  s(-24, -19),
  s(-12, -18),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(22, 370),
  s(162, 164),
  s(51, 50),
  s(18, 15),
  s(-4, -4),
  s(4, -9),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(27, 68);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(22, 57), s(24, 71), s(23, 73), s(25, 72), s(24, 76), s(24, 71), s(21, 68), s(19, 59), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(6, 37), s(14, 57), s(16, 68), s(20, 78), s(21, 85), s(22, 88), s(21, 90), s(16, 95), s(18, 90), s(20, 85), s(25, 85), s(-15, 112), s(14, 75), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(0, 110), s(1, 129), s(1, 153), s(0, 161), s(2, 173), s(3, 179), s(4, 186), s(9, 190), s(10, 195), s(11, 197), s(14, 202), s(19, 205), s(28, 200), s(29, 194), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(49, 211), s(49, 225), s(54, 256), s(58, 270), s(61, 280), s(65, 282), s(66, 302), s(69, 308), s(72, 307), s(75, 311), s(76, 322), s(78, 316), s(82, 320), s(80, 325), s(85, 327), s(82, 332), s(91, 326), s(95, 327), s(115, 310), s(122, 312), s(168, 281), s(154, 278), s(185, 256), s(236, 238), s(292, 198), s(219, 232), s(206, 217), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(12, 27), s(21, 39), s(31, 45), s(39, 49), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(2, 15), s(8, 20), s(12, 23), s(15, 28), s(16, 30), s(20, 29), s(22, 37), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(7, 4), s(11, 8), s(19, 12), s(25, 20), s(29, 24), s(36, 25), s(39, 32), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(-13, 133), s(-13, 146), s(-13, 152), s(-13, 165), s(-14, 178), s(-14, 180), s(-15, 188), s(-15, 195), s(-16, 206), s(-15, 205), s(-20, 212), s(-1, 198), s(-27, 241), s(11, 209), 
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(79, 36);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(77, 59);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(123, 7);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(103, -30);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(37, 36);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(80, 17);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(64, -43);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(23, 33);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(70, 33);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(91, 24);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(85, 13);

pub const PASSER_SQ_RULE_BONUS: ScoreTuple = s(-32, 85);

pub const TEMPO_BONUS: ScoreTuple = s(34, 24);

// KING SAFETY FEATURES
pub const ATTACKS: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight attacks
[
  s(-5, 12), s(3, -5), s(8, -2), s(12, -6), s(10, -5), s(10, -6), s(11, -5), s(12, -7), s(12, -8), s(11, -5), s(10, -4), s(11, -8), s(9, -4), s(11, -5), s(9, -7), s(8, -5), s(7, -4), s(5, -3), s(4, -6), s(4, -9), s(6, -11), s(2, -10), s(0, -12), s(0, -15), s(-1, -17), s(-7, -20), s(-4, -15), s(-31, -21), 
],
// Bishop attacks
[
  s(1, 11), s(6, -3), s(10, -5), s(10, -4), s(10, -2), s(12, -5), s(11, -6), s(10, -5), s(13, -8), s(11, -7), s(10, -7), s(9, -6), s(9, -8), s(9, -7), s(10, -10), s(6, -9), s(8, -10), s(6, -8), s(6, -9), s(3, -7), s(4, -6), s(3, -14), s(4, -8), s(1, -13), s(-4, -9), s(0, -17), s(7, -17), s(-17, -21), 
],
// Rook attacks
[
  s(8, 4), s(2, 0), s(6, -5), s(7, -7), s(6, -8), s(5, -6), s(6, -7), s(5, -6), s(6, -8), s(5, -7), s(5, -7), s(5, -9), s(4, -6), s(4, -6), s(3, -5), s(3, -4), s(2, -3), s(2, -2), s(2, -1), s(1, -1), s(0, 0), s(1, -1), s(0, 0), s(0, -1), s(0, 0), s(-2, 0), s(-15, 5), s(8, -6), 
],
// Queen attacks
[
  s(-1, 7), s(1, -4), s(4, -7), s(6, -11), s(5, -8), s(5, -7), s(6, -9), s(7, -7), s(6, -9), s(8, -9), s(7, -6), s(9, -11), s(9, -9), s(10, -10), s(9, -8), s(10, -11), s(11, -12), s(13, -11), s(11, -8), s(17, -14), s(15, -8), s(9, -4), s(18, -10), s(14, -12), s(9, 0), s(7, -1), s(23, -7), s(5, -2), 
],
// Pawn attacks
[
  s(0, 8), s(5, -2), s(4, -4), s(1, -4), s(1, -3), s(1, -3), s(0, -2), s(1, -3), s(0, -2), s(0, -3), s(0, -2), s(0, -3), s(0, -3), s(1, -4), s(1, -3), s(1, -4), s(2, -4), s(2, -4), s(3, -4), s(2, -4), s(4, -7), s(5, -7), s(4, -7), s(8, -10), s(9, -8), s(2, -6), s(35, -16), s(31, -18), 
],
];

pub const DEFENSES: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight defenses
[
  s(-5, 2), s(0, 2), s(-4, 2), s(-3, 1), s(-4, 3), s(-3, 2), s(-4, 3), s(-4, 1), s(-4, 1), s(-5, 2), s(-4, 2), s(-4, 1), s(-6, 3), s(-5, 4), s(-4, 3), s(-6, 5), s(-6, 5), s(-4, 3), s(-3, 6), s(-4, 6), s(-5, 7), s(-2, 9), s(-5, 9), s(-3, 12), s(-4, 12), s(-9, 21), s(-6, 13), s(-5, 28), 
],
// Bishop defenses
[
  s(1, 0), s(-1, 0), s(-4, 2), s(-5, 2), s(-3, 2), s(-4, 2), s(-5, 4), s(-4, 2), s(-4, 3), s(-4, 3), s(-3, 2), s(-4, 1), s(-4, 4), s(-5, 3), s(-3, 3), s(-3, 1), s(-2, 1), s(-2, 2), s(-3, 3), s(-1, 1), s(-1, 0), s(-2, 3), s(-4, 3), s(-1, 3), s(1, 4), s(-1, 6), s(2, 0), s(66, 7), 
],
// Rook defenses
[
  s(-28, 12), s(-3, 4), s(-5, 6), s(-5, 5), s(-4, 4), s(-5, 6), s(-3, 4), s(-4, 4), s(-4, 5), s(-4, 5), s(-3, 4), s(-3, 4), s(-2, 3), s(-3, 5), s(-2, 3), s(-1, 3), s(-1, 2), s(0, 2), s(-1, 3), s(-1, 4), s(-3, 5), s(-1, 4), s(-1, 4), s(-3, 5), s(-4, 5), s(-4, 4), s(5, 0), s(-5, 3), 
],
// Queen defenses
[
  s(0, -3), s(-2, 3), s(-1, 2), s(-2, 2), s(-2, 3), s(-2, 4), s(-3, 5), s(-2, 6), s(-3, 7), s(-3, 7), s(-2, 5), s(-3, 6), s(-2, 6), s(-2, 6), s(-1, 4), s(-1, 4), s(-1, 6), s(0, 5), s(0, 4), s(0, 5), s(2, 1), s(4, 0), s(-2, 8), s(0, 7), s(9, 0), s(12, -2), s(-15, 15), s(3, -3), 
],
// Pawn defenses
[
  s(-15, 0), s(-21, 6), s(-16, 5), s(-13, 4), s(-11, 3), s(-11, 2), s(-10, 2), s(-9, 3), s(-7, 1), s(-6, 0), s(-4, 0), s(-4, 0), s(-2, -3), s(-2, -3), s(-1, -4), s(0, -5), s(0, -5), s(0, -8), s(1, -12), s(1, -11), s(4, -15), s(6, -18), s(9, -19), s(10, -21), s(13, -21), s(31, -29), s(5, -21), s(18, -27), 
],
];

pub const ENEMY_KING_RANK: Prt = Prt::new([
  s(84, 20),
  s(61, 35),
  s(51, 47),
  s(49, 56),
  s(46, 69),
  s(43, 93),
  s(51, 110),
  s(53, 91),
]);

pub const TROPHISM_BONUS: [ScoreTuple; 160] = [
  s(-154, -9), s(53, -29), s(70, -24), s(74, -30), s(79, -27), s(78, -31), s(77, -29), s(81, -31), s(80, -42), s(75, -30), s(75, -30), s(70, -30), s(70, -26), s(71, -31), s(69, -27), s(67, -25), s(64, -20), s(66, -28), s(63, -21), s(60, -20), s(63, -22), s(53, -10), s(54, -13), s(53, -12), s(54, -13), s(50, -8), s(51, -10), s(49, -9), s(45, -1), s(44, -5), s(44, -3), s(42, -3), s(39, -1), s(42, -2), s(39, -1), s(33, 6), s(35, 6), s(33, 5), s(33, 5), s(29, 12), s(29, 8), s(29, 12), s(25, 13), s(24, 16), s(24, 14), s(22, 18), s(18, 21), s(18, 18), s(19, 22), s(17, 23), s(15, 24), s(15, 23), s(13, 25), s(11, 28), s(9, 30), s(8, 34), s(7, 32), s(1, 39), s(2, 38), s(0, 40), s(-2, 40), s(0, 40), s(0, 42), s(-5, 46), s(-3, 45), s(-5, 46), s(-8, 51), s(-9, 51), s(-12, 55), s(-8, 51), s(-13, 55), s(-17, 61), s(-16, 54), s(-14, 54), s(-23, 66), s(-23, 63), s(-21, 65), s(-23, 65), s(-23, 66), s(-26, 68), s(-28, 72), s(-27, 73), s(-33, 77), s(-35, 79), s(-30, 76), s(-35, 76), s(-33, 82), s(-38, 81), s(-37, 76), s(-39, 84), s(-41, 90), s(-44, 93), s(-45, 93), s(-45, 83), s(-42, 82), s(-46, 102), s(-48, 94), s(-48, 99), s(-53, 104), s(-48, 101), s(-53, 110), s(-54, 106), s(-45, 97), s(-51, 109), s(-54, 112), s(-61, 96), s(-52, 111), s(-48, 108), s(-57, 119), s(-59, 111), s(-58, 103), s(-51, 113), s(-52, 104), s(-51, 105), s(-51, 66), s(-51, 138), s(-66, 115), s(-70, 123), s(-61, 71), s(-54, 175), s(-43, 173), s(-59, 196), s(-58, 155), s(-66, 66), s(-54, 169), s(-39, 146), s(-44, -87), s(-102, 192), s(-38, 60), s(-108, 154), s(-73, 142), s(-66, 186), s(-24, 80), s(-139, 19), s(-106, 9), s(-125, -77), s(-96, 16), s(177, 113), s(-28, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), 
];

pub const PAWN_STORM_BONUS: [ScoreTuple; 160] = [
  s(54, 36), s(26, -14), s(42, -6), s(40, 0), s(40, 9), s(41, 0), s(34, 19), s(35, 9), s(36, 12), s(29, 21), s(28, 23), s(35, 9), s(25, 25), s(32, 18), s(31, 16), s(17, 30), s(23, 22), s(34, 9), s(11, 29), s(19, 29), s(22, 15), s(15, 30), s(11, 31), s(27, 19), s(8, 30), s(14, 27), s(21, 27), s(12, 26), s(4, 35), s(20, 20), s(19, 27), s(3, 30), s(11, 33), s(12, 25), s(3, 35), s(8, 36), s(21, 20), s(-11, 30), s(-2, 32), s(12, 17), s(6, 27), s(4, 30), s(-3, 18), s(-11, 22), s(-14, 28), s(44, 45), s(-170, 50), s(0, 12), s(46, 98), s(68, -101), s(-7, -136), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), 
];

pub const FILE_STRUCTURE: [ScoreTuple; 64] = [
  s(37, 46), s(44, 37), s(39, 55), s(29, 39), s(36, 36), s(40, 33), s(32, -4), s(1, 1), s(32, 31), s(21, -25), s(1, 1), s(1, 1), s(9, -12), s(1, 1), s(1, 1), s(1, 1), s(35, 44), s(43, 38), s(48, 59), s(1, 1), s(32, 40), s(47, 37), s(1, 1), s(1, 1), s(39, 29), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(25, 54), s(39, 54), s(1, 1), s(1, 1), s(32, 50), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(17, 74), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), 
];
