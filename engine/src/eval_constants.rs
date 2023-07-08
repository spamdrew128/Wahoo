#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{evaluation::ScoreTuple, board_representation::NUM_PIECES, piece_tables::{Pst, Prt}};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(252, 290), s(355, 320), s(341, 382), s(398, 346), s(492, 335), s(307, 350), s(365, 302), s(315, 243), 
  s(340, 331), s(380, 359), s(445, 335), s(425, 352), s(419, 334), s(460, 320), s(397, 340), s(389, 300), 
  s(367, 328), s(450, 338), s(458, 370), s(465, 359), s(462, 352), s(528, 339), s(465, 321), s(418, 303), 
  s(429, 336), s(449, 360), s(448, 389), s(497, 380), s(465, 381), s(502, 368), s(454, 355), s(468, 314), 
  s(417, 347), s(443, 349), s(453, 385), s(466, 385), s(467, 381), s(462, 378), s(479, 351), s(432, 340), 
  s(412, 329), s(419, 359), s(441, 361), s(445, 374), s(461, 376), s(449, 350), s(455, 330), s(415, 337), 
  s(395, 298), s(397, 322), s(414, 337), s(436, 343), s(432, 345), s(435, 331), s(426, 306), s(415, 306), 
  s(310, 291), s(392, 296), s(382, 314), s(394, 330), s(405, 323), s(393, 322), s(392, 308), s(405, 262), 
]),
// Bishop PST
Pst::new([
  s(421, 397), s(406, 391), s(340, 405), s(275, 414), s(339, 402), s(321, 403), s(403, 393), s(406, 385), 
  s(418, 388), s(435, 393), s(409, 385), s(386, 381), s(427, 378), s(431, 385), s(408, 393), s(363, 384), 
  s(442, 382), s(472, 377), s(462, 386), s(462, 373), s(467, 370), s(504, 377), s(476, 383), s(478, 370), 
  s(432, 379), s(456, 385), s(453, 398), s(502, 395), s(467, 400), s(485, 386), s(450, 381), s(440, 386), 
  s(439, 373), s(448, 384), s(460, 393), s(475, 401), s(482, 389), s(446, 392), s(444, 378), s(433, 371), 
  s(443, 368), s(462, 379), s(462, 392), s(462, 390), s(460, 398), s(471, 384), s(450, 382), s(445, 368), 
  s(457, 350), s(462, 355), s(463, 362), s(447, 376), s(455, 376), s(466, 371), s(479, 359), s(445, 349), 
  s(424, 352), s(449, 368), s(429, 369), s(423, 374), s(439, 369), s(419, 383), s(435, 364), s(434, 350), 
]),
// Rook PST
Pst::new([
  s(659, 748), s(683, 739), s(622, 764), s(675, 746), s(656, 751), s(610, 758), s(684, 734), s(671, 741), 
  s(632, 752), s(636, 755), s(662, 751), s(690, 741), s(693, 727), s(673, 739), s(621, 752), s(640, 745), 
  s(609, 738), s(641, 737), s(638, 739), s(655, 732), s(657, 725), s(663, 722), s(696, 716), s(632, 721), 
  s(580, 739), s(626, 725), s(616, 742), s(635, 729), s(627, 728), s(620, 736), s(620, 724), s(595, 730), 
  s(573, 730), s(588, 730), s(604, 730), s(618, 726), s(639, 712), s(594, 724), s(621, 712), s(582, 718), 
  s(578, 709), s(595, 716), s(608, 708), s(616, 704), s(610, 713), s(614, 704), s(612, 706), s(581, 699), 
  s(563, 712), s(599, 702), s(604, 705), s(614, 707), s(618, 700), s(605, 704), s(608, 697), s(527, 715), 
  s(589, 704), s(600, 706), s(612, 704), s(617, 700), s(616, 697), s(595, 706), s(569, 714), s(579, 688), 
]),
// Queen PST
Pst::new([
  s(1174, 1189), s(1175, 1221), s(1146, 1261), s(1179, 1241), s(1271, 1198), s(1207, 1236), s(1287, 1165), s(1239, 1225), 
  s(1198, 1151), s(1168, 1201), s(1190, 1232), s(1177, 1256), s(1113, 1321), s(1215, 1216), s(1221, 1198), s(1280, 1160), 
  s(1216, 1138), s(1204, 1163), s(1209, 1171), s(1196, 1223), s(1235, 1203), s(1272, 1184), s(1270, 1144), s(1246, 1178), 
  s(1182, 1180), s(1203, 1172), s(1184, 1182), s(1186, 1215), s(1198, 1213), s(1224, 1188), s(1214, 1210), s(1216, 1181), 
  s(1213, 1128), s(1183, 1182), s(1207, 1170), s(1193, 1216), s(1208, 1185), s(1198, 1179), s(1224, 1167), s(1207, 1174), 
  s(1201, 1127), s(1221, 1112), s(1217, 1150), s(1216, 1145), s(1214, 1171), s(1219, 1144), s(1226, 1138), s(1217, 1145), 
  s(1205, 1098), s(1221, 1103), s(1230, 1099), s(1234, 1096), s(1236, 1108), s(1241, 1094), s(1223, 1073), s(1236, 1063), 
  s(1222, 1085), s(1209, 1088), s(1220, 1087), s(1227, 1082), s(1222, 1093), s(1186, 1099), s(1211, 1075), s(1199, 1058), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(147, 302), s(142, 299), s(130, 293), s(158, 267), s(126, 284), s(124, 268), s(79, 301), s(53, 319), 
  s(97, 165), s(100, 160), s(132, 154), s(133, 140), s(186, 134), s(198, 137), s(183, 158), s(139, 158), 
  s(81, 148), s(96, 136), s(102, 137), s(122, 115), s(123, 131), s(119, 131), s(104, 136), s(91, 136), 
  s(77, 127), s(74, 128), s(98, 123), s(123, 115), s(114, 121), s(114, 120), s(85, 119), s(86, 115), 
  s(84, 121), s(91, 121), s(103, 123), s(110, 131), s(117, 138), s(117, 131), s(127, 110), s(105, 110), 
  s(76, 129), s(90, 119), s(85, 138), s(100, 133), s(97, 153), s(132, 132), s(132, 111), s(93, 108), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-97, -104), s(71, -64), s(118, -49), s(-39, -12), s(-114, 12), s(-124, 34), s(72, 0), s(43, -49), 
  s(64, -26), s(7, 37), s(-70, 44), s(81, 18), s(-1, 43), s(-63, 78), s(16, 47), s(-49, 24), 
  s(-47, 23), s(27, 39), s(94, 29), s(11, 36), s(70, 40), s(117, 63), s(161, 53), s(1, 18), 
  s(19, -7), s(1, 42), s(31, 40), s(-15, 51), s(-44, 58), s(-22, 57), s(-6, 50), s(-94, 19), 
  s(-132, 6), s(18, 5), s(0, 33), s(-62, 52), s(-68, 55), s(-21, 34), s(-43, 22), s(-89, -1), 
  s(27, -31), s(19, -2), s(8, 15), s(-16, 28), s(4, 25), s(15, 11), s(20, 0), s(-8, -18), 
  s(54, -49), s(47, -19), s(8, 5), s(-41, 21), s(-27, 21), s(0, 8), s(39, -13), s(30, -38), 
  s(16, -93), s(54, -57), s(23, -30), s(-92, 0), s(-14, -27), s(-51, -1), s(25, -39), s(20, -78), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(47, 202), s(42, 199), s(30, 193), s(58, 167), s(26, 184), s(24, 168), s(-20, 201), s(-46, 219), 
  s(46, 211), s(26, 214), s(10, 180), s(15, 152), s(-32, 149), s(-7, 162), s(-69, 187), s(-64, 206), 
  s(26, 118), s(7, 109), s(16, 86), s(0, 81), s(0, 66), s(35, 68), s(-25, 113), s(-17, 109), 
  s(6, 68), s(-6, 63), s(-30, 48), s(-22, 45), s(-39, 47), s(-36, 49), s(0, 67), s(6, 60), 
  s(7, 25), s(-5, 32), s(-29, 25), s(-42, 31), s(-32, 15), s(10, 10), s(-22, 41), s(18, 24), 
  s(-2, 18), s(10, 24), s(-3, 11), s(-38, 24), s(-25, 13), s(-18, 14), s(-16, 30), s(-11, 24), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(63, -262),
  s(23, -150),
  s(1, -62),
  s(12, -38),
  s(-3, -3),
  s(-5, -13),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(51, -29),
  s(9, -24),
  s(-4, -26),
  s(-18, -14),
  s(-33, -18),
  s(-17, -18),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(80, 370),
  s(197, 138),
  s(48, 51),
  s(18, 16),
  s(-6, -7),
  s(6, -8),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(30, 69);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(4, 51), s(5, 59), s(3, 61), s(9, 57), s(9, 60), s(11, 55), s(7, 52), s(17, 34), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(10, 16), s(18, 33), s(21, 44), s(23, 54), s(23, 60), s(24, 65), s(22, 66), s(15, 72), s(19, 65), s(28, 59), s(51, 55), s(-15, 94), s(45, 44), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(7, 44), s(11, 57), s(18, 60), s(16, 71), s(21, 84), s(22, 91), s(25, 91), s(34, 93), s(33, 97), s(32, 101), s(39, 103), s(41, 106), s(45, 102), s(34, 101), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(4, 345), s(7, 309), s(6, 328), s(14, 326), s(17, 337), s(19, 341), s(19, 361), s(22, 372), s(27, 373), s(29, 381), s(31, 393), s(31, 391), s(35, 393), s(38, 395), s(35, 407), s(39, 412), s(50, 394), s(57, 396), s(76, 384), s(56, 399), s(125, 361), s(68, 381), s(174, 325), s(188, 293), s(281, 252), s(202, 307), s(182, 290), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(20, 22), s(31, 33), s(41, 37), s(51, 39), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(6, 11), s(14, 15), s(20, 19), s(25, 20), s(25, 21), s(29, 18), s(32, 28), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(4, 2), s(7, 11), s(14, 15), s(22, 19), s(18, 30), s(31, 28), s(35, 36), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(4, 5), s(4, 16), s(8, 7), s(6, 24), s(6, 35), s(3, 33), s(2, 41), s(0, 50), s(-4, 65), s(5, 59), s(-28, 95), s(-16, 89), s(-38, 146), s(7, 94), 
];

pub const KING_ZONE_ATTACKS: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight attack values
[
  s(-9, -22), s(-8, 0), s(0, 16), s(2, 12), s(7, 11), s(11, 7), s(6, 23), s(14, 9), s(19, 7), s(20, 7), s(27, 2), s(32, 0), s(28, 0), s(35, -4), s(45, -6), s(41, -7), s(40, -6), s(45, -10), s(47, -14), s(40, -15), s(69, -21), s(55, -17), s(34, -19), s(22, -18), s(12, -15), s(-61, -12), s(24, -15), s(-80, -5), 
],
// Bishop attack values
[
  s(2, 14), s(-2, 13), s(0, 17), s(4, 13), s(7, 17), s(15, 5), s(15, 2), s(16, 8), s(28, -2), s(22, 0), s(36, -7), s(29, -5), s(36, -4), s(37, -8), s(40, -5), s(32, -8), s(34, -7), s(38, -10), s(41, -8), s(43, -10), s(20, -5), s(25, -7), s(-7, -1), s(-16, -1), s(-37, 1), s(-32, -5), s(20, -21), s(-41, -3), 
],
// Rook attack values
[
  s(16, 4), s(2, -2), s(14, -10), s(21, -22), s(20, -16), s(25, -19), s(24, -11), s(17, -9), s(31, -15), s(32, -16), s(38, -18), s(34, -16), s(32, -14), s(30, -12), s(41, -15), s(36, -12), s(37, -12), s(50, -15), s(44, -13), s(40, -11), s(29, -8), s(30, -7), s(40, -10), s(37, -8), s(26, -2), s(67, -14), s(17, -1), s(33, -6), 
],
// Queen attack values
[
  s(-1, -13), s(0, -10), s(5, -18), s(10, -21), s(14, -14), s(12, -5), s(14, -8), s(16, 2), s(23, -11), s(28, -4), s(23, 0), s(30, -1), s(39, -6), s(37, -4), s(46, -10), s(62, -18), s(63, -19), s(83, -33), s(86, -32), s(94, -36), s(122, -52), s(119, -52), s(119, -44), s(145, -62), s(155, -59), s(197, -88), s(154, -55), s(88, -45), 
],
// Pawn attack values
[
  s(7, 12), s(8, -3), s(8, -10), s(9, -7), s(8, -8), s(6, -9), s(9, -8), s(10, -8), s(7, -8), s(10, -9), s(12, -11), s(12, -11), s(20, -13), s(20, -12), s(17, -13), s(23, -14), s(29, -17), s(28, -16), s(22, -15), s(27, -17), s(37, -19), s(63, -24), s(47, -19), s(30, -19), s(33, -17), s(45, -13), s(46, -9), s(18, -4), 
],
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(96, 34);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(92, 58);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(150, -5);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(102, 5);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(42, 41);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(86, 20);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(65, -29);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(26, 39);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(75, 33);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(86, 54);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(116, 2);

pub const CHECK_BONUS: [ScoreTuple; (NUM_PIECES - 1) as usize] = [
  s(180, 29), s(160, 94), s(256, 13), s(115, 162), s(439, -3), 
];

pub const TEMPO_BONUS: ScoreTuple = s(42, 30);
