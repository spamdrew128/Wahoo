#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::{evaluation::ScoreTuple, board_representation::{NUM_SQUARES, NUM_PIECES}};

macro_rules! s {
  ($mg:expr, $eg:expr) => {
    ScoreTuple::new($mg, $eg)
  };
}

pub const PST: [[ScoreTuple; NUM_SQUARES as usize]; NUM_PIECES as usize] = [
// Knight PST
[
  s!(97, 196), s!(126, 282), s!(172, 239), s!(275, 159), s!(220, 151), s!(187, 178), s!(174, 117), s!(172, 170), 
  s!(186, 196), s!(288, 122), s!(297, 159), s!(268, 200), s!(302, 166), s!(310, 182), s!(296, 140), s!(246, 162), 
  s!(256, 168), s!(259, 193), s!(291, 220), s!(335, 166), s!(335, 160), s!(389, 154), s!(295, 166), s!(242, 181), 
  s!(240, 166), s!(259, 179), s!(289, 175), s!(291, 195), s!(285, 196), s!(295, 178), s!(265, 212), s!(298, 158), 
  s!(237, 182), s!(253, 188), s!(261, 184), s!(252, 212), s!(275, 188), s!(272, 191), s!(256, 193), s!(255, 162), 
  s!(223, 157), s!(267, 158), s!(252, 176), s!(250, 215), s!(258, 178), s!(263, 188), s!(277, 162), s!(245, 179), 
  s!(215, 167), s!(215, 265), s!(248, 176), s!(240, 197), s!(244, 184), s!(277, 174), s!(238, 207), s!(233, 179), 
  s!(254, 66), s!(217, 141), s!(257, 206), s!(176, 213), s!(241, 202), s!(234, 159), s!(232, 177), s!(253, 147), 
],
// Bishop PST
[
  s!(172, 228), s!(257, 207), s!(256, 199), s!(117, 229), s!(167, 174), s!(160, 191), s!(166, 218), s!(245, 183), 
  s!(289, 215), s!(318, 191), s!(277, 201), s!(330, 175), s!(330, 174), s!(274, 227), s!(266, 164), s!(305, 148), 
  s!(267, 209), s!(291, 205), s!(308, 198), s!(294, 209), s!(334, 183), s!(367, 202), s!(293, 209), s!(278, 182), 
  s!(278, 191), s!(279, 233), s!(326, 205), s!(291, 228), s!(323, 192), s!(318, 225), s!(282, 200), s!(287, 197), 
  s!(272, 193), s!(290, 198), s!(281, 236), s!(302, 195), s!(308, 208), s!(292, 197), s!(276, 221), s!(276, 233), 
  s!(275, 203), s!(291, 226), s!(285, 209), s!(267, 227), s!(290, 204), s!(301, 197), s!(300, 199), s!(282, 186), 
  s!(278, 247), s!(273, 213), s!(283, 181), s!(280, 209), s!(277, 202), s!(292, 197), s!(292, 192), s!(286, 196), 
  s!(291, 195), s!(192, 251), s!(264, 177), s!(283, 172), s!(307, 192), s!(259, 196), s!(186, 250), s!(220, 229), 
],
// Rook PST
[
  s!(501, 292), s!(546, 275), s!(562, 274), s!(512, 314), s!(475, 320), s!(622, 243), s!(537, 286), s!(487, 289), 
  s!(396, 337), s!(422, 331), s!(470, 318), s!(470, 324), s!(458, 318), s!(499, 298), s!(462, 324), s!(464, 297), 
  s!(409, 321), s!(410, 330), s!(357, 342), s!(480, 300), s!(453, 313), s!(425, 309), s!(436, 323), s!(416, 323), 
  s!(370, 352), s!(391, 311), s!(402, 315), s!(412, 299), s!(398, 317), s!(400, 321), s!(404, 323), s!(410, 331), 
  s!(360, 345), s!(412, 294), s!(399, 303), s!(417, 305), s!(378, 334), s!(397, 322), s!(409, 316), s!(352, 327), 
  s!(353, 291), s!(388, 315), s!(380, 334), s!(359, 336), s!(388, 315), s!(403, 284), s!(396, 279), s!(380, 299), 
  s!(362, 334), s!(384, 314), s!(373, 323), s!(397, 302), s!(377, 326), s!(404, 308), s!(369, 318), s!(341, 309), 
  s!(365, 330), s!(370, 341), s!(379, 337), s!(389, 335), s!(395, 331), s!(372, 339), s!(347, 337), s!(363, 306), 
],
// Queen PST
[
  s!(2539, -205), s!(2642, -277), s!(2647, -253), s!(2711, -266), s!(2719, -309), s!(2636, -155), s!(2617, -197), s!(2571, -164), 
  s!(2532, -138), s!(2547, -175), s!(2597, -256), s!(2604, -185), s!(2684, -314), s!(2606, -111), s!(2595, -183), s!(2576, -133), 
  s!(2545, -195), s!(2545, -142), s!(2563, -133), s!(2607, -216), s!(2654, -261), s!(2601, -102), s!(2601, -126), s!(2572, -164), 
  s!(2541, -205), s!(2551, -191), s!(2570, -236), s!(2570, -190), s!(2570, -152), s!(2554, -153), s!(2559, -164), s!(2567, -142), 
  s!(2538, -154), s!(2532, -191), s!(2545, -143), s!(2536, -121), s!(2542, -133), s!(2545, -140), s!(2550, -105), s!(2540, -134), 
  s!(2522, -173), s!(2545, -178), s!(2560, -190), s!(2549, -154), s!(2542, -124), s!(2542, -103), s!(2575, -195), s!(2558, -255), 
  s!(2491, -121), s!(2538, -117), s!(2563, -226), s!(2540, -166), s!(2535, -142), s!(2547, -155), s!(2575, -213), s!(2502, -96), 
  s!(2466, -70), s!(2521, -204), s!(2557, -246), s!(2545, -209), s!(2520, -143), s!(2437, -114), s!(2476, -114), s!(2579, -182), 
],
// Pawn PST
[
  s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), 
  s!(189, 198), s!(121, 204), s!(130, 178), s!(128, 205), s!(134, 172), s!(21, 251), s!(110, 237), s!(55, 227), 
  s!(40, 146), s!(35, 143), s!(57, 148), s!(95, 131), s!(81, 124), s!(67, 145), s!(78, 163), s!(65, 152), 
  s!(38, 126), s!(57, 108), s!(58, 103), s!(67, 81), s!(72, 72), s!(63, 92), s!(67, 109), s!(32, 121), 
  s!(9, 102), s!(42, 92), s!(50, 89), s!(68, 82), s!(67, 74), s!(61, 88), s!(53, 102), s!(12, 87), 
  s!(22, 90), s!(50, 86), s!(52, 75), s!(41, 79), s!(45, 95), s!(55, 86), s!(75, 82), s!(28, 76), 
  s!(2, 102), s!(41, 92), s!(35, 91), s!(21, 88), s!(23, 100), s!(69, 95), s!(64, 90), s!(13, 85), 
  s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), s!(0, 0), 
],
// King PST
[
  s!(225, 65), s!(150, 21), s!(-143, 79), s!(-51, 39), s!(-132, 87), s!(-264, 28), s!(-444, 4), s!(-406, -77), 
  s!(244, 64), s!(-100, 69), s!(-279, 166), s!(-388, 154), s!(-206, 26), s!(-267, -1), s!(-463, -47), s!(-303, 23), 
  s!(77, 32), s!(122, 40), s!(113, 42), s!(-5, 45), s!(227, -127), s!(-181, -14), s!(-124, 6), s!(-108, 18), 
  s!(88, 1), s!(130, -2), s!(-36, 35), s!(-171, 62), s!(49, -7), s!(-94, 30), s!(-38, 31), s!(-146, 9), 
  s!(-38, -9), s!(38, 5), s!(-87, 32), s!(-132, 45), s!(-144, 48), s!(-107, 28), s!(-2, 3), s!(16, -17), 
  s!(-36, 16), s!(0, 21), s!(-23, 17), s!(-59, 30), s!(-33, 35), s!(-60, 26), s!(23, -21), s!(41, -28), 
  s!(13, -14), s!(15, 5), s!(47, 1), s!(7, 8), s!(-5, 15), s!(42, 11), s!(48, -10), s!(46, -4), 
  s!(48, -3), s!(36, -12), s!(65, -34), s!(28, -8), s!(47, -20), s!(41, -16), s!(80, -20), s!(67, -15), 
],
];