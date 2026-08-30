#[derive(Debug, Clone)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}
impl Interval {
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

struct MeetingRooms {}
impl MeetingRooms {
    /*
    問題の理解:
    会議の開始時刻と終了時刻からなる会議時間間隔オブジェクトの配列が与えられる。任意の順序となっている。
    与えられた会議時間全てが重複しないかを確認して、重複しなければtrue,重複するようであればfalseを返す。
    [(0,30),(5,10),(15,20)]のとき、
    (0,30), (5,10) が重複している。
    (0,30), (15,20) が重複している。
    答えはfalseとなる。

    memo:
    数直線上にして考えてみる。
    endの時間でintervalsをソートする。
    intervals[i].end < intervals[i+1].start であれば会議時間は重複していない。
    重複を見つけたら早期リターンして、関数の終わりに到達したら重複なしということでtrueを返す。
    ソートの時間計算量が支配的なのでO(n log n)となる。問題の制約からnは500が上限なので問題ない。

    Wrong Answerとなった。intervals[i].end <= intervals[i+1].start 条件の境界値判定を間違えていた。
    修正してAccepted
    */
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_by_key(|interval| interval.end);
        for i in 0..intervals.len() {
            let Some(next_interval) = intervals.get(i + 1) else {
                break;
            };
            if intervals[i].end <= next_interval.start {
                continue;
            }
            return false;
        }

        true
    }

    pub fn can_attend_meetings_2(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_by_key(|interval| interval.end);
        intervals.windows(2).all(|window| {
            let (interval, next_interval) = (&window[0], &window[1]);
            interval.end <= next_interval.start
        })
    }
}
