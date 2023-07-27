#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{eval::{evaluation::ScoreTuple, piece_tables::{Pst, Prt}}, board::board_representation::NUM_PIECES};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(196, 296), s(238, 349), s(296, 382), s(344, 362), s(361, 377), s(247, 356), s(216, 355), s(250, 262), 
  s(322, 335), s(353, 356), s(379, 349), s(364, 362), s(385, 345), s(384, 344), s(338, 350), s(339, 314), 
  s(344, 333), s(386, 349), s(407, 375), s(401, 375), s(383, 376), s(427, 357), s(373, 343), s(363, 320), 
  s(373, 342), s(385, 367), s(400, 391), s(433, 390), s(406, 390), s(422, 384), s(383, 365), s(395, 333), 
  s(367, 347), s(382, 357), s(393, 390), s(408, 386), s(404, 391), s(402, 381), s(399, 360), s(373, 352), 
  s(356, 325), s(368, 350), s(385, 360), s(385, 375), s(401, 374), s(389, 351), s(385, 342), s(360, 341), 
  s(338, 302), s(347, 324), s(357, 342), s(376, 339), s(377, 341), s(372, 339), s(357, 320), s(357, 323), 
  s(300, 291), s(336, 307), s(338, 313), s(348, 322), s(352, 320), s(355, 309), s(334, 322), s(340, 291), 
]),
// Bishop PST
Pst::new([
  s(364, 402), s(337, 401), s(287, 409), s(261, 416), s(284, 412), s(280, 409), s(351, 405), s(352, 387), 
  s(371, 373), s(380, 388), s(379, 382), s(333, 392), s(348, 387), s(375, 392), s(342, 397), s(337, 379), 
  s(385, 386), s(410, 378), s(399, 390), s(399, 379), s(382, 386), s(402, 398), s(379, 396), s(391, 384), 
  s(377, 377), s(398, 385), s(400, 391), s(432, 404), s(406, 403), s(413, 396), s(386, 388), s(379, 382), 
  s(384, 365), s(393, 383), s(400, 395), s(418, 398), s(414, 396), s(391, 395), s(376, 387), s(388, 362), 
  s(388, 363), s(409, 377), s(412, 385), s(404, 388), s(402, 396), s(404, 388), s(400, 378), s(396, 361), 
  s(408, 358), s(406, 352), s(413, 358), s(394, 372), s(397, 373), s(409, 369), s(414, 366), s(401, 346), 
  s(379, 345), s(407, 371), s(380, 364), s(377, 360), s(383, 361), s(374, 374), s(393, 355), s(396, 340), 
]),
// Rook PST
Pst::new([
  s(588, 707), s(608, 704), s(573, 725), s(592, 715), s(609, 707), s(574, 716), s(581, 713), s(613, 698), 
  s(559, 708), s(560, 716), s(571, 723), s(584, 711), s(568, 710), s(570, 715), s(571, 703), s(575, 702), 
  s(543, 700), s(562, 704), s(558, 703), s(558, 706), s(572, 702), s(555, 701), s(607, 692), s(558, 690), 
  s(521, 703), s(547, 694), s(539, 713), s(541, 707), s(548, 697), s(530, 704), s(537, 698), s(515, 698), 
  s(512, 685), s(520, 691), s(528, 696), s(540, 690), s(542, 691), s(500, 704), s(530, 692), s(504, 690), 
  s(514, 672), s(521, 678), s(532, 670), s(532, 677), s(536, 677), s(528, 678), s(545, 664), s(517, 663), 
  s(503, 665), s(523, 666), s(533, 670), s(536, 670), s(540, 663), s(525, 672), s(548, 655), s(483, 671), 
  s(518, 666), s(522, 663), s(526, 667), s(534, 663), s(533, 660), s(523, 664), s(509, 669), s(507, 657), 
]),
// Queen PST
Pst::new([
  s(1011, 1224), s(1031, 1226), s(1031, 1260), s(1064, 1239), s(1050, 1256), s(1042, 1246), s(1126, 1159), s(1071, 1210), 
  s(1055, 1120), s(1033, 1149), s(1050, 1166), s(1027, 1200), s(1006, 1231), s(1053, 1177), s(1076, 1130), s(1109, 1129), 
  s(1059, 1104), s(1044, 1118), s(1060, 1131), s(1048, 1162), s(1057, 1161), s(1073, 1138), s(1086, 1098), s(1082, 1107), 
  s(1049, 1113), s(1046, 1119), s(1052, 1120), s(1046, 1146), s(1062, 1139), s(1066, 1125), s(1069, 1129), s(1069, 1111), 
  s(1056, 1077), s(1049, 1114), s(1051, 1127), s(1050, 1151), s(1058, 1138), s(1050, 1120), s(1068, 1099), s(1067, 1094), 
  s(1058, 1068), s(1066, 1085), s(1061, 1109), s(1057, 1115), s(1055, 1127), s(1072, 1095), s(1081, 1081), s(1073, 1072), 
  s(1063, 1050), s(1063, 1061), s(1072, 1056), s(1075, 1066), s(1072, 1078), s(1081, 1042), s(1078, 1043), s(1084, 1009), 
  s(1053, 1050), s(1053, 1047), s(1056, 1045), s(1066, 1033), s(1063, 1040), s(1047, 1040), s(1065, 1006), s(1054, 1034), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(138, 291), s(146, 284), s(128, 291), s(147, 267), s(107, 278), s(141, 282), s(106, 310), s(105, 308), 
  s(85, 180), s(73, 177), s(100, 181), s(109, 160), s(122, 137), s(143, 203), s(118, 207), s(117, 183), 
  s(68, 158), s(76, 140), s(85, 136), s(98, 111), s(112, 113), s(106, 132), s(93, 134), s(90, 136), 
  s(65, 129), s(66, 125), s(84, 119), s(102, 106), s(102, 106), s(101, 109), s(80, 106), s(83, 113), 
  s(69, 126), s(79, 119), s(87, 119), s(94, 124), s(97, 125), s(97, 115), s(102, 101), s(91, 107), 
  s(67, 133), s(74, 124), s(73, 135), s(84, 133), s(80, 141), s(111, 117), s(99, 107), s(87, 113), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-63, -28), s(-45, 19), s(11, 27), s(-90, 73), s(-38, 54), s(-69, 67), s(-35, 68), s(54, -30), 
  s(-90, 32), s(-34, 61), s(-107, 81), s(70, 60), s(-6, 74), s(-51, 106), s(-26, 87), s(-92, 67), 
  s(-175, 1), s(-48, 10), s(-60, 35), s(-93, 50), s(-29, 51), s(39, 51), s(-3, 42), s(-100, 11), 
  s(-105, -69), s(-68, -36), s(-78, -17), s(-107, -1), s(-119, 5), s(-90, -3), s(-125, -10), s(-201, -38), 
  s(-136, -68), s(-79, -41), s(-98, -8), s(-111, 7), s(-113, 9), s(-96, -8), s(-142, -18), s(-208, -34), 
  s(-30, 35), s(5, 58), s(-18, 83), s(-10, 97), s(-8, 99), s(-6, 85), s(-8, 73), s(-58, 59), 
  s(54, 1), s(38, 32), s(28, 54), s(0, 70), s(8, 77), s(23, 65), s(55, 44), s(52, 18), 
  s(72, -85), s(104, -62), s(79, -20), s(0, 4), s(50, 0), s(28, 5), s(82, -30), s(76, -67), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(38, 191), s(46, 184), s(28, 191), s(47, 167), s(7, 178), s(41, 182), s(6, 210), s(5, 208), 
  s(26, 203), s(37, 204), s(26, 164), s(28, 135), s(12, 156), s(-8, 136), s(-61, 170), s(-70, 195), 
  s(21, 112), s(19, 107), s(20, 84), s(10, 77), s(-1, 69), s(13, 75), s(-23, 105), s(-23, 108), 
  s(9, 67), s(6, 60), s(-14, 47), s(-6, 41), s(-20, 40), s(-4, 42), s(6, 65), s(-6, 61), 
  s(7, 21), s(-5, 29), s(-27, 26), s(-18, 16), s(-21, 14), s(-4, 16), s(-9, 43), s(14, 25), 
  s(-1, 17), s(0, 22), s(-10, 12), s(-9, 5), s(1, 0), s(-13, 13), s(0, 21), s(-1, 19), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(-6, -179),
  s(0, -112),
  s(0, -59),
  s(-7, -30),
  s(-5, -7),
  s(-9, -6),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(39, -23),
  s(5, -30),
  s(-2, -27),
  s(-15, -15),
  s(-27, -19),
  s(-14, -20),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(83, 299),
  s(159, 170),
  s(52, 51),
  s(18, 16),
  s(-6, -6),
  s(4, -11),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(24, 71);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(6, 46), s(8, 59), s(6, 61), s(8, 60), s(7, 63), s(8, 57), s(5, 53), s(4, 41), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(3, 17), s(10, 36), s(12, 47), s(15, 57), s(15, 65), s(16, 68), s(13, 71), s(8, 75), s(9, 71), s(10, 66), s(16, 65), s(-31, 94), s(1, 55), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(3, 61), s(5, 77), s(5, 98), s(5, 104), s(7, 114), s(9, 120), s(10, 126), s(15, 130), s(16, 135), s(17, 138), s(20, 144), s(24, 148), s(33, 143), s(27, 142), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(42, 187), s(41, 231), s(46, 269), s(50, 286), s(53, 297), s(57, 300), s(59, 322), s(62, 329), s(65, 328), s(69, 332), s(69, 344), s(72, 339), s(76, 343), s(75, 347), s(80, 349), s(78, 353), s(88, 346), s(93, 345), s(115, 327), s(125, 326), s(174, 292), s(166, 286), s(199, 262), s(254, 239), s(272, 222), s(190, 261), s(159, 247), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(13, 22), s(24, 32), s(34, 38), s(42, 43), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(4, 12), s(11, 17), s(16, 19), s(20, 24), s(21, 26), s(26, 24), s(29, 32), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(7, 4), s(11, 8), s(18, 13), s(24, 21), s(28, 26), s(35, 25), s(40, 30), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(-8, 66), s(-6, 76), s(-7, 80), s(-8, 94), s(-8, 107), s(-10, 108), s(-11, 117), s(-11, 124), s(-12, 136), s(-12, 134), s(-17, 142), s(0, 126), s(-30, 175), s(-5, 153), 
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(80, 36);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(79, 60);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(126, 3);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(102, -27);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(37, 39);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(81, 18);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(63, -39);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(23, 34);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(71, 35);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(91, 25);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(85, 16);

pub const TEMPO_BONUS: ScoreTuple = s(33, 30);

// KING SAFETY FEATURES
pub const ATTACKS: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight attacks
[
  s(0, 3), s(5, -4), s(9, -1), s(13, -4), s(11, -2), s(12, -4), s(11, -2), s(13, -3), s(13, -3), s(11, -1), s(10, -1), s(11, -2), s(8, 0), s(11, -1), s(9, -1), s(8, -1), s(8, -1), s(6, 0), s(5, -1), s(11, -4), s(9, -3), s(7, -4), s(2, -4), s(9, -7), s(2, -6), s(-9, -5), s(-4, -4), s(-112, -3), 
],
// Bishop attacks
[
  s(6, 1), s(7, -2), s(12, -3), s(11, -3), s(10, -1), s(12, -3), s(11, -3), s(11, -3), s(12, -3), s(11, -3), s(11, -3), s(10, -3), s(9, -2), s(9, -2), s(9, -2), s(9, -3), s(11, -3), s(7, -2), s(7, -2), s(2, -1), s(4, -1), s(5, -4), s(2, -1), s(-1, -2), s(1, -4), s(11, -7), s(12, -7), s(-16, -7), 
],
// Rook attacks
[
  s(2, 0), s(3, -1), s(8, -4), s(9, -6), s(8, -5), s(7, -5), s(8, -5), s(7, -4), s(8, -5), s(7, -5), s(7, -4), s(8, -4), s(6, -3), s(6, -3), s(6, -3), s(6, -2), s(5, -2), s(4, -2), s(6, -2), s(4, -1), s(2, -1), s(7, -2), s(3, -1), s(7, -2), s(7, -1), s(6, -1), s(-23, 6), s(22, -6), 
],
// Queen attacks
[
  s(2, -3), s(3, -5), s(6, -6), s(8, -8), s(7, -6), s(7, -5), s(7, -6), s(7, -4), s(7, -5), s(8, -4), s(7, -2), s(9, -2), s(8, -1), s(10, -2), s(9, -2), s(9, -1), s(9, 0), s(10, 0), s(11, 0), s(15, -1), s(14, -1), s(9, 0), s(19, -2), s(13, -1), s(11, 2), s(6, 1), s(37, -5), s(40, -6), 
],
// Pawn attacks
[
  s(-3, 6), s(5, 0), s(3, -1), s(2, -1), s(1, 0), s(1, 0), s(1, 0), s(1, -1), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(1, 0), s(1, 0), s(2, -1), s(3, -2), s(4, -2), s(5, -2), s(2, -2), s(6, -4), s(8, -4), s(5, -4), s(10, -5), s(7, -4), s(-1, -2), s(37, -5), s(-4, 0), 
],
];

pub const DEFENSES: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight defenses
[
  s(-9, 6), s(-1, 1), s(-3, 1), s(-3, 0), s(-4, 0), s(-3, 0), s(-3, 0), s(-3, 0), s(-3, 0), s(-3, 0), s(-3, 0), s(-1, -1), s(-4, 0), s(-4, 0), s(-2, 0), s(-4, 0), s(-5, 1), s(-2, 0), s(-2, 1), s(-3, 2), s(-7, 4), s(-5, 4), s(-5, 5), s(-8, 8), s(-17, 11), s(-21, 14), s(-17, 9), s(0, 20), 
],
// Bishop defenses
[
  s(-2, 1), s(-1, 0), s(-3, 0), s(-5, 1), s(-3, 0), s(-3, 0), s(-5, 1), s(-3, 0), s(-3, 1), s(-3, 1), s(-2, 0), s(-3, 0), s(-3, 0), s(-5, 0), s(-3, 0), s(-3, 0), s(-2, 0), s(0, 0), s(-5, 1), s(-1, 0), s(0, 0), s(-4, 2), s(-4, 2), s(-5, 3), s(-1, 5), s(-21, 8), s(-18, 6), s(-13, 13), 
],
// Rook defenses
[
  s(-9, 6), s(-1, 2), s(-5, 4), s(-6, 3), s(-3, 2), s(-4, 3), s(-2, 2), s(-3, 2), s(-3, 2), s(-3, 1), s(-2, 1), s(-2, 1), s(0, 0), s(-1, 1), s(-1, 0), s(-1, 0), s(-1, 0), s(0, 0), s(0, 0), s(1, 0), s(-2, 2), s(3, 0), s(1, 1), s(-7, 4), s(-16, 7), s(-15, 6), s(8, 1), s(-12, 6), 
],
// Queen defenses
[
  s(9, -17), s(-2, 3), s(-1, 1), s(-1, 1), s(-1, 0), s(-1, 1), s(-2, 3), s(-2, 3), s(-3, 3), s(-3, 3), s(-2, 2), s(-2, 2), s(-1, 0), s(-1, 1), s(0, 0), s(1, -1), s(2, -2), s(2, -1), s(6, -4), s(2, -1), s(5, -1), s(7, -2), s(2, 1), s(8, -1), s(26, -3), s(45, -3), s(-23, 13), s(9, -1), 
],
// Pawn defenses
[
  s(-10, -2), s(-21, 2), s(-16, 1), s(-13, 1), s(-11, 1), s(-11, 0), s(-10, 0), s(-10, 1), s(-7, 0), s(-6, 0), s(-4, -1), s(-4, -1), s(-1, -2), s(-1, -2), s(-1, -2), s(0, -2), s(0, -2), s(-1, -2), s(-1, -2), s(0, -3), s(0, -3), s(-1, -2), s(0, -3), s(0, -3), s(2, -4), s(27, -8), s(2, -7), s(2, -8), 
],
];

pub const ENEMY_KING_RANK: Prt = Prt::new([
  s(106, 133),
  s(85, 151),
  s(73, 156),
  s(53, 116),
  s(49, 107),
  s(51, 127),
  s(73, 143),
  s(71, 153),
]);

pub const TROPHISM_BONUS: [ScoreTuple; 160] = [
  s(-134, 19), s(150, 1), s(139, 10), s(142, 8), s(138, 13), s(139, 10), s(136, 13), s(135, 14), s(137, 10), s(133, 13), s(132, 14), s(127, 14), s(121, 17), s(126, 15), s(123, 16), s(118, 19), s(115, 21), s(118, 17), s(114, 20), s(108, 23), s(112, 21), s(101, 27), s(102, 26), s(98, 27), s(101, 27), s(97, 29), s(98, 28), s(95, 29), s(90, 33), s(90, 31), s(90, 32), s(87, 32), s(83, 35), s(87, 34), s(84, 35), s(77, 39), s(79, 39), s(77, 39), s(75, 40), s(71, 44), s(72, 41), s(72, 44), s(67, 45), s(66, 47), s(67, 45), s(65, 48), s(59, 51), s(60, 49), s(60, 51), s(58, 52), s(56, 53), s(56, 52), s(54, 54), s(52, 56), s(49, 58), s(48, 60), s(47, 59), s(41, 64), s(42, 63), s(40, 64), s(37, 65), s(38, 66), s(39, 66), s(33, 69), s(35, 69), s(33, 70), s(30, 73), s(28, 73), s(25, 76), s(28, 74), s(24, 76), s(21, 80), s(21, 76), s(23, 76), s(14, 84), s(14, 82), s(15, 83), s(13, 83), s(14, 85), s(12, 85), s(8, 89), s(11, 87), s(4, 91), s(1, 93), s(7, 92), s(0, 91), s(1, 97), s(-3, 94), s(0, 90), s(-2, 95), s(-6, 102), s(-7, 103), s(-9, 102), s(-8, 93), s(-6, 92), s(-9, 109), s(-12, 101), s(-12, 105), s(-20, 116), s(-13, 105), s(-15, 112), s(-19, 108), s(-7, 100), s(-13, 108), s(-16, 112), s(-27, 99), s(-25, 117), s(-13, 116), s(-23, 112), s(-25, 106), s(-22, 105), s(-16, 104), s(-20, 101), s(-20, 110), s(-15, 43), s(-21, 121), s(-35, 59), s(-40, 38), s(-29, 7), s(-26, 77), s(-15, 49), s(-25, 87), s(-33, 33), s(-32, -4), s(-23, 66), s(-7, 31), s(-48, -14), s(-66, 35), s(-4, 5), s(-66, 19), s(-37, 10), s(-47, 11), s(7, 3), s(-52, 0), s(-21, 0), s(-42, 0), s(-10, 1), s(20, 3), s(0, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), 
];

pub const PAWN_STORM_BONUS: [ScoreTuple; 160] = [
  s(-134, 19), s(150, 1), s(139, 10), s(142, 8), s(138, 13), s(139, 10), s(136, 13), s(135, 14), s(137, 10), s(133, 13), s(132, 14), s(127, 14), s(121, 17), s(126, 15), s(123, 16), s(118, 19), s(115, 21), s(118, 17), s(114, 20), s(108, 23), s(112, 21), s(101, 27), s(102, 26), s(98, 27), s(101, 27), s(97, 29), s(98, 28), s(95, 29), s(90, 33), s(90, 31), s(90, 32), s(87, 32), s(83, 35), s(87, 34), s(84, 35), s(77, 39), s(79, 39), s(77, 39), s(75, 40), s(71, 44), s(72, 41), s(72, 44), s(67, 45), s(66, 47), s(67, 45), s(65, 48), s(59, 51), s(60, 49), s(60, 51), s(58, 52), s(56, 53), s(56, 52), s(54, 54), s(52, 56), s(49, 58), s(48, 60), s(47, 59), s(41, 64), s(42, 63), s(40, 64), s(37, 65), s(38, 66), s(39, 66), s(33, 69), s(35, 69), s(33, 70), s(30, 73), s(28, 73), s(25, 76), s(28, 74), s(24, 76), s(21, 80), s(21, 76), s(23, 76), s(14, 84), s(14, 82), s(15, 83), s(13, 83), s(14, 85), s(12, 85), s(8, 89), s(11, 87), s(4, 91), s(1, 93), s(7, 92), s(0, 91), s(1, 97), s(-3, 94), s(0, 90), s(-2, 95), s(-6, 102), s(-7, 103), s(-9, 102), s(-8, 93), s(-6, 92), s(-9, 109), s(-12, 101), s(-12, 105), s(-20, 116), s(-13, 105), s(-15, 112), s(-19, 108), s(-7, 100), s(-13, 108), s(-16, 112), s(-27, 99), s(-25, 117), s(-13, 116), s(-23, 112), s(-25, 106), s(-22, 105), s(-16, 104), s(-20, 101), s(-20, 110), s(-15, 43), s(-21, 121), s(-35, 59), s(-40, 38), s(-29, 7), s(-26, 77), s(-15, 49), s(-25, 87), s(-33, 33), s(-32, -4), s(-23, 66), s(-7, 31), s(-48, -14), s(-66, 35), s(-4, 5), s(-66, 19), s(-37, 10), s(-47, 11), s(7, 3), s(-52, 0), s(-21, 0), s(-42, 0), s(-10, 1), s(20, 3), s(0, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), 
];
