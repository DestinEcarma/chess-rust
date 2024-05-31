use crate::{bitboard::Bitboard, square::Square};

pub type NR = usize;

pub const ROOK_MAGIC_NR: [NR; 64] = [
	108086547960578048,
	54043490002149376,
	252236763908538498,
	2774243760896212996,
	5332279628319170568,
	936759734797664264,
	9331467774851023360,
	9331459527425411200,
	9259541573545246740,
	324329542990696448,
	281546934788097,
	281509604950048,
	703721811214465,
	140754693399040,
	1266645993521408,
	2599984372762337410,
	1729523544156553218,
	94576416813228032,
	13835199343868780544,
	4683885999355408386,
	576602039614703616,
	282574622688256,
	4701955924276806161,
	298541596209779844,
	5188296443899166720,
	4617403482113122308,
	2832348397699464,
	434677632687214592,
	2305882593780301954,
	81627786196029448,
	9223373138522276352,
	38563454788863236,
	14268710114209826944,
	9804340924275228872,
	36598621101105169,
	2341881702390960128,
	2883851908257164288,
	1013451220590791168,
	153139988173721880,
	4690674712713,
	317216620838912,
	2328361282765930501,
	4646537215213586,
	36284152217608,
	2522580975236546576,
	18577365643362560,
	288813289130164240,
	5764611923233603593,
	9043208268939392,
	175990609281600,
	10772909928843584000,
	27303227497120000,
	9015997780460800,
	167904223803080832,
	20277264440689664,
	290483586879201792,
	2305913927723132161,
	2394740621328422,
	3747566705814143106,
	155031408019489,
	4785091918172163,
	13999720951055646723,
	166633740397842468,
	4973104291419029578,
];

pub const BISHOP_MAGIC_NR: [NR; 64] = [
	3175321417872935168,
	2614341802106619264,
	1226254534297288862,
	11027074687564792960,
	576760098655846437,
	14997558535397611522,
	1127154609881600,
	36751193373934593,
	4683761273639355456,
	13835695840779438112,
	612916451900260420,
	1154122227239911937,
	54610612919502856,
	4644955859976704,
	11529502066119541793,
	2026691721513927680,
	2251869339068416,
	3495925019958313024,
	578714757717049728,
	4574003335282692,
	2451084786549137410,
	288300753493169156,
	11530411731886018560,
	1161085419927554,
	9323858737923088,
	776255344477201,
	1152974283614715968,
	4656765997330481224,
	9223653580836225028,
	3460183991955571456,
	9228580423603847680,
	4611758037143684112,
	164383619878561858,
	6757804622873601,
	212215677324289,
	11575692779126864,
	1126179348414720,
	6053409649931190404,
	580964996178547200,
	9224005909636736000,
	73225312977028224,
	9370046954578024,
	2455025297171424258,
	9231259942197932544,
	36319072450790208,
	5767431137630617632,
	2315158423915758976,
	9261661434079676448,
	76000520581678080,
	9376777688284930050,
	1127635082544192,
	306574638954317831,
	287575449403521,
	1161093036769312,
	162134569024684320,
	2418716676112548096,
	217052942380318728,
	1442702744125196816,
	721007054202361856,
	37296533935490048,
	9227972393773958656,
	72102141707305472,
	2594653944703095296,
	1229517884942849184,
];

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, Default)]
pub struct Magic {
	pub mask: Bitboard,
	pub shift: u8,
	pub offset: usize,
	pub nr: NR,
}

impl Magic {
	#[inline(always)]
	pub fn get_index(&self, blocker: Bitboard) -> Square {
		(((blocker & self.mask).wrapping_mul(self.nr) >> self.shift) + self.offset) as Square
	}
}
