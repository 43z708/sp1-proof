//
//data.js スプライトデータとか
//

//敵マスター
class TekiMaster {
	constructor(tnum, r, hp, score) {
		this.tnum = tnum;
		this.r = r;
		this.hp = hp;
		this.score = score;
	}
}

let tekiMaster = [
	new TekiMaster(0, 10, 1, 100), //0,敵A
	new TekiMaster(1, 10, 1, 100), //1,敵B
	new TekiMaster(2, 70, 5000, 10000), //2,敵H
	new TekiMaster(3, 10, 1, 10), //3,敵I
];

//スプライトクラス
class Sprite {
	constructor(x, y, w, h) {
		this.x = x;
		this.y = y;
		this.w = w;
		this.h = h;
	}
}

//スプライト
let sprite = [
	new Sprite(0, 0, 22, 42), //0,自機 左2
	new Sprite(23, 0, 33, 42), //1,自機 左1
	new Sprite(57, 0, 43, 42), //2,自機 正面
	new Sprite(101, 0, 33, 42), //3,自機 右1
	new Sprite(135, 0, 21, 42), //4,自機 右2

	new Sprite(0, 50, 3, 7), //5,弾1
	new Sprite(4, 50, 5, 5), //6,弾2

	new Sprite(3, 42, 16, 5), // 7,噴射 左2
	new Sprite(29, 42, 21, 5), // 8,噴射 左1
	new Sprite(69, 42, 19, 5), // 9,噴射 正面
	new Sprite(108, 42, 21, 5), //10,噴射 右1
	new Sprite(138, 42, 16, 5), //11,噴射 右2

	new Sprite(11, 50, 7, 7), //12,敵弾1-1
	new Sprite(19, 50, 7, 7), //13,敵弾1-2
	new Sprite(32, 49, 8, 8), //14,敵弾2-1
	new Sprite(42, 47, 12, 12), //15,敵弾2-2

	new Sprite(5, 351, 9, 9), //16  ,爆発1
	new Sprite(21, 346, 20, 20), //17  ,爆発2
	new Sprite(46, 343, 29, 27), //18  ,爆発3
	new Sprite(80, 343, 33, 30), //19  ,爆発4
	new Sprite(117, 340, 36, 33), //20  ,爆発5
	new Sprite(153, 340, 37, 33), //21  ,爆発6
	new Sprite(191, 341, 25, 31), //22  ,爆発7
	new Sprite(216, 349, 19, 16), //23  ,爆発8
	new Sprite(241, 350, 15, 14), //24  ,爆発9
	new Sprite(259, 350, 14, 13), //25  ,爆発10
	new Sprite(276, 351, 13, 12), //26  ,爆発11

	new Sprite(6, 373, 9, 9), //27  ,ヒット1
	new Sprite(19, 371, 16, 15), //28  ,ヒット2
	new Sprite(38, 373, 11, 12), //29  ,ヒット3
	new Sprite(54, 372, 17, 17), //30  ,ヒット4
	new Sprite(75, 374, 13, 14), //31  ,ヒット5

	new Sprite(4, 62, 24, 27), //32  ,敵B1
	new Sprite(36, 62, 24, 27), //33  ,敵B2
	new Sprite(68, 62, 24, 27), //34  ,敵B3
	new Sprite(100, 62, 24, 27), //35  ,敵B4
	new Sprite(133, 62, 24, 27), //36  ,敵B5
	new Sprite(161, 62, 30, 27), //37  ,敵B6

	new Sprite(4, 95, 24, 26), //38  ,敵A1
	new Sprite(36, 95, 24, 26), //39  ,敵A2
	new Sprite(68, 95, 24, 26), //40  ,敵A3
	new Sprite(100, 95, 24, 26), //41  ,敵A4
	new Sprite(133, 92, 24, 29), //42  ,敵A5
	new Sprite(161, 95, 30, 26), //43  ,敵A6

	new Sprite(4, 125, 24, 29), //44  ,敵C1
	new Sprite(36, 125, 24, 29), //45  ,敵C2
	new Sprite(68, 125, 24, 29), //46  ,敵C3
	new Sprite(100, 125, 24, 29), //47  ,敵C4
	new Sprite(133, 124, 24, 30), //48  ,敵C5
	new Sprite(161, 125, 30, 29), //49  ,敵C6

	new Sprite(4, 160, 25, 27), //50  ,敵D1
	new Sprite(34, 160, 26, 27), //51  ,敵D2
	new Sprite(66, 160, 26, 27), //52  ,敵D3
	new Sprite(98, 160, 26, 27), //53  ,敵D4
	new Sprite(132, 160, 26, 27), //54  ,敵D5
	new Sprite(161, 158, 30, 29), //55  ,敵D6

	new Sprite(4, 194, 24, 28), //56  ,敵E1
	new Sprite(36, 194, 24, 28), //57  ,敵E2
	new Sprite(68, 194, 24, 28), //58  ,敵E3
	new Sprite(100, 194, 24, 28), //59  ,敵E4
	new Sprite(133, 194, 24, 30), //60  ,敵E5
	new Sprite(161, 194, 30, 28), //61  ,敵E6

	new Sprite(4, 230, 22, 26), //62  ,敵F1
	new Sprite(41, 230, 22, 26), //63  ,敵F2
	new Sprite(73, 230, 22, 26), //64  ,敵F3
	new Sprite(105, 230, 22, 26), //65  ,敵F4
	new Sprite(137, 230, 22, 26), //66  ,敵F5

	new Sprite(6, 261, 24, 28), //67  ,敵G1
	new Sprite(38, 261, 24, 28), //68  ,敵G2
	new Sprite(70, 261, 24, 28), //69  ,敵G3
	new Sprite(102, 261, 24, 28), //70  ,敵G4
	new Sprite(135, 261, 24, 28), //71  ,敵G5

	new Sprite(206, 58, 69, 73), //72  ,敵B(中)
	new Sprite(204, 134, 69, 73), //73  ,敵A(中)
	new Sprite(205, 212, 69, 78), //74  ,敵C(中)

	new Sprite(337, 0, 139, 147), //75  ,敵B(大)
	new Sprite(336, 151, 139, 147), //76  ,敵A(大)
	new Sprite(336, 301, 139, 155), //77  ,敵C()
];
