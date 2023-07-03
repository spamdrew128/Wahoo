#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{evaluation::ScoreTuple, board_representation::NUM_PIECES, pst::{Pst, Rst}};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(200, 274), s(299, 286), s(291, 335), s(364, 298), s(480, 287), s(277, 309), s(339, 268), s(275, 222), 
  s(319, 317), s(346, 335), s(477, 293), s(433, 327), s(424, 317), s(483, 292), s(413, 308), s(398, 285), 
  s(331, 316), s(442, 312), s(427, 350), s(462, 342), s(486, 328), s(521, 322), s(464, 304), s(432, 292), 
  s(390, 325), s(402, 341), s(404, 366), s(456, 362), s(426, 366), s(469, 352), s(412, 345), s(434, 313), 
  s(379, 331), s(390, 326), s(407, 359), s(414, 363), s(424, 360), s(414, 359), s(442, 329), s(395, 329), 
  s(365, 310), s(378, 329), s(401, 326), s(402, 348), s(425, 347), s(412, 321), s(416, 309), s(376, 321), 
  s(364, 287), s(367, 311), s(382, 315), s(411, 318), s(410, 323), s(414, 307), s(401, 297), s(390, 297), 
  s(268, 301), s(383, 293), s(364, 308), s(384, 322), s(392, 317), s(385, 315), s(387, 305), s(375, 271), 
]),
// Bishop PST
Pst::new([
  s(372, 345), s(359, 338), s(269, 358), s(244, 362), s(311, 346), s(301, 345), s(365, 339), s(352, 335), 
  s(371, 347), s(405, 345), s(363, 346), s(353, 339), s(432, 333), s(423, 336), s(425, 339), s(350, 341), 
  s(365, 352), s(428, 337), s(429, 342), s(428, 338), s(445, 334), s(468, 340), s(441, 344), s(420, 345), 
  s(383, 342), s(406, 348), s(413, 360), s(456, 356), s(434, 360), s(441, 350), s(412, 342), s(390, 354), 
  s(396, 338), s(405, 349), s(410, 355), s(436, 360), s(437, 351), s(406, 350), s(405, 342), s(405, 338), 
  s(409, 333), s(424, 341), s(422, 352), s(422, 350), s(426, 358), s(433, 345), s(415, 345), s(413, 339), 
  s(427, 322), s(429, 322), s(429, 327), s(416, 338), s(425, 337), s(439, 331), s(449, 324), s(417, 323), 
  s(395, 327), s(429, 339), s(412, 342), s(401, 343), s(419, 340), s(405, 347), s(413, 335), s(400, 326), 
]),
// Rook PST
Pst::new([
  s(573, 694), s(590, 688), s(532, 712), s(579, 697), s(571, 701), s(552, 703), s(587, 688), s(614, 681), 
  s(549, 699), s(553, 704), s(588, 697), s(614, 688), s(628, 673), s(633, 678), s(572, 695), s(583, 688), 
  s(515, 702), s(564, 697), s(560, 698), s(581, 693), s(573, 686), s(624, 670), s(638, 671), s(583, 674), 
  s(503, 703), s(553, 686), s(544, 702), s(559, 690), s(564, 686), s(571, 689), s(568, 680), s(546, 686), 
  s(496, 699), s(513, 696), s(529, 698), s(535, 694), s(562, 677), s(538, 683), s(566, 675), s(523, 681), 
  s(501, 685), s(521, 687), s(532, 679), s(536, 679), s(542, 683), s(556, 668), s(569, 667), s(525, 666), 
  s(502, 682), s(530, 674), s(541, 675), s(548, 682), s(557, 671), s(564, 667), s(572, 660), s(487, 681), 
  s(539, 672), s(548, 676), s(564, 675), s(575, 672), s(577, 667), s(566, 670), s(537, 676), s(544, 650), 
]),
// Queen PST
Pst::new([
  s(1113, 1156), s(1092, 1197), s(1084, 1229), s(1112, 1211), s(1234, 1147), s(1182, 1182), s(1233, 1133), s(1205, 1178), 
  s(1130, 1136), s(1084, 1182), s(1131, 1192), s(1104, 1232), s(1072, 1288), s(1197, 1178), s(1169, 1185), s(1219, 1162), 
  s(1144, 1133), s(1128, 1154), s(1143, 1155), s(1127, 1226), s(1189, 1202), s(1239, 1185), s(1233, 1153), s(1214, 1198), 
  s(1116, 1170), s(1130, 1167), s(1118, 1176), s(1121, 1209), s(1140, 1220), s(1159, 1224), s(1152, 1249), s(1157, 1227), 
  s(1147, 1131), s(1114, 1186), s(1140, 1162), s(1130, 1208), s(1139, 1193), s(1139, 1202), s(1159, 1204), s(1150, 1219), 
  s(1135, 1144), s(1159, 1112), s(1153, 1146), s(1157, 1140), s(1155, 1169), s(1165, 1161), s(1166, 1168), s(1159, 1178), 
  s(1143, 1110), s(1156, 1117), s(1175, 1101), s(1183, 1095), s(1186, 1107), s(1190, 1108), s(1174, 1087), s(1185, 1091), 
  s(1169, 1098), s(1161, 1105), s(1177, 1097), s(1188, 1097), s(1179, 1108), s(1147, 1122), s(1163, 1102), s(1146, 1080), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(158, 279), s(170, 276), s(133, 273), s(163, 248), s(133, 264), s(151, 242), s(100, 279), s(60, 297), 
  s(95, 168), s(119, 165), s(123, 149), s(132, 143), s(197, 116), s(199, 127), s(192, 161), s(137, 148), 
  s(79, 152), s(113, 144), s(104, 129), s(123, 105), s(132, 118), s(123, 124), s(133, 137), s(88, 133), 
  s(72, 140), s(100, 143), s(104, 123), s(127, 112), s(129, 115), s(120, 119), s(125, 128), s(81, 120), 
  s(76, 129), s(103, 134), s(103, 119), s(108, 126), s(125, 130), s(118, 125), s(152, 120), s(98, 110), 
  s(79, 135), s(116, 132), s(94, 132), s(111, 130), s(118, 142), s(148, 128), s(170, 121), s(98, 105), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-23, -130), s(132, -85), s(167, -63), s(29, -31), s(-99, 0), s(-97, 31), s(139, -17), s(56, -48), 
  s(129, -53), s(40, 18), s(-6, 19), s(99, 5), s(23, 27), s(-26, 58), s(34, 30), s(-4, 9), 
  s(2, -1), s(52, 20), s(100, 14), s(5, 29), s(23, 37), s(112, 52), s(143, 41), s(7, 9), 
  s(6, -16), s(-36, 37), s(-19, 42), s(-76, 56), s(-111, 64), s(-80, 64), s(-49, 51), s(-98, 17), 
  s(-154, 4), s(-24, 5), s(-87, 47), s(-165, 70), s(-172, 75), s(-117, 57), s(-97, 34), s(-127, 8), 
  s(12, -28), s(-27, 5), s(-84, 35), s(-130, 52), s(-103, 52), s(-96, 43), s(-38, 19), s(-51, -2), 
  s(60, -51), s(24, -15), s(-32, 15), s(-76, 27), s(-69, 30), s(-43, 19), s(21, -7), s(32, -34), 
  s(34, -98), s(80, -63), s(53, -39), s(-89, -1), s(6, -34), s(-34, -10), s(61, -51), s(53, -88), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(58, 179), s(70, 176), s(33, 173), s(63, 148), s(33, 164), s(51, 142), s(0, 179), s(-39, 197), 
  s(46, 188), s(6, 195), s(14, 169), s(9, 135), s(-40, 149), s(-9, 154), s(-67, 169), s(-57, 192), 
  s(17, 94), s(-4, 83), s(15, 72), s(3, 70), s(-2, 54), s(42, 51), s(-49, 95), s(-19, 89), 
  s(-8, 44), s(-36, 40), s(-38, 35), s(-30, 34), s(-41, 34), s(-22, 30), s(-24, 46), s(10, 39), 
  s(-13, 3), s(-26, 10), s(-36, 12), s(-52, 18), s(-24, 0), s(18, -9), s(-14, 12), s(24, 1), 
  s(-23, -1), s(-4, 0), s(-2, -1), s(-42, 14), s(-20, 1), s(2, -5), s(19, -3), s(-18, 7), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_RST: Rst = Rst::new([
  s(66, -237),
  s(31, -152),
  s(-3, -62),
  s(6, -39),
  s(-7, -6),
  s(-15, -14),
  s(0, 0),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(27, 73);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(20, 48), s(31, 72), s(35, 83), s(49, 87), s(55, 98), s(64, 95), s(69, 93), s(90, 72), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(15, 30), s(29, 50), s(39, 69), s(48, 82), s(53, 94), s(59, 102), s(62, 103), s(60, 111), s(71, 105), s(86, 100), s(115, 94), s(43, 138), s(137, 82), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(13, 43), s(19, 58), s(29, 63), s(29, 75), s(40, 87), s(45, 94), s(53, 96), s(69, 98), s(74, 106), s(79, 111), s(89, 115), s(96, 121), s(104, 117), s(89, 118), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(2, 205), s(5, 209), s(6, 218), s(14, 227), s(19, 244), s(21, 257), s(21, 285), s(25, 295), s(31, 300), s(33, 315), s(37, 329), s(37, 335), s(42, 338), s(45, 349), s(45, 363), s(48, 375), s(58, 365), s(67, 372), s(92, 363), s(62, 394), s(133, 353), s(94, 381), s(193, 339), s(228, 302), s(305, 266), s(256, 330), s(241, 307), 
];

