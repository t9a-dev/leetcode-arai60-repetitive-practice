use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone)]
pub struct Interval {
    start: i32,
    end: i32,
}
impl Interval {
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

struct MinRequiredMeetingRoomsManager {
    end_times: BinaryHeap<Reverse<i32>>,
    rooms_count: i32,
}
impl MinRequiredMeetingRoomsManager {
    pub fn new() -> Self {
        Self {
            end_times: BinaryHeap::new(),
            rooms_count: 0,
        }
    }

    pub fn reserve(&mut self, interval: Interval) {
        let Some(Reverse(reserved_end_time)) = self.end_times.peek() else {
            self.rooms_count += 1;
            self.end_times.push(Reverse(interval.end));
            return;
        };

        if *reserved_end_time <= interval.start {
            self.end_times.pop();
            self.end_times.push(Reverse(interval.end));
        } else {
            self.end_times.push(Reverse(interval.end));
            self.rooms_count += 1;
        }
    }
}

/*
start == endが同じ時は先に部屋を開けて、次の会議が始まると考える。
つまり、優先的にendの方を扱いたい。
*/
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum MeetingEvent {
    End,
    Start,
}
struct MeetingRoomsII {}
impl MeetingRoomsII {
    /*
    問題の理解:
    開始時刻と終了時刻からなる会議時間間隔オブジェクトの配列intervalsが与えられる。
    競合なく全ての会議をスケジュールするために必要な最小の会議室数を求めて返す。

    memo:
    会議終了時間でソートを行う。
    会議時間の重複を見つけるたびにmin_required_roomsカウントをインクリメントする。
    このやり方だとうまく判定できないケースがある。
    [(0,5),(4,9),(8,13)]
    会議室は2つあれば良いが、答えが3になり正しくない。
    一度使った会議室が使い終わってもカウントに残り続けるため。
    min heapで会議室の終了時間を管理して、会議室の数を増やす時の判断材料とする。
    min_heap.end <= intervals[i].start then min_heap.pop(), min_heap.push(intervals[i].end)
    else min_heap.push(intervals[i].end), min_required_count += 1
    すでに会議が終了している会議室を見つけたら会議室数を増やさない。そうでなければ会議室数を増やすと考える。
    状態を管理する構造体を増やした方が良さそう。

    Wrong Answerとなった。intervalsはendではなく、startでソートしないと時間の流れがおかしくなる。
    intervalsをstartでソートするように修正してAccepted

    所感:
    Meeting Roomsの問題に引っ張られてとりあえずintervals.endでソートしておけばよいという感じでよく考えていなかったのが致命的だった。
    他のロジックは実装できていたので非常に良くないことをしてしまった。
    */
    pub fn min_meeting_rooms(mut intervals: Vec<Interval>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        let mut min_required_meeting_rooms_manager = MinRequiredMeetingRoomsManager::new();
        intervals.sort_by_key(|interval| interval.start);
        intervals
            .into_iter()
            .for_each(|interval| min_required_meeting_rooms_manager.reserve(interval));

        min_required_meeting_rooms_manager.rooms_count
    }

    /*
    ミーティングが始まったら使用中の部屋数をインクリメントする。
    ミーティングが終わったら使用中の部屋数をデクリメントする。
    この処理の中で一番多く必要となった部屋数のカウント=最低限必要な最小の部屋の数となる。
    interval[i].start == intarval[i+1].end となったときに、ミーティングが終わった方を優先的に処理する必要がある。
    [(0,1),(1,2)]のとき必要な部屋数は1とカウントする必要があるため。
    */
    pub fn min_meeting_rooms_2(intervals: Vec<Interval>) -> i32 {
        let mut meeting_events = intervals
            .into_iter()
            .flat_map(|interval| {
                vec![
                    (interval.end, MeetingEvent::End),
                    (interval.start, MeetingEvent::Start),
                ]
            })
            .collect::<Vec<_>>();
        meeting_events.sort();

        let mut min_required_meeting_rooms_count = 0;
        let mut using_rooms_count = 0;
        for (_, meeting_event) in meeting_events {
            match meeting_event {
                MeetingEvent::Start => using_rooms_count += 1,
                MeetingEvent::End => using_rooms_count -= 1,
            };
            min_required_meeting_rooms_count =
                min_required_meeting_rooms_count.max(using_rooms_count);
        }

        min_required_meeting_rooms_count
    }
}
