#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{eval::{evaluation::ScoreTuple, piece_tables::{Pst, Prt}}, board::board_representation::NUM_PIECES};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [[Pst; NUM_PIECES as usize]; 2] = [
[
// Knight PST
Pst::new([
  s(220, 255), s(222, 327), s(219, 352), s(344, 345), s(296, 358), s(260, 358), s(183, 348), s(168, 290), 
  s(287, 303), s(322, 324), s(348, 317), s(346, 328), s(344, 332), s(342, 329), s(316, 341), s(300, 315), 
  s(328, 307), s(338, 328), s(397, 333), s(355, 352), s(370, 348), s(375, 347), s(355, 319), s(316, 308), 
  s(360, 316), s(350, 349), s(389, 361), s(377, 367), s(394, 368), s(367, 365), s(350, 340), s(342, 322), 
  s(344, 334), s(366, 339), s(371, 359), s(373, 370), s(372, 368), s(358, 367), s(350, 338), s(332, 332), 
  s(331, 325), s(353, 329), s(359, 335), s(368, 356), s(351, 358), s(347, 337), s(332, 330), s(321, 310), 
  s(329, 308), s(328, 309), s(343, 321), s(347, 326), s(343, 321), s(324, 322), s(318, 303), s(306, 289), 
  s(311, 283), s(304, 314), s(324, 300), s(321, 310), s(316, 305), s(301, 295), s(302, 313), s(258, 276), 
]),
// Bishop PST
Pst::new([
  s(331, 365), s(298, 390), s(278, 382), s(258, 389), s(248, 391), s(244, 390), s(324, 376), s(340, 385), 
  s(305, 357), s(308, 380), s(346, 370), s(332, 360), s(316, 363), s(363, 356), s(369, 359), s(348, 353), 
  s(363, 365), s(367, 367), s(382, 370), s(359, 367), s(380, 347), s(381, 364), s(384, 356), s(368, 359), 
  s(357, 357), s(365, 360), s(392, 368), s(388, 377), s(409, 376), s(379, 366), s(371, 366), s(352, 357), 
  s(371, 336), s(356, 364), s(372, 369), s(394, 374), s(395, 373), s(374, 373), s(370, 358), s(358, 347), 
  s(376, 343), s(384, 350), s(386, 362), s(383, 371), s(380, 369), s(387, 364), s(383, 359), s(364, 340), 
  s(380, 333), s(401, 340), s(395, 344), s(376, 352), s(372, 350), s(388, 338), s(380, 332), s(385, 336), 
  s(376, 323), s(377, 337), s(354, 355), s(364, 342), s(353, 342), s(359, 347), s(384, 348), s(357, 327), 
]),
// Rook PST
Pst::new([
  s(589, 628), s(531, 652), s(564, 649), s(602, 629), s(572, 645), s(557, 650), s(600, 631), s(563, 645), 
  s(564, 628), s(562, 635), s(578, 640), s(556, 638), s(578, 636), s(556, 648), s(547, 642), s(543, 641), 
  s(549, 619), s(604, 619), s(566, 622), s(567, 626), s(545, 632), s(542, 633), s(552, 631), s(528, 632), 
  s(504, 628), s(525, 627), s(526, 635), s(541, 628), s(535, 627), s(530, 636), s(535, 624), s(505, 634), 
  s(492, 620), s(531, 618), s(499, 631), s(531, 621), s(525, 622), s(511, 627), s(501, 625), s(493, 621), 
  s(505, 599), s(534, 603), s(521, 606), s(524, 608), s(514, 610), s(514, 603), s(506, 607), s(494, 606), 
  s(468, 605), s(546, 584), s(517, 601), s(525, 598), s(522, 597), s(517, 601), s(503, 601), s(484, 600), 
  s(489, 589), s(485, 609), s(510, 600), s(520, 596), s(517, 594), s(509, 599), s(505, 597), s(501, 603), 
]),
// Queen PST
Pst::new([
  s(978, 1204), s(1045, 1157), s(993, 1233), s(981, 1246), s(995, 1233), s(969, 1250), s(965, 1222), s(924, 1248), 
  s(1010, 1066), s(962, 1098), s(965, 1136), s(921, 1170), s(938, 1146), s(964, 1098), s(946, 1099), s(959, 1083), 
  s(981, 1060), s(989, 1057), s(974, 1097), s(971, 1105), s(950, 1119), s(971, 1079), s(955, 1063), s(970, 1055), 
  s(966, 1063), s(966, 1083), s(966, 1081), s(959, 1093), s(955, 1103), s(960, 1071), s(950, 1075), s(955, 1064), 
  s(965, 1044), s(966, 1060), s(953, 1073), s(957, 1105), s(949, 1103), s(950, 1088), s(950, 1067), s(952, 1042), 
  s(963, 1030), s(973, 1044), s(971, 1051), s(951, 1090), s(954, 1080), s(958, 1071), s(962, 1047), s(955, 1035), 
  s(954, 987), s(966, 997), s(974, 999), s(969, 1036), s(969, 1035), s(967, 1025), s(957, 1033), s(960, 1018), 
  s(946, 987), s(946, 967), s(936, 1011), s(950, 1015), s(960, 996), s(950, 1008), s(944, 1022), s(943, 1025), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(104, 261), s(158, 233), s(180, 239), s(176, 217), s(146, 218), s(148, 205), s(112, 237), s(130, 232), 
  s(120, 162), s(143, 163), s(157, 144), s(124, 132), s(118, 128), s(126, 137), s(92, 149), s(86, 154), 
  s(85, 135), s(91, 125), s(108, 119), s(108, 112), s(99, 112), s(86, 122), s(75, 127), s(65, 136), 
  s(85, 110), s(82, 106), s(104, 104), s(101, 107), s(99, 106), s(83, 114), s(66, 114), s(61, 116), 
  s(93, 104), s(107, 96), s(104, 110), s(96, 121), s(93, 121), s(83, 116), s(74, 113), s(61, 116), 
  s(88, 103), s(118, 97), s(121, 115), s(83, 138), s(83, 130), s(71, 129), s(70, 119), s(61, 126), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(7, 8), s(-68, 93), s(-49, 99), s(-76, 111), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(-87, 148), s(-42, 179), s(-95, 204), s(-11, 191), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(-94, 95), s(1, 115), s(3, 129), s(-69, 146), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(-104, 23), s(-52, 50), s(-42, 53), s(-84, 64), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(-128, -10), s(-55, 2), s(-31, 10), s(-60, 26), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(-62, -33), s(-9, -23), s(-12, -14), s(-16, -2), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(11, -63), s(17, -42), s(-13, -29), s(-30, -19), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(35, -106), s(37, -70), s(7, -52), s(7, -54), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]),
],
[
// Knight PST
Pst::new([
  s(168, 290), s(183, 348), s(260, 358), s(296, 358), s(344, 345), s(219, 352), s(222, 327), s(220, 255), 
  s(300, 315), s(316, 341), s(342, 329), s(344, 332), s(346, 328), s(348, 317), s(322, 324), s(287, 303), 
  s(316, 308), s(355, 319), s(375, 347), s(370, 348), s(355, 352), s(397, 333), s(338, 328), s(328, 307), 
  s(342, 322), s(350, 340), s(367, 365), s(394, 368), s(377, 367), s(389, 361), s(350, 349), s(360, 316), 
  s(332, 332), s(350, 338), s(358, 367), s(372, 368), s(373, 370), s(371, 359), s(366, 339), s(344, 334), 
  s(321, 310), s(332, 330), s(347, 337), s(351, 358), s(368, 356), s(359, 335), s(353, 329), s(331, 325), 
  s(306, 289), s(318, 303), s(324, 322), s(343, 321), s(347, 326), s(343, 321), s(328, 309), s(329, 308), 
  s(258, 276), s(302, 313), s(301, 295), s(316, 305), s(321, 310), s(324, 300), s(304, 314), s(311, 283), 
]),
// Bishop PST
Pst::new([
  s(340, 385), s(324, 376), s(244, 390), s(248, 391), s(258, 389), s(278, 382), s(298, 390), s(331, 365), 
  s(348, 353), s(369, 359), s(363, 356), s(316, 363), s(332, 360), s(346, 370), s(308, 380), s(305, 357), 
  s(368, 359), s(384, 356), s(381, 364), s(380, 347), s(359, 367), s(382, 370), s(367, 367), s(363, 365), 
  s(352, 357), s(371, 366), s(379, 366), s(409, 376), s(388, 377), s(392, 368), s(365, 360), s(357, 357), 
  s(358, 347), s(370, 358), s(374, 373), s(395, 373), s(394, 374), s(372, 369), s(356, 364), s(371, 336), 
  s(364, 340), s(383, 359), s(387, 364), s(380, 369), s(383, 371), s(386, 362), s(384, 350), s(376, 343), 
  s(385, 336), s(380, 332), s(388, 338), s(372, 350), s(376, 352), s(395, 344), s(401, 340), s(380, 333), 
  s(357, 327), s(384, 348), s(359, 347), s(353, 342), s(364, 342), s(354, 355), s(377, 337), s(376, 323), 
]),
// Rook PST
Pst::new([
  s(563, 645), s(600, 631), s(557, 650), s(572, 645), s(602, 629), s(564, 649), s(531, 652), s(589, 628), 
  s(543, 641), s(547, 642), s(556, 648), s(578, 636), s(556, 638), s(578, 640), s(562, 635), s(564, 628), 
  s(528, 632), s(552, 631), s(542, 633), s(545, 632), s(567, 626), s(566, 622), s(604, 619), s(549, 619), 
  s(505, 634), s(535, 624), s(530, 636), s(535, 627), s(541, 628), s(526, 635), s(525, 627), s(504, 628), 
  s(493, 621), s(501, 625), s(511, 627), s(525, 622), s(531, 621), s(499, 631), s(531, 618), s(492, 620), 
  s(494, 606), s(506, 607), s(514, 603), s(514, 610), s(524, 608), s(521, 606), s(534, 603), s(505, 599), 
  s(484, 600), s(503, 601), s(517, 601), s(522, 597), s(525, 598), s(517, 601), s(546, 584), s(468, 605), 
  s(501, 603), s(505, 597), s(509, 599), s(517, 594), s(520, 596), s(510, 600), s(485, 609), s(489, 589), 
]),
// Queen PST
Pst::new([
  s(924, 1248), s(965, 1222), s(969, 1250), s(995, 1233), s(981, 1246), s(993, 1233), s(1045, 1157), s(978, 1204), 
  s(959, 1083), s(946, 1099), s(964, 1098), s(938, 1146), s(921, 1170), s(965, 1136), s(962, 1098), s(1010, 1066), 
  s(970, 1055), s(955, 1063), s(971, 1079), s(950, 1119), s(971, 1105), s(974, 1097), s(989, 1057), s(981, 1060), 
  s(955, 1064), s(950, 1075), s(960, 1071), s(955, 1103), s(959, 1093), s(966, 1081), s(966, 1083), s(966, 1063), 
  s(952, 1042), s(950, 1067), s(950, 1088), s(949, 1103), s(957, 1105), s(953, 1073), s(966, 1060), s(965, 1044), 
  s(955, 1035), s(962, 1047), s(958, 1071), s(954, 1080), s(951, 1090), s(971, 1051), s(973, 1044), s(963, 1030), 
  s(960, 1018), s(957, 1033), s(967, 1025), s(969, 1035), s(969, 1036), s(974, 999), s(966, 997), s(954, 987), 
  s(943, 1025), s(944, 1022), s(950, 1008), s(960, 996), s(950, 1015), s(936, 1011), s(946, 967), s(946, 987), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(130, 232), s(112, 237), s(148, 205), s(146, 218), s(176, 217), s(180, 239), s(158, 233), s(104, 261), 
  s(86, 154), s(92, 149), s(126, 137), s(118, 128), s(124, 132), s(157, 144), s(143, 163), s(120, 162), 
  s(65, 136), s(75, 127), s(86, 122), s(99, 112), s(108, 112), s(108, 119), s(91, 125), s(85, 135), 
  s(61, 116), s(66, 114), s(83, 114), s(99, 106), s(101, 107), s(104, 104), s(82, 106), s(85, 110), 
  s(61, 116), s(74, 113), s(83, 116), s(93, 121), s(96, 121), s(104, 110), s(107, 96), s(93, 104), 
  s(61, 126), s(70, 119), s(71, 129), s(83, 130), s(83, 138), s(121, 115), s(118, 97), s(88, 103), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(-76, 111), s(-49, 99), s(-68, 93), s(7, 8), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(-11, 191), s(-95, 204), s(-42, 179), s(-87, 148), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(-69, 146), s(3, 129), s(1, 115), s(-94, 95), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(-84, 64), s(-42, 53), s(-52, 50), s(-104, 23), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(-60, 26), s(-31, 10), s(-55, 2), s(-128, -10), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(-16, -2), s(-12, -14), s(-9, -23), s(-62, -33), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(-30, -19), s(-13, -29), s(17, -42), s(11, -63), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(7, -54), s(7, -52), s(37, -70), s(35, -106), 
]),
],
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(84, 138), s(98, 123), s(65, 144), s(104, 106), s(13, 132), s(57, 98), s(-46, 149), s(-51, 164), 
  s(44, 142), s(44, 140), s(26, 121), s(34, 107), s(33, 107), s(31, 102), s(-14, 119), s(-44, 143), 
  s(35, 70), s(29, 68), s(29, 59), s(12, 58), s(8, 57), s(35, 45), s(-6, 72), s(-10, 67), 
  s(21, 34), s(7, 42), s(-9, 38), s(-2, 33), s(-14, 30), s(7, 24), s(16, 39), s(0, 36), 
  s(12, 4), s(-5, 27), s(-23, 18), s(-19, 13), s(-13, 6), s(9, 5), s(1, 26), s(12, 9), 
  s(0, 20), s(-3, 21), s(-14, 5), s(-8, 0), s(0, -3), s(4, 0), s(4, 6), s(-2, 11), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(-46, -178),
  s(0, -110),
  s(-5, -45),
  s(-14, -16),
  s(-11, 6),
  s(-8, 1),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(61, -28),
  s(12, -31),
  s(-1, -27),
  s(-15, -16),
  s(-23, -20),
  s(-12, -20),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(21, 372),
  s(180, 158),
  s(52, 49),
  s(18, 15),
  s(-4, -4),
  s(2, -8),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(27, 67);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(23, 55), s(27, 71), s(27, 74), s(30, 73), s(30, 78), s(31, 74), s(29, 71), s(28, 62), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(7, 36), s(15, 56), s(18, 67), s(22, 77), s(23, 84), s(25, 88), s(23, 90), s(19, 94), s(21, 90), s(23, 85), s(28, 86), s(-19, 115), s(25, 75), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(1, 111), s(1, 130), s(2, 152), s(1, 161), s(3, 173), s(4, 179), s(5, 186), s(10, 190), s(11, 195), s(13, 197), s(16, 203), s(21, 206), s(32, 200), s(29, 197), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(52, 218), s(50, 231), s(53, 262), s(57, 274), s(58, 282), s(62, 284), s(62, 304), s(64, 309), s(67, 308), s(70, 311), s(70, 322), s(72, 316), s(75, 321), s(74, 325), s(79, 325), s(77, 330), s(86, 325), s(90, 324), s(111, 306), s(116, 307), s(166, 273), s(148, 272), s(176, 253), s(224, 237), s(278, 195), s(199, 234), s(203, 198), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(11, 27), s(19, 38), s(28, 45), s(35, 49), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(2, 15), s(8, 20), s(12, 23), s(15, 28), s(15, 30), s(18, 29), s(21, 37), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(7, 4), s(11, 9), s(19, 13), s(24, 20), s(28, 25), s(35, 26), s(39, 32), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(-13, 138), s(-12, 149), s(-12, 154), s(-11, 167), s(-11, 180), s(-12, 182), s(-12, 189), s(-11, 196), s(-13, 207), s(-11, 203), s(-15, 211), s(3, 196), s(-20, 233), s(21, 193), 
];

pub const PAWN_THREAT_ON_KNIGHT: [ScoreTuple; 2] = [s(98, 87), s(76, 30)];
pub const PAWN_THREAT_ON_BISHOP: [ScoreTuple; 2] = [s(94, 68), s(74, 58)];
pub const PAWN_THREAT_ON_ROOK: [ScoreTuple; 2] = [s(165, 134), s(109, 3)];
pub const PAWN_THREAT_ON_QUEEN: [ScoreTuple; 2] = [s(235, 109), s(98, -33)];
pub const KNIGHT_THREAT_ON_BISHOP: [ScoreTuple; 2] = [s(36, 31), s(39, 38)];
pub const KNIGHT_THREAT_ON_ROOK: [ScoreTuple; 2] = [s(63, 70), s(83, 14)];
pub const KNIGHT_THREAT_ON_QUEEN: [ScoreTuple; 2] = [s(113, 232), s(64, -40)];
pub const BISHOP_THREAT_ON_KNIGHT: [ScoreTuple; 2] = [s(18, 36), s(26, 31)];
pub const BISHOP_THREAT_ON_ROOK: [ScoreTuple; 2] = [s(84, 93), s(66, 32)];
pub const BISHOP_THREAT_ON_QUEEN: [ScoreTuple; 2] = [s(122, 217), s(90, 23)];
pub const ROOK_THREAT_ON_QUEEN: [ScoreTuple; 2] = [s(116, 398), s(84, 7)];

pub const PASSER_SQ_RULE_BONUS: ScoreTuple = s(-26, 81);

pub const TEMPO_BONUS: ScoreTuple = s(34, 23);

// KING SAFETY FEATURES
pub const ATTACKS: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight attacks
[
  s(-4, 10), s(5, -7), s(9, -4), s(12, -6), s(11, -5), s(11, -6), s(10, -4), s(11, -6), s(12, -6), s(11, -3), s(9, -2), s(10, -4), s(8, -1), s(11, -3), s(8, -4), s(7, -3), s(6, -2), s(4, 0), s(4, -5), s(4, -6), s(7, -9), s(3, -9), s(3, -10), s(3, -14), s(0, -14), s(2, -20), s(0, -16), s(-32, -13), 
],
// Bishop attacks
[
  s(3, 7), s(7, -4), s(11, -5), s(10, -4), s(10, -2), s(11, -4), s(9, -4), s(10, -5), s(12, -6), s(10, -5), s(10, -6), s(9, -5), s(9, -6), s(8, -5), s(9, -7), s(6, -6), s(9, -7), s(6, -6), s(6, -7), s(3, -5), s(3, -4), s(4, -10), s(5, -7), s(3, -10), s(-1, -8), s(6, -16), s(5, -13), s(-5, -16), 
],
// Rook attacks
[
  s(5, 2), s(3, -2), s(5, -5), s(6, -7), s(6, -7), s(5, -5), s(5, -7), s(4, -4), s(6, -7), s(5, -6), s(5, -6), s(5, -7), s(3, -4), s(3, -4), s(3, -3), s(3, -2), s(2, -1), s(1, -1), s(1, 0), s(1, 0), s(0, 0), s(1, 0), s(0, 0), s(0, 0), s(0, 0), s(-7, 1), s(-10, 3), s(3, -3), 
],
// Queen attacks
[
  s(-1, 4), s(1, -4), s(3, -7), s(5, -8), s(4, -6), s(3, -3), s(4, -5), s(5, -3), s(5, -6), s(6, -3), s(4, 0), s(6, 0), s(5, 0), s(7, -1), s(6, 0), s(8, -3), s(8, -2), s(10, -4), s(9, -2), s(14, -5), s(14, -5), s(8, -2), s(25, -11), s(14, -11), s(11, -1), s(13, -5), s(-15, 11), s(10, -15), 
],
// Pawn attacks
[
  s(1, 6), s(6, -2), s(4, -5), s(2, -4), s(1, -4), s(2, -4), s(0, -2), s(1, -4), s(0, -3), s(0, -2), s(0, -3), s(1, -3), s(0, -2), s(1, -3), s(0, -2), s(1, -3), s(2, -3), s(2, -3), s(4, -4), s(2, -3), s(5, -6), s(5, -6), s(7, -7), s(9, -8), s(11, -7), s(4, -5), s(28, -14), s(18, -12), 
],
];

pub const DEFENSES: [[ScoreTuple; 28]; (NUM_PIECES - 1) as usize] = [
// Knight defenses
[
  s(-5, 4), s(-1, 3), s(-4, 3), s(-3, 2), s(-4, 3), s(-3, 3), s(-3, 3), s(-3, 1), s(-3, 2), s(-4, 2), s(-4, 2), s(-3, 0), s(-5, 3), s(-5, 3), s(-3, 2), s(-5, 4), s(-5, 4), s(-2, 2), s(-2, 5), s(-2, 4), s(-5, 6), s(-2, 7), s(-5, 8), s(-3, 11), s(-8, 12), s(-8, 18), s(-4, 12), s(-4, 23), 
],
// Bishop defenses
[
  s(0, 0), s(-2, 0), s(-4, 2), s(-5, 2), s(-3, 2), s(-3, 2), s(-5, 3), s(-3, 2), s(-4, 2), s(-4, 3), s(-3, 1), s(-3, 1), s(-4, 3), s(-4, 2), s(-3, 2), s(-2, 0), s(-3, 1), s(-1, 0), s(-3, 2), s(0, 0), s(0, 0), s(-2, 2), s(-5, 3), s(0, 2), s(1, 3), s(-2, 5), s(0, 1), s(5, 10), 
],
// Rook defenses
[
  s(-22, 12), s(-4, 4), s(-5, 6), s(-5, 5), s(-4, 4), s(-5, 6), s(-3, 4), s(-4, 4), s(-4, 5), s(-4, 4), s(-3, 3), s(-3, 4), s(-2, 2), s(-3, 5), s(-2, 3), s(-2, 2), s(0, 1), s(-1, 2), s(-2, 3), s(-1, 3), s(-4, 4), s(-1, 3), s(-2, 4), s(-3, 4), s(-5, 5), s(-6, 4), s(4, 1), s(-5, 4), 
],
// Queen defenses
[
  s(2, -8), s(-2, 2), s(-2, 1), s(-2, 1), s(-3, 2), s(-2, 2), s(-3, 4), s(-3, 5), s(-4, 6), s(-4, 6), s(-3, 4), s(-3, 4), s(-2, 2), s(-2, 3), s(0, 0), s(-1, 3), s(0, 0), s(0, 2), s(2, 0), s(1, 0), s(3, 0), s(7, -1), s(-1, 7), s(0, 5), s(14, 0), s(12, 0), s(5, 5), s(2, 3), 
],
// Pawn defenses
[
  s(-10, -4), s(-16, 3), s(-12, 1), s(-10, 2), s(-8, 0), s(-8, 0), s(-8, 0), s(-6, 0), s(-4, -1), s(-4, -2), s(-2, -2), s(-3, -2), s(0, -5), s(0, -5), s(0, -6), s(1, -7), s(0, -6), s(1, -7), s(2, -10), s(2, -9), s(5, -12), s(7, -14), s(10, -15), s(11, -17), s(14, -18), s(43, -27), s(10, -20), s(14, -22), 
],
];

pub const ENEMY_KING_RANK: Prt = Prt::new([
  s(89, 31),
  s(65, 46),
  s(54, 58),
  s(50, 65),
  s(45, 76),
  s(36, 105),
  s(43, 122),
  s(53, 100),
]);

pub const TROPHISM_BONUS: [ScoreTuple; 160] = [
  s(-169, -8), s(65, -30), s(74, -24), s(82, -30), s(85, -26), s(85, -30), s(84, -27), s(87, -28), s(87, -36), s(83, -28), s(83, -27), s(77, -27), s(76, -24), s(77, -28), s(75, -25), s(73, -21), s(70, -18), s(71, -24), s(68, -17), s(64, -15), s(67, -17), s(56, -6), s(57, -9), s(56, -8), s(58, -9), s(52, -3), s(54, -5), s(52, -4), s(46, 3), s(47, 0), s(46, 1), s(43, 1), s(41, 3), s(43, 4), s(41, 3), s(32, 12), s(34, 13), s(34, 10), s(34, 11), s(28, 18), s(29, 14), s(29, 18), s(25, 19), s(24, 21), s(24, 19), s(22, 22), s(17, 26), s(18, 23), s(19, 26), s(16, 27), s(14, 29), s(14, 28), s(13, 30), s(11, 32), s(7, 35), s(7, 39), s(6, 37), s(0, 44), s(1, 43), s(0, 43), s(-3, 45), s(0, 45), s(-1, 46), s(-7, 50), s(-5, 50), s(-6, 50), s(-10, 55), s(-11, 54), s(-14, 58), s(-10, 55), s(-15, 58), s(-19, 64), s(-18, 58), s(-16, 58), s(-26, 69), s(-26, 67), s(-23, 68), s(-26, 69), s(-25, 70), s(-29, 72), s(-30, 75), s(-30, 76), s(-35, 80), s(-38, 82), s(-33, 80), s(-38, 79), s(-35, 85), s(-41, 83), s(-39, 78), s(-42, 87), s(-45, 92), s(-48, 95), s(-48, 95), s(-47, 85), s(-46, 85), s(-49, 104), s(-52, 96), s(-51, 100), s(-57, 104), s(-51, 101), s(-57, 110), s(-57, 107), s(-48, 100), s(-54, 110), s(-56, 113), s(-64, 99), s(-55, 112), s(-53, 112), s(-59, 121), s(-63, 113), s(-60, 106), s(-52, 112), s(-56, 107), s(-55, 111), s(-52, 67), s(-55, 149), s(-74, 128), s(-74, 119), s(-66, 68), s(-55, 172), s(-47, 163), s(-63, 189), s(-68, 146), s(-81, 90), s(-53, 177), s(-45, 157), s(-48, -84), s(-96, 208), s(-40, 83), s(-128, 144), s(-92, 184), s(-69, 184), s(-21, 63), s(-136, 35), s(-104, -20), s(-122, -84), s(-74, 16), s(173, 104), s(-31, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), 
];

pub const PAWN_STORM_BONUS: [ScoreTuple; 160] = [
  s(49, 37), s(28, -5), s(43, 3), s(40, 9), s(40, 18), s(41, 11), s(32, 27), s(34, 18), s(36, 19), s(28, 28), s(27, 31), s(34, 19), s(24, 31), s(32, 25), s(31, 24), s(18, 34), s(24, 29), s(34, 17), s(11, 34), s(20, 34), s(24, 21), s(17, 33), s(12, 35), s(27, 25), s(11, 33), s(15, 30), s(21, 33), s(15, 28), s(7, 37), s(23, 24), s(20, 31), s(5, 33), s(12, 35), s(13, 30), s(3, 37), s(9, 39), s(21, 25), s(-8, 33), s(-3, 35), s(11, 23), s(5, 31), s(10, 28), s(-6, 23), s(-8, 22), s(-18, 31), s(50, 43), s(-164, 49), s(4, 14), s(78, 98), s(77, -92), s(-5, -135), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), 
];

pub const FILE_STRUCTURE: [ScoreTuple; 193] = [
  s(13, 52), s(31, 38), s(37, 40), s(21, 37), s(17, 42), s(31, 28), s(29, 0), s(1, 1), s(20, 31), s(22, -22), s(1, 1), s(1, 1), s(10, -11), s(1, 1), s(1, 1), s(1, 1), s(25, 44), s(33, 29), s(22, 47), s(1, 1), s(30, 29), s(31, 26), s(1, 1), s(1, 1), s(27, 19), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(29, 33), s(34, 25), s(1, 1), s(1, 1), s(33, 17), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(30, 24), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(16, 51), s(31, 41), s(34, 53), s(1, 1), s(18, 42), s(35, 34), s(1, 1), s(1, 1), s(28, 26), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(25, 37), s(33, 22), s(1, 1), s(1, 1), s(26, 29), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(31, 11), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(10, 57), s(27, 48), s(1, 1), s(1, 1), s(20, 45), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(19, 39), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(1, 1), s(8, 67), 
];

pub const STM_QUEEN_CONTACT_CHECKS: ScoreTuple = s(51, 90);

pub const NON_STM_QUEEN_CONTACT_CHECKS: ScoreTuple = s(34, -36);
