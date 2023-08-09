#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{eval::{evaluation::ScoreTuple, piece_tables::{Pst, Prt}}, board::board_representation::NUM_PIECES};

use super::piece_tables::SafetyPrt;

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(190, 300), s(229, 348), s(294, 376), s(347, 352), s(356, 369), s(250, 346), s(219, 351), s(248, 261), 
  s(316, 338), s(350, 351), s(376, 340), s(361, 354), s(384, 337), s(380, 337), s(334, 347), s(338, 309), 
  s(338, 336), s(382, 342), s(404, 365), s(400, 366), s(383, 366), s(424, 347), s(373, 335), s(358, 315), 
  s(370, 342), s(382, 361), s(397, 381), s(430, 382), s(404, 382), s(419, 376), s(380, 360), s(392, 328), 
  s(364, 348), s(379, 354), s(391, 382), s(405, 381), s(402, 385), s(400, 373), s(396, 355), s(370, 351), 
  s(354, 327), s(366, 348), s(382, 356), s(383, 371), s(398, 371), s(387, 347), s(383, 342), s(357, 341), 
  s(335, 309), s(345, 328), s(355, 340), s(374, 339), s(374, 341), s(369, 338), s(355, 320), s(353, 327), 
  s(298, 298), s(332, 320), s(334, 318), s(344, 328), s(349, 325), s(351, 314), s(329, 332), s(337, 300), 
]),
// Bishop PST
Pst::new([
  s(362, 400), s(337, 393), s(288, 399), s(261, 405), s(282, 402), s(278, 399), s(339, 400), s(349, 379), 
  s(369, 371), s(380, 383), s(379, 374), s(333, 382), s(351, 376), s(371, 385), s(343, 390), s(333, 375), 
  s(385, 385), s(409, 373), s(399, 381), s(399, 370), s(382, 375), s(404, 388), s(379, 387), s(391, 375), 
  s(377, 375), s(398, 379), s(402, 383), s(434, 393), s(408, 392), s(413, 387), s(386, 380), s(380, 376), 
  s(385, 365), s(395, 378), s(401, 389), s(419, 391), s(415, 388), s(392, 387), s(377, 381), s(390, 356), 
  s(389, 365), s(410, 376), s(414, 381), s(405, 384), s(403, 391), s(405, 383), s(401, 373), s(396, 359), 
  s(408, 363), s(408, 353), s(415, 358), s(395, 370), s(398, 372), s(410, 367), s(416, 364), s(400, 347), 
  s(379, 354), s(408, 375), s(381, 370), s(377, 365), s(383, 366), s(374, 378), s(393, 358), s(398, 344), 
]),
// Rook PST
Pst::new([
  s(587, 692), s(608, 685), s(575, 704), s(596, 690), s(613, 683), s(572, 694), s(573, 696), s(615, 678), 
  s(563, 690), s(565, 695), s(576, 699), s(589, 687), s(577, 684), s(580, 688), s(575, 682), s(580, 681), 
  s(547, 683), s(567, 683), s(562, 681), s(564, 681), s(580, 675), s(565, 675), s(616, 667), s(566, 667), 
  s(524, 686), s(551, 675), s(544, 690), s(548, 682), s(556, 670), s(540, 676), s(544, 673), s(522, 674), 
  s(516, 670), s(523, 673), s(531, 675), s(545, 668), s(547, 667), s(509, 678), s(539, 668), s(510, 668), 
  s(517, 659), s(525, 663), s(535, 652), s(537, 656), s(541, 655), s(535, 653), s(553, 641), s(522, 642), 
  s(507, 653), s(527, 653), s(537, 654), s(541, 651), s(545, 643), s(531, 650), s(554, 635), s(488, 653), 
  s(522, 656), s(525, 652), s(530, 655), s(538, 649), s(537, 645), s(527, 649), s(514, 653), s(511, 643), 
]),
// Queen PST
Pst::new([
  s(989, 1241), s(1019, 1233), s(1023, 1262), s(1049, 1245), s(1043, 1253), s(1047, 1236), s(1111, 1167), s(1054, 1220), 
  s(1036, 1106), s(1013, 1134), s(1031, 1147), s(1007, 1182), s(987, 1209), s(1032, 1155), s(1059, 1112), s(1091, 1108), 
  s(1040, 1089), s(1023, 1104), s(1039, 1115), s(1028, 1146), s(1038, 1139), s(1054, 1117), s(1069, 1071), s(1068, 1085), 
  s(1030, 1096), s(1027, 1102), s(1032, 1103), s(1028, 1125), s(1044, 1117), s(1048, 1101), s(1052, 1107), s(1052, 1087), 
  s(1038, 1061), s(1030, 1100), s(1032, 1110), s(1031, 1134), s(1039, 1121), s(1033, 1101), s(1050, 1081), s(1050, 1074), 
  s(1039, 1057), s(1047, 1071), s(1041, 1095), s(1037, 1101), s(1036, 1113), s(1054, 1078), s(1063, 1067), s(1055, 1057), 
  s(1044, 1042), s(1044, 1050), s(1054, 1042), s(1056, 1052), s(1053, 1063), s(1063, 1024), s(1060, 1026), s(1066, 997), 
  s(1033, 1042), s(1034, 1037), s(1038, 1034), s(1048, 1019), s(1045, 1026), s(1027, 1026), s(1044, 996), s(1035, 1023), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(157, 242), s(155, 235), s(156, 236), s(177, 217), s(141, 227), s(164, 217), s(97, 246), s(85, 259), 
  s(89, 167), s(82, 158), s(116, 151), s(118, 141), s(130, 121), s(178, 134), s(160, 156), s(128, 154), 
  s(69, 151), s(77, 133), s(87, 129), s(99, 109), s(114, 115), s(111, 116), s(93, 123), s(86, 126), 
  s(66, 125), s(66, 121), s(85, 116), s(103, 108), s(101, 109), s(105, 107), s(81, 104), s(84, 107), 
  s(69, 122), s(78, 116), s(86, 117), s(93, 123), s(95, 124), s(102, 116), s(97, 101), s(88, 104), 
  s(67, 130), s(73, 122), s(73, 131), s(82, 133), s(79, 140), s(115, 122), s(99, 107), s(81, 107), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-26, 51), s(-49, 115), s(20, 113), s(-56, 148), s(-16, 129), s(-49, 143), s(-15, 143), s(58, 48), 
  s(-84, 164), s(-23, 196), s(-95, 210), s(69, 195), s(-19, 200), s(-24, 218), s(16, 198), s(-67, 182), 
  s(-144, 91), s(-34, 102), s(-44, 113), s(-72, 122), s(-21, 122), s(67, 109), s(31, 109), s(-60, 79), 
  s(-71, 13), s(-31, 39), s(-52, 46), s(-90, 57), s(-103, 57), s(-58, 46), s(-80, 46), s(-158, 22), 
  s(-97, -25), s(-36, -2), s(-51, 13), s(-78, 25), s(-71, 23), s(-34, 2), s(-82, 0), s(-158, -10), 
  s(-38, -51), s(5, -36), s(-18, -21), s(-14, -14), s(-14, -15), s(-10, -25), s(-14, -36), s(-73, -45), 
  s(32, -70), s(23, -46), s(8, -38), s(-19, -29), s(-15, -27), s(-3, -38), s(31, -51), s(23, -71), 
  s(49, -110), s(82, -90), s(57, -66), s(-22, -49), s(24, -60), s(1, -53), s(56, -77), s(49, -111), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(57, 142), s(55, 135), s(56, 136), s(77, 117), s(41, 127), s(64, 117), s(-2, 146), s(-14, 159), 
  s(48, 135), s(58, 144), s(41, 117), s(38, 100), s(27, 124), s(9, 117), s(-40, 130), s(-49, 153), 
  s(37, 57), s(32, 64), s(30, 58), s(14, 66), s(1, 59), s(27, 57), s(-15, 83), s(-10, 80), 
  s(22, 24), s(11, 37), s(-11, 39), s(-5, 37), s(-20, 36), s(-1, 35), s(5, 54), s(-1, 47), 
  s(12, -1), s(-6, 24), s(-26, 24), s(-18, 16), s(-21, 14), s(-1, 14), s(-14, 42), s(15, 19), 
  s(0, 12), s(-1, 21), s(-9, 11), s(-9, 6), s(-4, 4), s(-7, 10), s(1, 20), s(-2, 21), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(-32, -185),
  s(4, -116),
  s(-2, -51),
  s(-10, -20),
  s(-9, 1),
  s(-11, 1),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(50, -25),
  s(10, -31),
  s(-1, -27),
  s(-16, -16),
  s(-26, -19),
  s(-14, -19),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(96, 269),
  s(164, 169),
  s(52, 52),
  s(18, 16),
  s(-5, -3),
  s(4, -9),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(24, 71);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(9, 52), s(10, 67), s(8, 71), s(9, 71), s(8, 75), s(8, 71), s(4, 68), s(2, 60), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(3, 24), s(9, 45), s(11, 57), s(14, 67), s(14, 75), s(14, 79), s(12, 81), s(7, 86), s(9, 82), s(10, 77), s(15, 77), s(-29, 107), s(3, 69), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(1, 76), s(2, 93), s(2, 116), s(2, 124), s(4, 135), s(6, 140), s(6, 147), s(12, 151), s(13, 155), s(14, 158), s(18, 163), s(23, 165), s(32, 161), s(30, 157), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(61, 144), s(60, 202), s(65, 245), s(70, 262), s(74, 274), s(78, 277), s(80, 298), s(83, 304), s(86, 304), s(90, 307), s(91, 319), s(94, 313), s(98, 318), s(97, 321), s(102, 324), s(100, 328), s(110, 321), s(114, 323), s(135, 305), s(142, 307), s(191, 274), s(178, 271), s(204, 254), s(235, 246), s(242, 233), s(177, 248), s(153, 237), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(14, 22), s(24, 32), s(34, 37), s(43, 41), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(4, 11), s(11, 15), s(16, 18), s(21, 22), s(22, 24), s(27, 22), s(30, 28), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(7, 3), s(11, 7), s(19, 11), s(25, 19), s(29, 24), s(37, 24), s(42, 30), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(-13, 101), s(-12, 112), s(-13, 117), s(-13, 131), s(-14, 145), s(-16, 147), s(-17, 155), s(-17, 163), s(-18, 175), s(-18, 174), s(-24, 184), s(-5, 168), s(-36, 214), s(9, 179), 
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(80, 37);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(78, 62);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(125, 6);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(103, -28);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(38, 37);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(80, 18);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(64, -41);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(23, 32);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(70, 34);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(92, 25);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(85, 13);

pub const PASSER_SQ_RULE_BONUS: ScoreTuple = s(-32, 85);

pub const TEMPO_BONUS: ScoreTuple = s(35, 24);

// KING SAFETY FEATURES
pub const ATTACKS: [[ScoreTuple; 8]; (NUM_PIECES - 1) as usize] = [
// Knight attacks
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Bishop attacks
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Rook attacks
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Queen attacks
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Pawn attacks
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
];

pub const DEFENSES: [[ScoreTuple; 8]; (NUM_PIECES - 1) as usize] = [
// Knight defenses
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Bishop defenses
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Rook defenses
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Queen defenses
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
// Pawn defenses
[
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
],
];

pub const ENEMY_KING_RANK: SafetyPrt = SafetyPrt::new([
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
]);

pub const TROPISM_BONUS: [ScoreTuple; 8] = [
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
];

pub const ATTACKING_PAWN_LOCATIONS: [[ScoreTuple; 8]; 18] = [
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
];

pub const DEFENDING_PAWN_LOCATIONS: [[ScoreTuple; 8]; 18] = [
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
  [s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24)],
];

pub const HIDDEN_BIASES: [ScoreTuple; 8] = [
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
];

pub const OUTPUT_WEIGHTS: [ScoreTuple; 8] = [
  s(-125, 0), s(78, -23), s(97, -18), s(103, -24), s(108, -22), s(105, -23), s(106, -24), s(108, -24),
];

pub const OUTPUT_BIAS: ScoreTuple = s(-125, 0);
