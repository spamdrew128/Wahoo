#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{evaluation::ScoreTuple, board_representation::NUM_PIECES, piece_tables::{Pst, Prt}};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(252, 292), s(366, 320), s(343, 385), s(396, 350), s(506, 334), s(306, 351), s(374, 303), s(314, 245), 
  s(351, 329), s(386, 359), s(443, 335), s(441, 350), s(420, 335), s(467, 318), s(430, 334), s(394, 300), 
  s(373, 328), s(452, 340), s(463, 370), s(471, 359), s(467, 351), s(527, 339), s(468, 321), s(422, 303), 
  s(434, 337), s(453, 360), s(452, 388), s(501, 381), s(469, 381), s(505, 368), s(458, 356), s(473, 315), 
  s(421, 347), s(448, 349), s(457, 386), s(471, 385), s(471, 381), s(466, 378), s(485, 351), s(435, 339), 
  s(416, 329), s(423, 360), s(445, 361), s(449, 374), s(465, 376), s(453, 349), s(459, 331), s(419, 336), 
  s(399, 298), s(401, 322), s(418, 338), s(440, 343), s(436, 345), s(439, 331), s(430, 307), s(418, 306), 
  s(314, 292), s(395, 296), s(386, 314), s(397, 331), s(409, 323), s(397, 322), s(395, 309), s(407, 264), 
]),
// Bishop PST
Pst::new([
  s(434, 395), s(410, 393), s(326, 410), s(286, 414), s(347, 401), s(330, 404), s(410, 395), s(410, 386), 
  s(421, 389), s(439, 394), s(417, 386), s(385, 383), s(435, 377), s(417, 390), s(412, 395), s(362, 387), 
  s(444, 384), s(477, 378), s(463, 388), s(465, 374), s(471, 370), s(508, 377), s(480, 384), s(481, 371), 
  s(436, 380), s(460, 386), s(456, 399), s(505, 395), s(470, 401), s(487, 389), s(452, 383), s(445, 387), 
  s(441, 374), s(453, 385), s(464, 394), s(478, 402), s(486, 389), s(449, 393), s(448, 379), s(436, 372), 
  s(446, 369), s(465, 380), s(464, 393), s(465, 391), s(463, 399), s(475, 384), s(452, 383), s(449, 368), 
  s(461, 351), s(465, 356), s(466, 363), s(450, 377), s(458, 378), s(469, 372), s(482, 360), s(448, 351), 
  s(425, 355), s(452, 368), s(432, 371), s(426, 375), s(443, 371), s(422, 385), s(439, 364), s(437, 350), 
]),
// Rook PST
Pst::new([
  s(661, 752), s(694, 740), s(633, 764), s(682, 749), s(657, 754), s(619, 760), s(692, 737), s(680, 743), 
  s(637, 754), s(640, 758), s(668, 753), s(698, 742), s(701, 729), s(679, 742), s(615, 757), s(640, 749), 
  s(615, 740), s(647, 740), s(641, 742), s(660, 735), s(664, 728), s(665, 726), s(708, 718), s(637, 725), 
  s(584, 743), s(630, 729), s(623, 745), s(641, 732), s(632, 731), s(624, 739), s(625, 727), s(597, 734), 
  s(578, 733), s(594, 733), s(609, 734), s(624, 728), s(644, 714), s(599, 727), s(627, 715), s(587, 722), 
  s(582, 712), s(600, 719), s(612, 710), s(620, 707), s(614, 716), s(618, 707), s(616, 708), s(585, 701), 
  s(567, 716), s(603, 705), s(608, 708), s(618, 710), s(623, 703), s(609, 708), s(613, 700), s(531, 718), 
  s(593, 708), s(604, 709), s(616, 707), s(621, 704), s(620, 701), s(599, 710), s(573, 718), s(583, 691), 
]),
// Queen PST
Pst::new([
  s(1197, 1184), s(1200, 1210), s(1164, 1253), s(1203, 1233), s(1295, 1184), s(1234, 1223), s(1308, 1155), s(1248, 1224), 
  s(1215, 1155), s(1185, 1206), s(1207, 1235), s(1197, 1257), s(1127, 1329), s(1228, 1222), s(1225, 1205), s(1289, 1169), 
  s(1233, 1143), s(1220, 1169), s(1224, 1177), s(1214, 1229), s(1249, 1212), s(1286, 1193), s(1288, 1150), s(1261, 1190), 
  s(1199, 1186), s(1223, 1173), s(1199, 1187), s(1204, 1216), s(1221, 1210), s(1241, 1196), s(1232, 1212), s(1233, 1187), 
  s(1230, 1133), s(1199, 1189), s(1224, 1176), s(1210, 1220), s(1226, 1189), s(1215, 1185), s(1241, 1174), s(1224, 1184), 
  s(1218, 1132), s(1237, 1118), s(1233, 1156), s(1233, 1150), s(1231, 1176), s(1236, 1149), s(1243, 1145), s(1233, 1153), 
  s(1222, 1102), s(1238, 1108), s(1247, 1104), s(1251, 1102), s(1253, 1112), s(1257, 1100), s(1239, 1080), s(1252, 1068), 
  s(1239, 1088), s(1225, 1096), s(1237, 1092), s(1243, 1090), s(1238, 1098), s(1204, 1105), s(1228, 1081), s(1214, 1069), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(153, 300), s(142, 299), s(135, 291), s(165, 266), s(128, 283), s(123, 268), s(84, 300), s(60, 317), 
  s(98, 166), s(102, 159), s(134, 153), s(135, 138), s(191, 134), s(197, 137), s(182, 157), s(141, 158), 
  s(82, 148), s(98, 136), s(103, 137), s(123, 115), s(124, 131), s(120, 131), s(106, 136), s(92, 136), 
  s(77, 127), s(75, 128), s(99, 123), s(124, 114), s(115, 121), s(114, 120), s(86, 119), s(87, 115), 
  s(84, 121), s(91, 121), s(104, 123), s(111, 131), s(118, 138), s(117, 131), s(129, 111), s(106, 110), 
  s(76, 130), s(91, 119), s(86, 138), s(100, 133), s(97, 153), s(132, 133), s(133, 111), s(93, 108), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-98, -107), s(79, -64), s(119, -49), s(-34, -10), s(-116, 13), s(-129, 36), s(67, 0), s(37, -49), 
  s(69, -25), s(11, 37), s(-68, 44), s(78, 19), s(0, 44), s(-60, 78), s(21, 47), s(-53, 25), 
  s(-43, 23), s(30, 38), s(95, 29), s(14, 36), s(71, 40), s(115, 63), s(166, 52), s(-1, 18), 
  s(29, -8), s(3, 42), s(34, 39), s(-17, 52), s(-45, 59), s(-16, 56), s(-4, 49), s(-96, 18), 
  s(-128, 6), s(20, 5), s(0, 32), s(-56, 50), s(-68, 55), s(-22, 33), s(-44, 23), s(-90, -1), 
  s(30, -30), s(18, -3), s(7, 15), s(-19, 28), s(5, 25), s(14, 10), s(20, 0), s(-9, -18), 
  s(57, -50), s(49, -19), s(6, 5), s(-41, 20), s(-26, 21), s(0, 8), s(40, -13), s(31, -38), 
  s(17, -93), s(55, -58), s(24, -31), s(-94, 0), s(-14, -27), s(-50, -2), s(26, -39), s(21, -78), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(53, 200), s(42, 199), s(35, 191), s(65, 166), s(28, 183), s(23, 168), s(-15, 200), s(-39, 217), 
  s(47, 211), s(29, 216), s(11, 181), s(13, 154), s(-35, 150), s(-4, 162), s(-64, 188), s(-64, 207), 
  s(28, 118), s(7, 109), s(17, 86), s(0, 82), s(0, 66), s(34, 68), s(-27, 113), s(-18, 110), 
  s(6, 69), s(-5, 62), s(-28, 47), s(-23, 46), s(-39, 47), s(-37, 50), s(4, 67), s(5, 60), 
  s(6, 25), s(-6, 32), s(-27, 25), s(-42, 31), s(-31, 15), s(9, 10), s(-21, 40), s(17, 25), 
  s(-2, 18), s(11, 24), s(-6, 11), s(-37, 24), s(-23, 13), s(-19, 14), s(-16, 31), s(-11, 25), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(54, -258),
  s(21, -150),
  s(1, -63),
  s(12, -38),
  s(-4, -3),
  s(-8, -11),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(45, -27),
  s(9, -24),
  s(-4, -26),
  s(-18, -14),
  s(-34, -18),
  s(-17, -19),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(75, 383),
  s(206, 137),
  s(48, 52),
  s(18, 16),
  s(-6, -7),
  s(6, -8),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(30, 70);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(3, 50), s(5, 58), s(2, 61), s(8, 57), s(8, 60), s(10, 55), s(6, 52), s(15, 34), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(10, 16), s(18, 32), s(20, 44), s(23, 53), s(23, 60), s(24, 65), s(22, 65), s(14, 72), s(19, 65), s(29, 58), s(53, 55), s(-13, 92), s(44, 45), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(6, 42), s(10, 55), s(17, 58), s(15, 69), s(20, 82), s(21, 89), s(24, 89), s(33, 90), s(32, 95), s(31, 98), s(39, 100), s(42, 104), s(46, 100), s(33, 99), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(2, 345), s(5, 311), s(4, 327), s(11, 330), s(14, 339), s(16, 343), s(17, 364), s(19, 374), s(24, 377), s(26, 385), s(28, 397), s(29, 394), s(32, 398), s(36, 398), s(32, 412), s(36, 415), s(48, 399), s(53, 402), s(75, 388), s(56, 405), s(134, 361), s(64, 385), s(173, 329), s(181, 302), s(292, 251), s(211, 313), s(180, 296), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(19, 23), s(31, 33), s(41, 37), s(52, 40), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(6, 11), s(14, 15), s(21, 19), s(25, 20), s(26, 21), s(30, 18), s(32, 27), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(5, 2), s(7, 12), s(14, 15), s(22, 19), s(19, 30), s(31, 28), s(37, 35), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(5, -3), s(6, 7), s(10, -1), s(8, 13), s(8, 25), s(6, 22), s(5, 30), s(1, 38), s(-2, 54), s(8, 48), s(-26, 82), s(-16, 78), s(-36, 136), s(8, 82), 
];

pub const KING_ZONE_ATTACKS: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight attack values
[
  s(-9, -23), s(-8, 0), s(0, 16), s(2, 12), s(6, 12), s(11, 7), s(6, 23), s(13, 9), s(19, 7), s(20, 8), s(28, 1), s(32, 1), s(29, 0), s(35, -3), s(45, -6), s(44, -8), s(41, -6), s(49, -11), s(49, -15), s(36, -14), s(69, -20), s(51, -16), s(32, -18), s(20, -18), s(14, -15), s(-62, -12), s(29, -16), s(-81, -6), 
],
// Bishop attack values
[
  s(2, 17), s(-2, 13), s(-1, 17), s(4, 13), s(6, 18), s(15, 5), s(14, 3), s(17, 8), s(28, -2), s(22, 0), s(35, -7), s(29, -4), s(36, -4), s(37, -8), s(41, -5), s(31, -8), s(36, -7), s(36, -10), s(42, -8), s(42, -10), s(18, -5), s(26, -7), s(-6, -1), s(-16, -1), s(-38, 1), s(-39, -4), s(23, -21), s(-36, -4), 
],
// Rook attack values
[
  s(15, 4), s(2, -2), s(14, -10), s(21, -21), s(20, -16), s(24, -18), s(24, -12), s(18, -10), s(31, -16), s(31, -16), s(38, -18), s(34, -16), s(32, -14), s(29, -12), s(42, -15), s(36, -12), s(38, -12), s(52, -16), s(45, -13), s(43, -12), s(32, -9), s(30, -7), s(38, -10), s(40, -9), s(32, -4), s(74, -16), s(16, 0), s(28, -5), 
],
// Queen attack values
[
  s(-2, -12), s(0, -10), s(4, -19), s(10, -22), s(13, -14), s(12, -5), s(14, -8), s(16, 1), s(23, -11), s(29, -6), s(24, 0), s(31, -2), s(38, -6), s(37, -5), s(46, -11), s(63, -20), s(62, -18), s(82, -33), s(87, -33), s(99, -40), s(130, -56), s(123, -55), s(118, -43), s(146, -63), s(162, -62), s(201, -90), s(146, -52), s(94, -48), 
],
// Pawn attack values
[
  s(7, 13), s(7, -3), s(8, -10), s(8, -7), s(8, -8), s(6, -9), s(9, -8), s(10, -8), s(7, -8), s(10, -9), s(11, -11), s(12, -11), s(20, -14), s(20, -13), s(17, -13), s(23, -14), s(28, -17), s(28, -16), s(21, -15), s(27, -17), s(38, -20), s(64, -24), s(48, -20), s(32, -19), s(32, -17), s(43, -13), s(50, -9), s(17, -4), 
],
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(96, 32);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(92, 58);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(147, -5);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(102, 3);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(43, 39);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(89, 13);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(63, -34);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(25, 38);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(75, 29);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(86, 48);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(115, 2);

pub const TEMPO_BONUS: ScoreTuple = s(42, 30);
