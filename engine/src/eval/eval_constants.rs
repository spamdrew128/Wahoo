#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{eval::{evaluation::ScoreTuple, piece_tables::{Pst, Prt}}, board::board_representation::NUM_PIECES};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(252, 288), s(360, 320), s(340, 383), s(392, 350), s(497, 335), s(307, 352), s(369, 303), s(319, 245), 
  s(346, 330), s(381, 358), s(472, 327), s(422, 354), s(445, 330), s(462, 322), s(432, 332), s(394, 301), 
  s(370, 328), s(452, 339), s(458, 371), s(473, 358), s(462, 354), s(553, 335), s(466, 323), s(432, 302), 
  s(431, 336), s(451, 360), s(449, 389), s(499, 381), s(467, 381), s(505, 369), s(456, 355), s(470, 315), 
  s(419, 346), s(445, 349), s(455, 385), s(468, 385), s(469, 381), s(464, 379), s(481, 352), s(434, 340), 
  s(414, 329), s(422, 360), s(443, 361), s(447, 374), s(463, 376), s(451, 350), s(457, 330), s(417, 337), 
  s(397, 298), s(399, 323), s(416, 338), s(438, 343), s(434, 345), s(438, 332), s(429, 306), s(417, 307), 
  s(309, 292), s(394, 295), s(383, 314), s(396, 330), s(407, 323), s(396, 321), s(395, 307), s(407, 263), 
]),
// Bishop PST
Pst::new([
  s(420, 398), s(406, 393), s(319, 410), s(273, 416), s(344, 402), s(322, 405), s(408, 394), s(408, 387), 
  s(421, 389), s(434, 395), s(410, 387), s(393, 381), s(426, 380), s(458, 380), s(408, 395), s(380, 382), 
  s(443, 383), s(475, 378), s(475, 384), s(464, 375), s(479, 370), s(506, 379), s(482, 383), s(480, 371), 
  s(435, 380), s(461, 385), s(456, 399), s(507, 396), s(470, 402), s(489, 388), s(453, 382), s(445, 386), 
  s(441, 374), s(450, 386), s(463, 395), s(478, 402), s(485, 390), s(449, 394), s(447, 380), s(435, 372), 
  s(445, 369), s(465, 380), s(465, 393), s(465, 392), s(463, 399), s(474, 385), s(452, 383), s(448, 370), 
  s(461, 351), s(464, 357), s(466, 364), s(450, 377), s(457, 378), s(469, 372), s(482, 360), s(447, 350), 
  s(426, 353), s(452, 369), s(432, 370), s(426, 375), s(442, 371), s(421, 385), s(438, 366), s(436, 351), 
]),
// Rook PST
Pst::new([
  s(664, 748), s(693, 738), s(635, 763), s(689, 746), s(672, 750), s(630, 755), s(689, 735), s(681, 740), 
  s(636, 752), s(641, 756), s(668, 751), s(691, 743), s(698, 728), s(685, 740), s(634, 752), s(645, 745), 
  s(612, 738), s(644, 737), s(642, 739), s(658, 733), s(659, 727), s(664, 724), s(706, 715), s(638, 721), 
  s(583, 739), s(629, 726), s(621, 742), s(638, 730), s(632, 728), s(623, 737), s(626, 724), s(602, 730), 
  s(577, 730), s(592, 730), s(608, 731), s(622, 726), s(643, 712), s(599, 725), s(628, 713), s(586, 719), 
  s(582, 710), s(600, 716), s(612, 708), s(619, 704), s(614, 714), s(618, 705), s(616, 706), s(584, 699), 
  s(567, 712), s(603, 702), s(608, 705), s(618, 708), s(622, 700), s(610, 705), s(612, 698), s(531, 716), 
  s(593, 705), s(604, 706), s(616, 704), s(621, 700), s(620, 698), s(598, 707), s(573, 714), s(583, 688), 
]),
// Queen PST
Pst::new([
  s(1184, 1191), s(1188, 1224), s(1180, 1248), s(1197, 1242), s(1295, 1194), s(1231, 1233), s(1297, 1166), s(1262, 1217), 
  s(1200, 1154), s(1169, 1207), s(1191, 1235), s(1177, 1261), s(1117, 1324), s(1225, 1228), s(1229, 1208), s(1289, 1158), 
  s(1218, 1141), s(1206, 1163), s(1226, 1162), s(1197, 1229), s(1248, 1211), s(1273, 1190), s(1275, 1151), s(1248, 1190), 
  s(1184, 1181), s(1208, 1171), s(1185, 1185), s(1189, 1219), s(1206, 1212), s(1225, 1197), s(1215, 1218), s(1219, 1190), 
  s(1216, 1128), s(1185, 1185), s(1210, 1173), s(1195, 1220), s(1211, 1189), s(1200, 1182), s(1226, 1170), s(1209, 1181), 
  s(1203, 1128), s(1224, 1114), s(1220, 1153), s(1219, 1148), s(1217, 1172), s(1222, 1145), s(1229, 1139), s(1219, 1148), 
  s(1208, 1099), s(1223, 1104), s(1233, 1100), s(1237, 1097), s(1239, 1108), s(1244, 1095), s(1226, 1073), s(1238, 1067), 
  s(1225, 1087), s(1213, 1088), s(1224, 1088), s(1230, 1083), s(1225, 1093), s(1189, 1099), s(1214, 1075), s(1202, 1058), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(154, 300), s(146, 298), s(134, 291), s(166, 265), s(131, 282), s(166, 258), s(85, 299), s(63, 316), 
  s(97, 166), s(101, 159), s(131, 155), s(133, 142), s(184, 135), s(199, 138), s(181, 159), s(139, 158), 
  s(82, 148), s(97, 136), s(103, 137), s(123, 115), s(123, 131), s(120, 131), s(105, 136), s(92, 136), 
  s(78, 127), s(75, 128), s(99, 123), s(124, 115), s(115, 121), s(114, 120), s(86, 119), s(87, 115), 
  s(85, 121), s(91, 121), s(104, 123), s(111, 131), s(118, 138), s(118, 131), s(128, 110), s(106, 110), 
  s(77, 129), s(91, 119), s(86, 138), s(101, 134), s(97, 153), s(133, 132), s(132, 111), s(94, 108), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-96, -105), s(75, -65), s(116, -49), s(-34, -12), s(-115, 12), s(-127, 35), s(65, 1), s(36, -48), 
  s(69, -27), s(11, 37), s(-65, 43), s(79, 18), s(-1, 43), s(-59, 77), s(20, 46), s(-53, 24), 
  s(-46, 23), s(28, 38), s(95, 29), s(14, 36), s(76, 39), s(120, 63), s(163, 52), s(-2, 18), 
  s(22, -7), s(0, 42), s(38, 39), s(-12, 51), s(-47, 59), s(-15, 55), s(-5, 49), s(-87, 17), 
  s(-125, 5), s(15, 6), s(2, 32), s(-58, 51), s(-66, 55), s(-20, 33), s(-44, 22), s(-90, -1), 
  s(30, -31), s(20, -3), s(9, 15), s(-14, 27), s(3, 25), s(15, 11), s(21, 0), s(-8, -18), 
  s(55, -49), s(47, -19), s(9, 5), s(-42, 21), s(-27, 21), s(-1, 8), s(38, -13), s(29, -37), 
  s(16, -92), s(53, -57), s(22, -29), s(-92, 0), s(-16, -26), s(-51, -1), s(24, -38), s(19, -78), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(54, 200), s(46, 198), s(34, 191), s(66, 165), s(31, 182), s(66, 158), s(-14, 199), s(-36, 216), 
  s(47, 210), s(26, 216), s(12, 179), s(15, 150), s(-31, 148), s(-6, 162), s(-65, 186), s(-63, 206), 
  s(26, 118), s(7, 109), s(16, 86), s(0, 81), s(0, 66), s(36, 68), s(-25, 113), s(-19, 110), 
  s(5, 68), s(-7, 63), s(-30, 48), s(-22, 46), s(-39, 48), s(-36, 49), s(0, 67), s(6, 60), 
  s(6, 25), s(-5, 33), s(-30, 26), s(-43, 31), s(-31, 15), s(9, 10), s(-23, 41), s(16, 24), 
  s(-3, 18), s(10, 24), s(-2, 10), s(-38, 24), s(-21, 12), s(-18, 14), s(-16, 30), s(-12, 25), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(56, -260),
  s(23, -150),
  s(1, -62),
  s(11, -38),
  s(-4, -2),
  s(-5, -13),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(42, -26),
  s(9, -24),
  s(-4, -26),
  s(-18, -14),
  s(-33, -18),
  s(-17, -18),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(79, 372),
  s(203, 136),
  s(48, 52),
  s(18, 16),
  s(-6, -7),
  s(6, -8),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(30, 70);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(4, 51), s(6, 58), s(4, 61), s(10, 56), s(10, 59), s(12, 55), s(8, 52), s(17, 33), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(10, 15), s(18, 32), s(20, 43), s(23, 53), s(23, 59), s(24, 65), s(22, 65), s(14, 71), s(21, 64), s(27, 59), s(50, 54), s(-10, 92), s(44, 43), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(7, 44), s(10, 57), s(18, 60), s(16, 71), s(21, 84), s(22, 90), s(25, 91), s(33, 92), s(32, 97), s(31, 100), s(38, 103), s(42, 106), s(44, 101), s(31, 101), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(4, 347), s(7, 312), s(6, 332), s(14, 330), s(17, 340), s(19, 344), s(19, 365), s(22, 375), s(27, 377), s(29, 384), s(32, 396), s(31, 394), s(36, 396), s(39, 397), s(37, 409), s(39, 415), s(51, 397), s(60, 397), s(80, 386), s(59, 400), s(124, 362), s(69, 383), s(176, 327), s(189, 297), s(289, 250), s(209, 307), s(178, 290), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(20, 23), s(31, 33), s(41, 37), s(51, 40), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(6, 11), s(14, 15), s(21, 19), s(25, 20), s(26, 21), s(30, 18), s(33, 27), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(4, 2), s(7, 12), s(14, 15), s(22, 19), s(19, 29), s(31, 29), s(35, 36), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(4, 5), s(4, 15), s(8, 7), s(7, 23), s(6, 34), s(4, 33), s(3, 41), s(0, 51), s(-5, 65), s(4, 60), s(-29, 95), s(-17, 90), s(-42, 150), s(5, 95), 
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(95, 31);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(91, 58);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(150, -7);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(101, 4);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(42, 40);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(88, 18);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(63, -28);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(25, 39);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(76, 31);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(84, 53);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(114, 0);

pub const TEMPO_BONUS: ScoreTuple = s(41, 29);

pub const ENEMY_VIRT_MOBILITY: [ScoreTuple; 28] = [
  s(-10, -23), s(-8, 0), s(0, 15), s(2, 11), s(6, 11), s(10, 6), s(6, 22), s(13, 9), s(18, 7), s(20, 8), s(27, 2), s(32, 0), s(28, 0), s(34, -4), s(45, -6), s(41, -7), s(41, -7), s(46, -11), s(47, -14), s(41, -15), s(71, -21), s(55, -17), s(36, -19), s(21, -18), s(13, -15), s(-62, -12), s(27, -16), s(-79, -5), 
];

pub const ATTACKS: [ScoreTuple; (NUM_PIECES - 1) as usize] = [
  s(-10, -23), s(-8, 0), s(0, 15), s(2, 11), s(6, 11),
];

pub const DEFENSES: [ScoreTuple; (NUM_PIECES - 1) as usize] = [
  s(-10, -23), s(-8, 0), s(0, 15), s(2, 11), s(6, 11),
];

pub const INNER_PAWN_SHIELD: ScoreTuple = s(41, 29);

pub const OUTER_PAWN_SHIELD: ScoreTuple = s(41, 29);

pub const BIAS: ScoreTuple = s(41, 29);