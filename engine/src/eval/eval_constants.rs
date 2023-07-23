#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{eval::{evaluation::ScoreTuple, piece_tables::{Pst, Prt}}, board::board_representation::NUM_PIECES};

const fn s(mg: i32, eg: i32) -> ScoreTuple { ScoreTuple::new(mg, eg) }

pub const MATERIAL_PSTS: [Pst; NUM_PIECES as usize] = [
// Knight PST
Pst::new([
  s(205, 274), s(269, 311), s(263, 366), s(327, 332), s(400, 330), s(259, 329), s(300, 291), s(264, 233), 
  s(250, 309), s(288, 334), s(374, 300), s(336, 330), s(350, 310), s(369, 300), s(344, 309), s(313, 280), 
  s(263, 306), s(345, 311), s(354, 340), s(372, 334), s(378, 328), s(441, 313), s(380, 300), s(356, 279), 
  s(329, 311), s(351, 327), s(347, 356), s(399, 350), s(374, 354), s(403, 343), s(360, 332), s(373, 296), 
  s(321, 318), s(342, 321), s(355, 351), s(372, 353), s(373, 351), s(367, 349), s(389, 324), s(340, 319), 
  s(316, 304), s(323, 329), s(348, 325), s(349, 342), s(368, 344), s(355, 320), s(361, 310), s(320, 315), 
  s(299, 277), s(301, 296), s(317, 306), s(347, 306), s(344, 310), s(346, 301), s(335, 284), s(326, 284), 
  s(207, 274), s(298, 279), s(284, 285), s(302, 299), s(315, 290), s(307, 288), s(301, 287), s(308, 247), 
]),
// Bishop PST
Pst::new([
  s(355, 344), s(329, 336), s(240, 356), s(222, 357), s(266, 346), s(262, 348), s(322, 335), s(332, 322), 
  s(336, 334), s(348, 345), s(326, 335), s(309, 324), s(345, 318), s(374, 316), s(328, 327), s(311, 312), 
  s(355, 333), s(384, 329), s(384, 335), s(377, 320), s(390, 305), s(417, 314), s(392, 309), s(398, 302), 
  s(347, 332), s(369, 337), s(374, 346), s(415, 345), s(385, 345), s(403, 327), s(368, 319), s(367, 322), 
  s(354, 322), s(369, 333), s(376, 345), s(393, 351), s(400, 339), s(363, 342), s(371, 318), s(357, 311), 
  s(362, 317), s(380, 327), s(380, 343), s(379, 342), s(381, 346), s(392, 335), s(370, 330), s(367, 314), 
  s(377, 296), s(379, 306), s(381, 311), s(370, 323), s(379, 325), s(392, 318), s(398, 312), s(366, 298), 
  s(343, 297), s(371, 312), s(354, 317), s(348, 321), s(364, 318), s(344, 332), s(360, 311), s(352, 301), 
]),
// Rook PST
Pst::new([
  s(559, 623), s(576, 617), s(530, 637), s(578, 622), s(571, 623), s(547, 625), s(561, 615), s(578, 614), 
  s(536, 618), s(548, 618), s(571, 614), s(592, 606), s(597, 594), s(610, 599), s(560, 612), s(558, 609), 
  s(507, 611), s(541, 609), s(544, 609), s(558, 603), s(567, 595), s(592, 587), s(617, 585), s(566, 585), 
  s(484, 612), s(529, 599), s(522, 613), s(546, 598), s(549, 594), s(554, 598), s(550, 589), s(529, 594), 
  s(476, 603), s(488, 605), s(509, 602), s(521, 598), s(548, 583), s(520, 589), s(547, 579), s(499, 588), 
  s(479, 586), s(495, 592), s(508, 583), s(516, 580), s(516, 585), s(529, 572), s(532, 572), s(498, 569), 
  s(464, 588), s(500, 578), s(504, 581), s(515, 583), s(524, 572), s(527, 573), s(533, 565), s(444, 585), 
  s(491, 581), s(501, 583), s(515, 578), s(524, 573), s(528, 568), s(514, 577), s(488, 581), s(494, 558), 
]),
// Queen PST
Pst::new([
  s(990, 1102), s(1025, 1110), s(1040, 1113), s(1057, 1111), s(1084, 1103), s(1076, 1104), s(1079, 1095), s(1063, 1114), 
  s(959, 997), s(927, 1038), s(965, 1043), s(962, 1058), s(936, 1081), s(1009, 1023), s(1008, 1021), s(1039, 1009), 
  s(972, 986), s(959, 1007), s(981, 1006), s(966, 1052), s(1013, 1035), s(1038, 1020), s(1036, 989), s(1024, 1017), 
  s(942, 1020), s(961, 1016), s(945, 1026), s(957, 1050), s(976, 1044), s(995, 1029), s(986, 1045), s(985, 1030), 
  s(962, 992), s(942, 1033), s(962, 1025), s(956, 1062), s(972, 1037), s(960, 1033), s(987, 1019), s(968, 1037), 
  s(953, 990), s(972, 980), s(969, 1012), s(971, 1010), s(973, 1028), s(979, 1006), s(982, 1010), s(974, 1011), 
  s(957, 965), s(968, 973), s(981, 967), s(990, 961), s(993, 971), s(1000, 962), s(983, 947), s(995, 933), 
  s(966, 962), s(959, 963), s(973, 956), s(981, 953), s(974, 967), s(947, 967), s(960, 956), s(946, 937), 
]),
// Pawn PST
Pst::new([
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
  s(197, 230), s(183, 233), s(171, 226), s(190, 208), s(169, 221), s(188, 206), s(140, 236), s(130, 245), 
  s(73, 172), s(68, 178), s(105, 167), s(109, 160), s(155, 149), s(172, 133), s(154, 163), s(117, 162), 
  s(69, 141), s(83, 127), s(94, 123), s(111, 102), s(120, 112), s(113, 114), s(101, 120), s(84, 125), 
  s(68, 120), s(63, 119), s(90, 110), s(115, 101), s(115, 102), s(108, 103), s(87, 102), s(81, 104), 
  s(75, 115), s(81, 111), s(96, 110), s(101, 117), s(115, 118), s(111, 113), s(123, 95), s(98, 99), 
  s(66, 126), s(80, 112), s(78, 125), s(101, 117), s(100, 131), s(129, 115), s(126, 98), s(87, 100), 
  s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), s(100, 100), 
]),
// King PST
Pst::new([
  s(-51, -111), s(21, -64), s(47, -46), s(-7, -30), s(-34, -14), s(-37, 7), s(28, -1), s(10, -43), 
  s(36, -33), s(10, 21), s(-21, 21), s(58, 12), s(14, 26), s(-17, 54), s(17, 33), s(-18, 5), 
  s(-12, 5), s(30, 24), s(61, 27), s(22, 28), s(41, 37), s(79, 59), s(94, 51), s(18, 7), 
  s(16, -9), s(-3, 36), s(-1, 46), s(-30, 57), s(-59, 63), s(-37, 60), s(-20, 48), s(-81, 15), 
  s(-96, -1), s(-12, 9), s(-42, 46), s(-99, 66), s(-109, 71), s(-83, 52), s(-82, 30), s(-107, 3), 
  s(15, -27), s(-7, 2), s(-56, 33), s(-89, 49), s(-70, 49), s(-62, 35), s(-19, 11), s(-31, -11), 
  s(62, -48), s(33, -17), s(-20, 13), s(-64, 29), s(-55, 30), s(-26, 16), s(29, -12), s(40, -39), 
  s(37, -94), s(70, -65), s(37, -37), s(-87, 0), s(-2, -30), s(-38, -9), s(51, -53), s(53, -90), 
]),
];

pub const PASSER_PST: Pst = Pst::new([
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
  s(97, 130), s(83, 133), s(71, 126), s(90, 108), s(69, 121), s(88, 106), s(40, 136), s(30, 145), 
  s(97, 146), s(81, 140), s(44, 119), s(41, 92), s(8, 92), s(37, 118), s(9, 125), s(4, 141), 
  s(30, 102), s(10, 96), s(14, 77), s(0, 73), s(-2, 60), s(35, 61), s(-18, 102), s(-14, 97), 
  s(6, 59), s(-6, 56), s(-27, 43), s(-21, 40), s(-35, 41), s(-25, 43), s(2, 62), s(11, 53), 
  s(7, 18), s(-8, 30), s(-26, 21), s(-37, 24), s(-21, 9), s(12, 5), s(-17, 35), s(27, 17), 
  s(3, 7), s(10, 17), s(2, 5), s(-29, 15), s(-18, 6), s(-9, 8), s(2, 19), s(-6, 16), 
  s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), s(0, 0), 
]);

pub const PASSER_BLOCKERS_PRT: Prt = Prt::new([
  s(-58, -172),
  s(0, -124),
  s(0, -58),
  s(9, -37),
  s(-4, -3),
  s(-7, -10),
  s(0, 0),
  s(0, 0),
]);

pub const ISOLATED_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(-8, 50),
  s(-10, -7),
  s(-3, -23),
  s(-16, -13),
  s(-31, -16),
  s(-17, -15),
  s(0, 0),
]);

pub const PHALANX_PAWNS_PRT: Prt = Prt::new([
  s(0, 0),
  s(74, 166),
  s(115, 127),
  s(44, 47),
  s(19, 13),
  s(-4, -8),
  s(8, -10),
  s(0, 0),
]);

pub const BISHOP_PAIR_BONUS: ScoreTuple = s(47, 49);

pub const KNIGHT_MOBILITY: [ScoreTuple; 9] = [
  s(0, 0), s(37, 24), s(38, 31), s(35, 34), s(42, 29), s(41, 34), s(43, 30), s(41, 27), s(55, 6), 
];

pub const BISHOP_MOBILITY: [ScoreTuple; 14] = [
  s(0, 0), s(15, 24), s(24, 37), s(28, 45), s(33, 51), s(35, 54), s(39, 53), s(40, 47), s(36, 48), s(45, 33), s(55, 21), s(80, 8), s(21, 41), s(61, -12), 
];

pub const ROOK_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(9, 76), s(13, 90), s(21, 95), s(19, 106), s(25, 119), s(26, 125), s(29, 127), s(39, 126), s(39, 130), s(39, 133), s(46, 134), s(56, 135), s(58, 131), s(47, 131), 
];

pub const QUEEN_MOBILITY: [ScoreTuple; 28] = [
  s(0, 0), s(51, 87), s(58, 85), s(60, 107), s(69, 114), s(72, 125), s(74, 133), s(74, 156), s(77, 166), s(82, 169), s(84, 176), s(87, 187), s(88, 187), s(92, 191), s(96, 191), s(98, 199), s(102, 205), s(115, 190), s(126, 189), s(146, 179), s(138, 186), s(163, 175), s(141, 166), s(158, 159), s(132, 142), s(130, 146), s(86, 122), s(86, 124), 
];

pub const KNIGHT_FORWARD_MOBILITY: [ScoreTuple; 5] = [
  s(0, 0), s(21, 31), s(33, 43), s(43, 48), s(56, 51), 
];

pub const BISHOP_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(5, 16), s(12, 21), s(17, 25), s(21, 26), s(21, 26), s(25, 18), s(30, 24), 
];

pub const ROOK_FORWARD_MOBILITY: [ScoreTuple; 8] = [
  s(0, 0), s(3, 10), s(6, 17), s(14, 20), s(24, 23), s(23, 32), s(32, 32), s(31, 42), 
];

pub const QUEEN_FORWARD_MOBILITY: [ScoreTuple; 15] = [
  s(0, 0), s(-5, 134), s(-5, 148), s(-3, 151), s(-2, 157), s(-1, 164), s(-2, 163), s(0, 165), s(-1, 168), s(-4, 180), s(13, 164), s(-4, 184), s(22, 162), s(45, 170), s(89, 122), 
];

pub const PAWN_THREAT_ON_KNIGHT: ScoreTuple = s(91, 24);
pub const PAWN_THREAT_ON_BISHOP: ScoreTuple = s(86, 50);
pub const PAWN_THREAT_ON_ROOK: ScoreTuple = s(125, -1);
pub const PAWN_THREAT_ON_QUEEN: ScoreTuple = s(103, -14);
pub const KNIGHT_THREAT_ON_BISHOP: ScoreTuple = s(40, 37);
pub const KNIGHT_THREAT_ON_ROOK: ScoreTuple = s(84, 13);
pub const KNIGHT_THREAT_ON_QUEEN: ScoreTuple = s(66, -46);
pub const BISHOP_THREAT_ON_KNIGHT: ScoreTuple = s(25, 32);
pub const BISHOP_THREAT_ON_ROOK: ScoreTuple = s(74, 25);
pub const BISHOP_THREAT_ON_QUEEN: ScoreTuple = s(83, 34);
pub const ROOK_THREAT_ON_QUEEN: ScoreTuple = s(118, -26);

pub const TEMPO_BONUS: ScoreTuple = s(38, 24);

// KING SAFETY FEATURES
pub const ENEMY_VIRT_MOBILITY: [ScoreTuple; 28] = [
  s(-157, -122), s(-179, -245), s(-80, -170), s(-44, -107), s(-10, 208), s(45, 45), s(63, 138), s(107, 281), s(125, 190), s(139, 192), s(204, 158), s(109, 198), s(170, 184), s(118, 206), s(137, 179), s(180, 183), s(143, 213), s(175, 231), s(174, 234), s(179, 262), s(251, 247), s(266, 268), s(278, 267), s(279, 268), s(267, 277), s(-245, 274), s(227, 301), s(215, 284), 
];

pub const ATTACKS: [ScoreTuple; (NUM_PIECES - 1) as usize] = [
  s(124, -46), s(107, 229), s(30, -1), s(68, 113), s(23, -59), 
];

pub const DEFENSES: [ScoreTuple; (NUM_PIECES - 1) as usize] = [
  s(-15, 43), s(-17, 31), s(-1, 10), s(8, 19), s(-46, -36), 
];

pub const BIAS: ScoreTuple = s(0, 0);
