#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{eval::{evaluation::ScoreTuple, piece_tables::{Pst, Prt, SafetyPrt}}, board::board_representation::NUM_PIECES};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(176, 275), s(231, 318), s(275, 350), s(324, 327), s(346, 343), s(250, 319), s(251, 313), s(226, 240), 
  s(274, 309), s(309, 322), s(337, 310), s(323, 323), s(340, 306), s(359, 305), s(305, 313), s(307, 283), 
  s(293, 307), s(335, 313), s(357, 337), s(351, 337), s(348, 334), s(392, 317), s(346, 300), s(323, 286), 
  s(322, 311), s(335, 329), s(352, 350), s(386, 347), s(355, 348), s(379, 343), s(336, 328), s(353, 298), 
  s(314, 320), s(331, 323), s(343, 351), s(357, 347), s(355, 351), s(352, 341), s(353, 322), s(323, 317), 
  s(302, 302), s(315, 319), s(333, 327), s(334, 340), s(352, 338), s(334, 316), s(330, 313), s(308, 312), 
  s(284, 282), s(293, 298), s(305, 309), s(327, 307), s(326, 308), s(325, 306), s(306, 290), s(306, 295), 
  s(236, 270), s(282, 287), s(281, 287), s(294, 295), s(302, 292), s(303, 284), s(282, 296), s(282, 274), 
]),
// Bishop PST
Pst::new([
  s(316, 365), s(300, 359), s(254, 366), s(235, 371), s(261, 368), s(264, 362), s(301, 363), s(304, 346), 
  s(328, 328), s(336, 340), s(336, 333), s(296, 340), s(315, 334), s(338, 341), s(308, 346), s(305, 329), 
  s(338, 341), s(365, 329), s(355, 338), s(358, 327), s(343, 333), s(376, 341), s(355, 339), s(359, 332), 
  s(329, 333), s(349, 337), s(356, 340), s(384, 350), s(364, 349), s(379, 341), s(344, 337), s(342, 330), 
  s(334, 323), s(343, 336), s(350, 346), s(371, 346), s(367, 345), s(342, 343), s(331, 336), s(343, 313), 
  s(335, 324), s(356, 331), s(362, 337), s(352, 342), s(356, 344), s(355, 338), s(354, 327), s(350, 315), 
  s(353, 318), s(354, 312), s(360, 314), s(344, 325), s(350, 325), s(365, 319), s(373, 317), s(352, 304), 
  s(327, 305), s(353, 325), s(332, 322), s(327, 317), s(336, 314), s(327, 330), s(347, 308), s(352, 294), 
]),
// Rook PST
Pst::new([
  s(535, 618), s(547, 616), s(516, 634), s(538, 623), s(555, 615), s(520, 621), s(541, 617), s(570, 605), 
  s(520, 609), s(528, 612), s(538, 617), s(558, 605), s(546, 602), s(557, 604), s(559, 596), s(561, 595), 
  s(502, 607), s(528, 606), s(526, 604), s(532, 604), s(559, 595), s(554, 590), s(601, 585), s(548, 586), 
  s(481, 609), s(511, 598), s(507, 612), s(513, 605), s(520, 595), s(521, 594), s(528, 590), s(502, 594), 
  s(471, 594), s(481, 597), s(494, 598), s(509, 592), s(515, 590), s(482, 596), s(514, 587), s(481, 587), 
  s(473, 584), s(481, 587), s(494, 576), s(495, 583), s(506, 579), s(503, 573), s(523, 563), s(493, 563), 
  s(461, 577), s(480, 578), s(493, 579), s(497, 576), s(503, 568), s(501, 568), s(527, 556), s(455, 574), 
  s(478, 579), s(482, 576), s(488, 578), s(498, 571), s(502, 565), s(493, 574), s(486, 574), s(478, 564), 
]),
// Queen PST
Pst::new([
  s(960, 1098), s(997, 1092), s(1019, 1099), s(1033, 1093), s(1035, 1093), s(1034, 1085), s(1058, 1065), s(1027, 1088), 
  s(949, 1007), s(927, 1034), s(954, 1035), s(943, 1053), s(919, 1065), s(947, 1020), s(975, 1015), s(1011, 1016), 
  s(948, 999), s(938, 1014), s(955, 1026), s(952, 1043), s(954, 1028), s(969, 1003), s(966, 982), s(965, 1005), 
  s(938, 1008), s(935, 1015), s(941, 1017), s(937, 1033), s(951, 1023), s(963, 1000), s(970, 1008), s(958, 998), 
  s(938, 989), s(936, 1014), s(940, 1017), s(944, 1039), s(954, 1029), s(940, 1009), s(958, 987), s(951, 991), 
  s(934, 983), s(947, 993), s(946, 1006), s(944, 1010), s(947, 1021), s(960, 986), s(971, 983), s(959, 971), 
  s(939, 962), s(942, 968), s(952, 963), s(958, 967), s(958, 976), s(970, 940), s(976, 937), s(972, 912), 
  s(927, 962), s(928, 959), s(934, 954), s(944, 951), s(943, 962), s(927, 956), s(944, 930), s(925, 948), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(187, 201), s(181, 197), s(175, 200), s(193, 183), s(157, 193), s(181, 185), s(127, 210), s(124, 216), 
  s(87, 157), s(81, 157), s(115, 147), s(117, 145), s(117, 132), s(178, 123), s(160, 154), s(131, 147), 
  s(64, 140), s(73, 124), s(87, 119), s(96, 101), s(113, 105), s(113, 106), s(99, 113), s(90, 118), 
  s(61, 115), s(60, 113), s(82, 107), s(98, 100), s(99, 100), s(98, 101), s(79, 98), s(80, 99), 
  s(65, 114), s(73, 109), s(85, 109), s(90, 115), s(100, 117), s(97, 112), s(105, 97), s(87, 100), 
  s(64, 122), s(69, 115), s(74, 122), s(83, 127), s(87, 135), s(117, 115), s(114, 100), s(81, 101), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-28, -65), s(-26, -15), s(5, -9), s(-31, 16), s(-4, 2), s(-5, 20), s(13, 27), s(17, -46), 
  s(-28, 11), s(-17, 45), s(-72, 54), s(28, 46), s(-6, 53), s(0, 77), s(17, 64), s(-15, 35), 
  s(-56, 31), s(13, 45), s(-33, 61), s(-44, 67), s(-3, 73), s(65, 77), s(62, 68), s(8, 34), 
  s(-24, 16), s(-11, 42), s(-50, 55), s(-92, 64), s(-91, 70), s(-55, 67), s(-52, 62), s(-91, 34), 
  s(-42, -8), s(-39, 19), s(-94, 45), s(-129, 58), s(-128, 60), s(-91, 48), s(-98, 38), s(-116, 19), 
  s(4, -21), s(3, -1), s(-62, 20), s(-81, 32), s(-77, 33), s(-66, 27), s(-29, 11), s(-42, -3), 
  s(66, -38), s(18, -14), s(-5, -3), s(-50, 7), s(-45, 12), s(-21, 3), s(33, -13), s(44, -33), 
  s(48, -83), s(66, -63), s(31, -42), s(-78, -20), s(-10, -39), s(-36, -22), s(41, -52), s(48, -85), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(87, 101), s(81, 97), s(75, 100), s(93, 83), s(57, 93), s(81, 85), s(27, 110), s(24, 116), 
  s(72, 109), s(79, 109), s(54, 90), s(47, 68), s(52, 85), s(32, 96), s(0, 98), s(-12, 118), 
  s(48, 50), s(40, 59), s(32, 53), s(15, 61), s(4, 56), s(29, 54), s(-13, 78), s(0, 70), 
  s(31, 21), s(15, 35), s(-13, 38), s(-5, 36), s(-20, 36), s(0, 33), s(4, 52), s(6, 42), 
  s(18, -2), s(-6, 25), s(-29, 25), s(-17, 16), s(-18, 13), s(2, 12), s(-8, 38), s(26, 13), 
  s(1, 10), s(0, 20), s(-12, 14), s(-4, 1), s(0, -2), s(2, 6), s(14, 16), s(0, 16), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(-58, -146),
  s(-3, -103),
  s(-2, -52),
  s(-9, -27),
  s(-11, -6),
  s(-18, -4),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(3, 17),
  s(2, -21),
  s(0, -25),
  s(-14, -14),
  s(-27, -19),
  s(-18, -17),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(72, 153),
  s(123, 136),
  s(53, 46),
  s(20, 16),
  s(-4, -4),
  s(3, -8),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(34, 63);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(27, 25), s(30, 35), s(28, 36), s(30, 34), s(29, 36), s(30, 30), s(29, 24), s(33, 7), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(14, 15), s(23, 30), s(27, 36), s(32, 44), s(33, 49), s(35, 52), s(34, 52), s(30, 55), s(34, 49), s(37, 43), s(48, 40), s(7, 65), s(34, 29), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(6, 57), s(8, 69), s(9, 89), s(9, 97), s(12, 105), s(14, 110), s(15, 116), s(21, 118), s(22, 123), s(23, 126), s(25, 131), s(28, 134), s(32, 130), s(23, 130), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(60, 19), s(61, 37), s(64, 79), s(67, 104), s(69, 118), s(72, 124), s(72, 149), s(73, 155), s(75, 157), s(78, 161), s(77, 173), s(78, 171), s(81, 175), s(79, 180), s(83, 182), s(80, 187), s(89, 181), s(93, 181), s(112, 167), s(120, 165), s(144, 149), s(130, 141), s(131, 135), s(130, 135), s(115, 125), s(69, 98), s(53, 84), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(13, 28), s(23, 39), s(33, 45), s(43, 49), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(2, 19), s(7, 25), s(10, 30), s(14, 34), s(14, 37), s(19, 35), s(23, 41), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(5, 11), s(10, 14), s(17, 18), s(21, 26), s(23, 31), s(30, 31), s(37, 36), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(-5, 121), s(-4, 132), s(-4, 137), s(-2, 142), s(-1, 151), s(0, 148), s(0, 151), s(3, 154), s(5, 161), s(10, 155), s(7, 161), s(37, 136), s(31, 148), s(75, 119), 
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(80, 24);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(79, 49);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(117, 5);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(103, -27);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(37, 33);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(79, 13);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(67, -49);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(23, 28);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(70, 26);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(92, 19);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(94, -14);

pub const PASSER_SQ_RULE_BONUS: ScoreTuple = s(-42, 83);

pub const TEMPO_BONUS: ScoreTuple = s(36, 19);

// KING SAFETY FEATURES
pub const ATTACKS: [[ScoreTuple; 8]; (NUM_PIECES - 2) as usize] = [
  [s(1016, -267), s(-504, -653), s(1046, -522), s(-703, -632), s(-344, 121), s(-393, 781), s(-502, -168), s(478, -457), ],
  [s(843, -383), s(-412, -594), s(903, -551), s(-569, -524), s(-716, 81), s(-459, 607), s(-560, -585), s(539, -486), ],
  [s(1397, -343), s(-1059, -706), s(874, -571), s(617, -606), s(-809, -224), s(-482, 549), s(-554, -545), s(204, -576), ],
  [s(564, -52), s(-645, -163), s(1087, -466), s(-187, -417), s(-914, 1037), s(-498, -2235), s(-535, -634), s(198, -414), ],
];

pub const DEFENSES: [[ScoreTuple; 8]; (NUM_PIECES - 2) as usize] = [
  [s(-206, -461), s(176, -570), s(-540, -570), s(50, -753), s(-660, 96), s(-509, -43), s(-851, -559), s(-836, -683), ],
  [s(-236, -514), s(102, -1098), s(-376, -834), s(-18, -1042), s(-557, 181), s(-740, 13), s(-955, -563), s(-795, -883), ],
  [s(46, -521), s(91, -702), s(-461, -623), s(-64, -997), s(-735, 134), s(-539, 4), s(-869, -660), s(350, -829), ],
  [s(346, -447), s(-667, -1221), s(-260, -293), s(-380, -1235), s(-636, -1987), s(-507, 333), s(-845, -580), s(-710, -645), ],
];

pub const ENEMY_KING_RANK: SafetyPrt = SafetyPrt::new([
  [s(10, 9), s(7, 0), s(-2, -8), s(12, 0), s(-7, -6), s(8, -10), s(-9, -4), s(-4, -2), ],
  [s(0, -1), s(-2, -9), s(2, 5), s(-2, -9), s(-2, 10), s(-1, -8), s(11, 7), s(1, -4), ],
  [s(12, -5), s(3, -5), s(0, 7), s(-8, 2), s(-9, 11), s(-6, 12), s(-11, -4), s(-12, 8), ],
  [s(0, 0), s(12, -11), s(-7, -6), s(8, -9), s(5, -1), s(-9, -2), s(5, 11), s(0, 3), ],
  [s(0, -10), s(8, 3), s(-2, -11), s(3, 10), s(0, -7), s(6, 9), s(0, -6), s(-2, -12), ],
  [s(0, 7), s(-3, -2), s(0, 6), s(8, -4), s(-9, -1), s(12, 3), s(0, 6), s(5, 0), ],
  [s(-3, -6), s(6, 8), s(-8, -11), s(-9, -3), s(-11, 6), s(-12, 0), s(-3, 12), s(10, 5), ],
  [s(-9, -11), s(-1, -5), s(2, -9), s(1, 2), s(-1, 5), s(-5, -2), s(-6, 6), s(-2, 1), ],
]);

pub const TROPISM: [ScoreTuple; 8] = 
  [s(3, -2), s(0, 3), s(-9, -7), s(-2, -1), s(-12, -6), s(10, -5), s(1, -11), s(4, 0), ];

pub const ATTACKING_PAWN_LOCATIONS: [[ScoreTuple; 8]; 18] = [
  [s(-2, -8), s(1, -7), s(-12, 10), s(0, 8), s(10, 2), s(2, 7), s(0, -4), s(10, 3), ],
  [s(8, 5), s(2, 3), s(1, 12), s(-3, -2), s(-10, -7), s(-4, 0), s(8, 4), s(-2, 0), ],
  [s(0, -8), s(10, 9), s(9, -8), s(-10, -7), s(-10, -8), s(-2, -9), s(-8, 10), s(7, -5), ],
  [s(0, 7), s(1, -9), s(0, 0), s(-6, -10), s(4, 6), s(1, 1), s(4, 8), s(-1, 8), ],
  [s(-2, 11), s(12, -12), s(-5, 4), s(9, -11), s(-6, -11), s(9, 0), s(-9, 0), s(-3, -1), ],
  [s(-3, 1), s(-9, -11), s(0, 10), s(0, 0), s(3, 5), s(-3, -10), s(7, 8), s(5, 6), ],
  [s(2, -7), s(0, 5), s(9, 4), s(-5, 4), s(0, 11), s(-1, -3), s(-9, -9), s(-10, 9), ],
  [s(8, 0), s(8, -10), s(-4, 2), s(-3, 12), s(5, -6), s(1, -8), s(-6, 4), s(-9, -4), ],
  [s(9, -6), s(6, 4), s(8, -5), s(8, 3), s(10, 2), s(-6, -12), s(-7, 4), s(-12, -5), ],
  [s(-3, -2), s(-7, -5), s(-3, 7), s(5, -9), s(1, -12), s(1, 3), s(0, 8), s(-12, 3), ],
  [s(5, -9), s(-11, -2), s(0, -1), s(-6, -11), s(-8, -11), s(11, 9), s(0, 0), s(-9, -1), ],
  [s(3, 0), s(8, 7), s(10, -2), s(4, -8), s(-4, 1), s(9, -7), s(0, -10), s(12, -7), ],
  [s(-11, -10), s(-12, -10), s(9, -12), s(6, -10), s(10, 0), s(-7, -9), s(-3, -8), s(2, 10), ],
  [s(-10, -7), s(2, 7), s(-3, -2), s(5, 0), s(-4, -10), s(-12, 0), s(-11, -11), s(10, -1), ],
  [s(7, -10), s(-3, 1), s(-6, -2), s(-6, -7), s(2, -5), s(11, -3), s(12, -11), s(-4, 0), ],
  [s(-6, -6), s(0, -8), s(2, 0), s(-3, 5), s(1, 5), s(1, 12), s(-9, -10), s(2, -10), ],
  [s(2, -8), s(-1, -3), s(0, 0), s(-5, -5), s(-9, -10), s(-9, 4), s(-10, 0), s(6, 2), ],
  [s(7, -10), s(-6, -4), s(-5, 3), s(-6, -7), s(-12, -5), s(-1, 4), s(8, 12), s(1, 2), ],
];

pub const DEFENDING_PAWN_LOCATIONS: [[ScoreTuple; 8]; 18] = [
  [s(6, 7), s(-5, -10), s(9, -6), s(-4, 1), s(10, -9), s(-1, -6), s(-5, 4), s(9, -9), ],
  [s(9, 1), s(-4, -3), s(-11, -4), s(-12, -2), s(10, -2), s(-7, 6), s(-6, 12), s(4, -7), ],
  [s(-4, -10), s(10, 11), s(-1, -1), s(12, -11), s(-3, -12), s(-4, 2), s(-9, 4), s(-6, -11), ],
  [s(-11, -11), s(2, 1), s(9, -10), s(2, -9), s(2, 8), s(-2, 11), s(2, 3), s(11, -2), ],
  [s(4, 1), s(10, 6), s(-3, 8), s(1, -7), s(-9, 6), s(-1, -1), s(10, -8), s(10, 3), ],
  [s(11, 11), s(0, 12), s(-9, 3), s(-10, -12), s(-8, 3), s(-11, -2), s(-1, -3), s(-8, 0), ],
  [s(-4, 3), s(2, 7), s(1, -4), s(0, -7), s(1, -2), s(0, -6), s(-5, -2), s(11, 8), ],
  [s(-5, 1), s(5, 4), s(6, 9), s(4, -4), s(-2, -11), s(3, -1), s(-10, 7), s(9, -4), ],
  [s(-2, -8), s(-2, -1), s(2, 0), s(6, -4), s(-7, -2), s(-9, 10), s(1, 10), s(-3, 1), ],
  [s(5, 4), s(7, -10), s(-5, -1), s(7, -9), s(-7, -3), s(5, -6), s(-6, -8), s(-11, 6), ],
  [s(-11, 6), s(-3, -1), s(-11, -2), s(-7, 0), s(-11, -7), s(3, -6), s(3, 7), s(-8, -7), ],
  [s(-7, -9), s(1, -10), s(0, -10), s(-2, 2), s(-1, 10), s(-11, 9), s(-3, 7), s(-9, 2), ],
  [s(10, 0), s(6, 8), s(10, -5), s(-6, -5), s(7, -11), s(-4, 9), s(4, 7), s(7, -12), ],
  [s(-1, 7), s(-11, -9), s(7, -2), s(-7, -5), s(8, -2), s(-7, -9), s(0, -4), s(-12, -3), ],
  [s(-8, -5), s(-2, -4), s(-3, 5), s(-12, 4), s(12, 3), s(-1, -9), s(-11, 10), s(8, 6), ],
  [s(6, 12), s(4, 0), s(12, 5), s(3, 3), s(-9, 0), s(-5, -3), s(8, 8), s(0, 0), ],
  [s(-1, 6), s(-2, -4), s(-7, 0), s(0, -7), s(-4, 8), s(-11, -8), s(1, 6), s(3, 2), ],
  [s(-5, 4), s(-4, 8), s(-12, 4), s(0, -7), s(-6, 9), s(-2, -10), s(-6, 0), s(9, -7), ],
];

pub const HIDDEN_BIASES: [ScoreTuple; 8] = 
  [s(-2428, -549), s(628, -769), s(-3364, -658), s(2168, -801), s(-685, -1006), s(-601, -1908), s(-862, -667), s(-320, -786), ];

pub const OUTPUT_WEIGHTS: [ScoreTuple; 8] = 
  [s(345, -563), s(188, -165), s(493, 678), s(-161, -375), s(-565, 267), s(555, -217), s(550, -320), s(266, 697), ];

pub const OUTPUT_BIAS: ScoreTuple = s(0, 0);

/*
fen: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
desc: startpos
output: S(-12.700624359361145, 0.05157874674895893) - S(-12.700624359361145, 0.05157874674895893)
= S(0, 0)

fen: r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
desc: kiwipete
output: S(22.4211632783269, 0.05157874674895893) - S(-0.025632858108570786, -10.626514863767879)
= S(22.446796136435474, 10.678093610516838)

fen: r1bq1b1r/ppp2kpp/2n5/3np3/2B5/8/PPPP1PPP/RNBQK2R w KQ - 0 7
desc: fried liver attack
output: S(-5.173741912441741, 0.05157874674895893) - S(-0.025632858108570786, -4.814613877889286)
= S(-5.148109054333171, 4.866192624638245)

fen: 1r2r1k1/pbp1qpp1/1p1p4/4nPR1/4P3/P1N4P/1PPQB3/1K1R4 w - - 1 24
desc: wahoo vs akimbo
output: S(33.896122422221275, -10.669948980852116) - S(-10.883481670355202, 0.05157874674895893)
= S(44.77960409257648, -10.721527727601075)

fen: 2kr3r/ppp1qppp/2b2n2/4p3/4P3/P1P2Q2/P1B2PPP/R1B2RK1 w - - 7 14
output: S(-9.032217624774349, 0.05157874674895893) - S(-3.234579708275637, 0.05157874674895893)
= S(-5.7976379164987115, 0)

fen: rnbq1b1r/ppPknQ1p/3pp3/1B6/5pp1/BP2P3/P1PK1PPP/RN4NR b - - 4 11
output: S(21.558016742157886, 0.05157874674895893) - S(-4.551614102819155, 0.658680128656735)
= S(26.10963084497704, -0.6071013819077761)

fen: 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1
output: S(-16.77403710759248, 0.05157874674895893) - S(-9.860413496130407, 0.05157874674895893)
= S(-6.913623611462073, 0)

fen: 8/3kp1p1/8/1p6/4PP2/5K2/1P6/8 w - - 0 1
output: S(-14.232479265508765, 0.05157874674895893) - S(-14.232479265508765, 0.05157874674895893)
= S(0, 0)

fen: 2k1n3/3bp1p1/8/1p6/4PP2/5K2/1P2R3/8 w - - 0 1
output: S(-9.803197245160682, 0.05157874674895893) - S(44.644654542807814, -1.2941655941975814)
= S(-54.447851787968496, 1.3457443409465404)

fen: 8/8/3bk2p/1r2p1pP/p1p3P1/P1B1K3/1PP5/5R2 b - - 25 52
output: S(56.638036218885226, -12.09943200095072) - S(-23.03039102276338, 0.05157874674895893)
= S(79.6684272416486, -12.15101074769968)

*/
