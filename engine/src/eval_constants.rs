#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{evaluation::ScoreTuple, board_representation::NUM_PIECES, pst::{Pst, Rst}};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(204, 272), s(324, 288), s(295, 345), s(347, 308), s(449, 297), s(262, 318), s(322, 276), s(264, 230), 
  s(328, 319), s(354, 341), s(483, 305), s(419, 338), s(419, 321), s(474, 300), s(404, 316), s(383, 290), 
  s(347, 321), s(457, 323), s(442, 361), s(463, 347), s(462, 336), s(511, 332), s(445, 315), s(397, 301), 
  s(406, 330), s(415, 353), s(421, 378), s(467, 370), s(429, 371), s(474, 358), s(415, 347), s(433, 313), 
  s(392, 339), s(406, 338), s(417, 374), s(421, 375), s(429, 369), s(421, 368), s(441, 340), s(402, 333), 
  s(378, 319), s(391, 345), s(410, 345), s(412, 361), s(430, 360), s(419, 334), s(420, 314), s(387, 327), 
  s(376, 297), s(377, 320), s(395, 331), s(418, 335), s(415, 339), s(416, 323), s(403, 304), s(397, 305), 
  s(293, 305), s(394, 304), s(376, 319), s(389, 334), s(401, 327), s(388, 326), s(394, 318), s(387, 272), 
]),
// Bishop PST
Pst::new([
  s(385, 363), s(364, 356), s(283, 373), s(229, 380), s(302, 366), s(283, 370), s(378, 360), s(358, 353), 
  s(380, 366), s(422, 365), s(372, 365), s(366, 359), s(419, 352), s(418, 359), s(402, 363), s(341, 360), 
  s(384, 371), s(444, 359), s(452, 364), s(439, 356), s(439, 354), s(461, 363), s(421, 366), s(413, 361), 
  s(402, 362), s(424, 370), s(422, 383), s(469, 376), s(439, 379), s(447, 370), s(423, 359), s(395, 370), 
  s(414, 357), s(416, 369), s(420, 376), s(443, 384), s(443, 373), s(416, 371), s(406, 364), s(411, 355), 
  s(419, 352), s(437, 361), s(430, 375), s(433, 371), s(432, 379), s(440, 367), s(426, 364), s(420, 356), 
  s(438, 337), s(439, 343), s(440, 347), s(423, 361), s(433, 360), s(444, 355), s(457, 345), s(428, 337), 
  s(406, 342), s(439, 355), s(421, 360), s(408, 362), s(429, 357), s(411, 370), s(420, 355), s(408, 344), 
]),
// Rook PST
Pst::new([
  s(606, 702), s(631, 694), s(566, 720), s(615, 704), s(602, 708), s(557, 716), s(642, 690), s(628, 694), 
  s(585, 706), s(582, 712), s(618, 705), s(636, 698), s(652, 682), s(627, 696), s(582, 708), s(599, 700), 
  s(559, 704), s(598, 702), s(590, 705), s(614, 698), s(598, 694), s(610, 689), s(642, 684), s(580, 690), 
  s(538, 708), s(585, 692), s(577, 708), s(586, 698), s(578, 698), s(566, 705), s(571, 694), s(551, 700), 
  s(530, 703), s(547, 702), s(558, 705), s(568, 700), s(587, 685), s(547, 696), s(576, 687), s(541, 691), 
  s(537, 690), s(560, 691), s(567, 685), s(572, 683), s(571, 689), s(574, 678), s(582, 679), s(544, 676), 
  s(536, 687), s(566, 680), s(574, 680), s(581, 686), s(586, 677), s(575, 677), s(577, 673), s(506, 689), 
  s(570, 680), s(580, 682), s(594, 681), s(600, 679), s(596, 677), s(578, 678), s(550, 688), s(562, 663), 
]),
// Queen PST
Pst::new([
  s(1113, 1183), s(1111, 1212), s(1088, 1240), s(1114, 1231), s(1223, 1174), s(1154, 1213), s(1227, 1155), s(1205, 1196), 
  s(1138, 1155), s(1099, 1200), s(1132, 1219), s(1116, 1248), s(1062, 1303), s(1164, 1208), s(1154, 1202), s(1223, 1167), 
  s(1156, 1152), s(1145, 1171), s(1158, 1174), s(1133, 1240), s(1178, 1220), s(1208, 1198), s(1197, 1166), s(1183, 1201), 
  s(1123, 1200), s(1142, 1193), s(1124, 1200), s(1121, 1238), s(1136, 1232), s(1155, 1216), s(1149, 1237), s(1155, 1209), 
  s(1161, 1152), s(1122, 1208), s(1149, 1189), s(1134, 1235), s(1143, 1211), s(1143, 1195), s(1162, 1196), s(1151, 1207), 
  s(1144, 1165), s(1171, 1135), s(1164, 1173), s(1166, 1164), s(1160, 1194), s(1167, 1166), s(1173, 1167), s(1163, 1184), 
  s(1151, 1137), s(1168, 1139), s(1184, 1125), s(1189, 1121), s(1190, 1134), s(1192, 1118), s(1173, 1101), s(1186, 1100), 
  s(1176, 1126), s(1168, 1124), s(1180, 1124), s(1192, 1119), s(1183, 1130), s(1146, 1129), s(1165, 1113), s(1151, 1095), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(160, 285), s(171, 277), s(138, 277), s(172, 251), s(140, 266), s(162, 245), s(105, 281), s(67, 301), 
  s(88, 164), s(102, 153), s(123, 152), s(129, 137), s(181, 124), s(195, 133), s(167, 157), s(130, 155), 
  s(75, 145), s(90, 133), s(98, 133), s(121, 108), s(120, 127), s(114, 128), s(100, 133), s(84, 134), 
  s(66, 126), s(72, 124), s(91, 122), s(116, 114), s(106, 120), s(106, 118), s(82, 115), s(75, 114), 
  s(77, 119), s(83, 119), s(98, 122), s(106, 128), s(110, 137), s(110, 130), s(119, 108), s(97, 108), 
  s(70, 128), s(87, 118), s(83, 136), s(99, 130), s(95, 149), s(126, 130), s(126, 108), s(88, 105), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-109, -106), s(39, -63), s(76, -39), s(-44, -12), s(-121, 12), s(-138, 40), s(48, 4), s(32, -41), 
  s(52, -30), s(-3, 36), s(-64, 40), s(72, 18), s(-10, 40), s(-52, 70), s(11, 41), s(-54, 25), 
  s(-53, 19), s(32, 32), s(94, 25), s(13, 34), s(52, 40), s(121, 57), s(159, 46), s(-12, 17), 
  s(28, -11), s(1, 37), s(47, 36), s(-14, 50), s(-32, 53), s(-8, 50), s(-6, 44), s(-80, 14), 
  s(-130, 4), s(12, 3), s(-1, 32), s(-63, 51), s(-72, 54), s(-23, 32), s(-46, 19), s(-94, -1), 
  s(30, -31), s(6, -1), s(0, 16), s(-18, 27), s(2, 25), s(9, 11), s(14, 0), s(-12, -17), 
  s(51, -49), s(44, -19), s(3, 6), s(-38, 19), s(-27, 21), s(-3, 9), s(34, -11), s(24, -34), 
  s(8, -89), s(52, -55), s(30, -29), s(-86, 2), s(-8, -24), s(-46, 0), s(27, -36), s(12, -72), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(60, 185), s(71, 177), s(38, 177), s(72, 151), s(40, 166), s(62, 145), s(5, 181), s(-32, 201), 
  s(53, 208), s(22, 217), s(19, 177), s(13, 154), s(-30, 155), s(0, 162), s(-59, 182), s(-58, 205), 
  s(30, 119), s(13, 109), s(18, 86), s(0, 85), s(1, 65), s(42, 67), s(-21, 113), s(-10, 108), 
  s(11, 69), s(-5, 64), s(-26, 48), s(-19, 44), s(-35, 46), s(-31, 50), s(0, 69), s(15, 60), 
  s(10, 26), s(0, 34), s(-27, 25), s(-44, 34), s(-32, 15), s(12, 10), s(-20, 42), s(23, 24), 
  s(-4, 19), s(12, 25), s(-1, 12), s(-42, 28), s(-21, 15), s(-15, 14), s(-9, 31), s(-8, 26), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_RST: Rst = Rst::new([
  s(43, -225),
  s(19, -143),
  s(-2, -57),
  s(6, -33),
  s(-7, 0),
  s(-9, -9),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_RST: Rst = Rst::new([
  s(0, 0),
  s(27, -19),
  s(8, -23),
  s(-5, -25),
  s(-17, -14),
  s(-33, -18),
  s(-17, -18),
  s(0, 0),
]);

pub const PHALANX_PAWNS_RST: Rst = Rst::new([
  s(0, 0),
  s(78, 331),
  s(202, 135),
  s(52, 53),
  s(20, 17),
  s(-5, -6),
  s(5, -7),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(30, 74);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(20, 54), s(31, 73), s(35, 82), s(46, 84), s(51, 92), s(59, 89), s(60, 88), s(71, 71), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(16, 27), s(29, 46), s(38, 62), s(45, 73), s(49, 83), s(52, 89), s(53, 90), s(48, 97), s(56, 92), s(67, 86), s(89, 84), s(23, 125), s(84, 77), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(11, 52), s(17, 66), s(26, 72), s(25, 84), s(34, 95), s(39, 102), s(46, 104), s(59, 107), s(61, 115), s(65, 120), s(76, 123), s(79, 130), s(83, 127), s(73, 129), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(9, 245), s(12, 245), s(12, 266), s(19, 273), s(22, 289), s(23, 301), s(22, 327), s(24, 338), s(27, 343), s(28, 356), s(30, 369), s(28, 373), s(33, 374), s(33, 384), s(30, 397), s(31, 407), s(41, 392), s(51, 396), s(64, 389), s(39, 412), s(110, 369), s(57, 398), s(165, 346), s(183, 314), s(259, 280), s(227, 328), s(195, 303), 
];

pub const KING_ZONE_ATTACKS: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight attack values
[
  s(-8, -10), s(-4, 3), s(3, 19), s(5, 12), s(9, 13), s(14, 8), s(9, 24), s(16, 11), s(21, 9), s(23, 9), s(31, 3), s(35, 3), s(30, 2), s(37, -2), s(47, -4), s(45, -5), s(42, -4), s(52, -9), s(45, -11), s(44, -12), s(76, -19), s(58, -14), s(43, -17), s(22, -15), s(15, -12), s(-57, -9), s(27, -11), s(-94, 0), 
],
// Bishop attack values
[
  s(5, 14), s(-1, 12), s(2, 16), s(7, 11), s(10, 17), s(17, 5), s(17, 2), s(19, 7), s(31, -2), s(24, 0), s(38, -7), s(31, -4), s(40, -5), s(39, -7), s(43, -5), s(35, -8), s(37, -6), s(40, -9), s(43, -8), s(44, -9), s(26, -6), s(29, -7), s(0, -2), s(-14, 0), s(-34, 1), s(-28, -5), s(23, -20), s(-35, -4), 
],
// Rook attack values
[
  s(17, 8), s(4, 0), s(16, -10), s(23, -22), s(22, -16), s(26, -18), s(25, -9), s(19, -8), s(32, -14), s(33, -15), s(39, -16), s(36, -15), s(33, -12), s(32, -11), s(43, -14), s(38, -11), s(40, -11), s(54, -15), s(46, -11), s(44, -10), s(30, -7), s(35, -7), s(42, -10), s(39, -7), s(31, -2), s(63, -12), s(10, 1), s(43, -6), 
],
// Queen attack values
[
  s(0, -10), s(0, -5), s(4, -16), s(11, -20), s(15, -14), s(13, -5), s(14, -7), s(17, 2), s(24, -10), s(29, -3), s(24, 0), s(33, -1), s(41, -6), s(37, -2), s(48, -9), s(66, -19), s(66, -18), s(81, -30), s(89, -32), s(96, -36), s(121, -49), s(128, -54), s(125, -43), s(150, -63), s(167, -64), s(207, -91), s(119, -34), s(62, -29), 
],
// Pawn attack values
[
  s(7, 21), s(8, -2), s(8, -10), s(9, -7), s(9, -8), s(7, -9), s(10, -9), s(10, -8), s(8, -8), s(10, -8), s(13, -11), s(12, -11), s(20, -13), s(20, -12), s(18, -13), s(23, -14), s(30, -16), s(30, -16), s(24, -16), s(28, -17), s(41, -19), s(65, -24), s(47, -19), s(31, -19), s(32, -16), s(51, -14), s(57, -11), s(28, -6), 
],
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(94, 36);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(92, 63);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(145, -7);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(104, 7);

pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(42, 43);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(89, 18);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(64, -17);

pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(25, 38);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(71, 37);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(84, 54);

pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(96, 5);
