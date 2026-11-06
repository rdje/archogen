//! The variant's model by unit steps, with what a paper timeline leaves out: each masked run contiguous with its decided switch, as `L` reads it; `δ`, `γ` and a compare's resolution; `W_wake`; several
//! alarms; tasks released by an alarm; and, the point of it, a read of the time apart from a write's landing, with the floor computed from what the code can know at that read.
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct TaskSpec {
    pub name: String,
    pub rank: i64,
    pub period: u64,
    pub own: u64,
    /// (alarm, deadline offset from the nominal release), by turns, one per write.
    pub arms: Vec<(usize, i64)>,
    pub writes_per_job: usize,
    pub released_by: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct AlarmSpec {
    pub c_s: u64,
    pub t_s: u64,
    pub reach: i64,
    pub rank: i64,
    pub floored: bool,
    /// How far into its service the read of the time falls; `read_at`, the lag to the landing and the lag to the read after it lie within `c_s`.
    pub read_at: u64,
    pub wake: Option<i64>,
    /// Its service also programs that alarm, this far past its read, as the role allows a service that calls a node.
    pub calls: Option<(usize, i64)>,
}

#[derive(Clone, Copy, Debug)]
pub struct Plat {
    pub s: u64,
    pub c_rel: u64,
    pub completion: u64,
    pub arm: u64,
    pub delta: u64,
    pub gamma: u64,
    /// The compare's resolution: ρ is `res - 1`.
    pub res: u64,
    pub wake: u64,
    pub external_first: bool,
}

/// What the code writing a compare takes as the last arrival.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Know {
    /// The timeline's own, at the landing: what `alarm_model.rs` does.
    Omniscient,
    /// The last compare value it wrote that had been reached at its read.
    ValueWritten,
    /// The record's protocol, as it says it: a compare standing that may match by the read and the lag is left, and its service picks up what is pending. Otherwise it writes, and reads the time again in the
    /// same masked run, which bounds the last arrival from above.
    WhatItReadAndWrote,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Device {
    /// An absolute level compare: arrival at max(value, landing).
    Compare,
    /// A delay-only timer, programmed `value - read`: arrival at landing + delay.
    Delay,
}

#[derive(Clone, Debug)]
pub struct LagModel {
    pub max: u64,
    /// The landing's actual lag, by turns, each at most `max`; and the next turn's, from the landing to the read after it.
    pub pattern: Vec<u64>,
    /// How far past that the read after the landing falls, which the record's `λ` says is nothing.
    pub past: u64,
}

#[derive(Debug, Default, Clone)]
pub struct Measured {
    pub worst: Vec<i64>,
    pub arrivals: Vec<Vec<i64>>,
    pub merged: u32,
    /// Per alarm, the latest a service ended past its deadline or write.
    pub latest_delivery: Vec<i64>,
    pub worst_delivery_case: Vec<(i64, i64, i64)>,
    pub longest_read_to_read: Vec<i64>,
    pub worst_wait: Vec<i64>,
    /// A deadline still pending at the horizon, and how long.
    pub oldest_pending: Vec<i64>,
    pub programmed_past_reach: u32,
}

impl Measured {
    pub fn least_separation(&self, a: usize) -> i64 {
        self.arrivals[a]
            .windows(2)
            .map(|p| p[1] - p[0])
            .min()
            .unwrap_or(i64::MAX)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Trap {
    Alarm(usize),
    Timer,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Doing {
    Entry { left: u64, trap: Trap },
    Alarm { a: usize, left: u64, done: u64 },
    Timer { left: u64 },
    Switch { to: Option<usize>, left: u64 },
}

struct Alarm {
    index: usize,
    spec: AlarmSpec,
    pending: Vec<(i64, i64)>,
    wake_next: Option<i64>,
    /// When its compare matches or its delay ends.
    fires: Option<i64>,
    request: Option<i64>,
    serving_arrival: i64,
    last_arrival: Option<i64>,
    /// A write read but not landed: (land at, value or None, read).
    inflight: Option<(i64, Option<i64>, i64)>,
    code_last: Option<i64>,
    shadow: Option<i64>,
    last_read: Option<i64>,
    being_delivered: Vec<(i64, i64)>,
    writes: usize,
}

#[derive(Clone, Copy)]
struct Seg {
    masked: bool,
    len: u64,
    arm: bool,
}

struct Job {
    nominal: i64,
    seg: usize,
    left: u64,
    debt: u64,
}

struct Live {
    spec: TaskSpec,
    segs: Vec<Seg>,
    jobs: VecDeque<Job>,
    next_release: i64,
    worst: i64,
    write_turn: usize,
}

fn ceil_to(value: i64, res: u64) -> i64 {
    let res = i64::try_from(res).expect("small");
    if res <= 1 {
        value
    } else {
        (value + res - 1).div_euclid(res) * res
    }
}

pub struct Run<'a> {
    pub tasks: &'a [TaskSpec],
    pub alarms: &'a [AlarmSpec],
    pub plat: Plat,
    pub know: Know,
    pub device: Device,
    pub lag: LagModel,
    pub horizon: i64,
}

impl Run<'_> {
    /// The earliest the compare standing can raise, as the code reckons it from its shadow.
    fn read_event(
        &self,
        al: &mut Alarm,
        read: i64,
        deadline: Option<i64>,
        measured: &mut Measured,
    ) {
        let lag_max = i64::try_from(self.lag.max).expect("small");
        let actual =
            i64::try_from(self.lag.pattern[al.writes % self.lag.pattern.len()]).expect("small");
        let back = i64::try_from(
            self.lag.pattern[(al.writes + 1) % self.lag.pattern.len()] + self.lag.past,
        )
        .expect("small");
        al.writes += 1;
        let land = read + actual;
        if self.know == Know::WhatItReadAndWrote {
            self.what_it_read_and_wrote(al, read, land, land + back, deadline, measured);
            return;
        }
        // A compare reached at this very instant has raised: a read there sees it, and no write undoes it.
        if self.know != Know::Omniscient && al.fires.is_some_and(|f| read >= f) {
            al.fires = None;
            if al.request.is_some() {
                measured.merged += 1;
            } else {
                al.request = Some(read);
                al.last_arrival = Some(read);
                measured.arrivals[al.index].push(read);
            }
        }
        let mut unneeded = false;
        if let Some(d) = deadline {
            let earliest = al.pending.iter().map(|p| p.0).chain(al.wake_next).min();
            al.pending.push((d, land));
            if self.know != Know::Omniscient
                && (al.fires.is_some() || al.request.is_some())
                && earliest.is_some_and(|e| d >= e)
            {
                unneeded = true;
            }
        }
        // What the code takes as the last arrival, at its read.
        match self.know {
            Know::Omniscient => {}
            Know::ValueWritten => {
                if let Some(s) = al.shadow {
                    if s <= read {
                        al.code_last = Some(s);
                    }
                }
            }
            Know::WhatItReadAndWrote => unreachable!("its own path"),
        }
        if unneeded {
            return;
        }
        let next = al.pending.iter().map(|p| p.0).chain(al.wake_next).min();
        let t_s = i64::try_from(al.spec.t_s).expect("small");
        let known = if self.know == Know::Omniscient {
            al.last_arrival
        } else {
            al.code_last
        };
        let value = next.map(|n| {
            let mut at = n.min(read + al.spec.reach).max(read);
            if al.spec.floored {
                if let Some(last) = known {
                    if last + t_s > read + al.spec.reach && last + t_s > n {
                        measured.programmed_past_reach += 1;
                    }
                    at = at.max(last + t_s);
                }
            }
            match self.device {
                Device::Compare => ceil_to(at, self.plat.res),
                // A delay counts from where it lands: rounded from the read, so that it is never early.
                Device::Delay => read + ceil_to(at - read, self.plat.res),
            }
        });
        // What it will know from reading again after the write, in the same masked run.
        if let Some(v) = value {
            match self.device {
                Device::Compare => al.shadow = Some(v),
                Device::Delay => {
                    // Fires at landing + (v - read), at most this.
                    al.shadow = Some(v.max(read) + lag_max);
                    if self.know == Know::ValueWritten {
                        al.shadow = Some(v);
                    }
                }
            }
        } else {
            al.shadow = None;
        }
        al.inflight = Some((land, value, read));
    }

    fn what_it_read_and_wrote(
        &self,
        al: &mut Alarm,
        read: i64,
        land: i64,
        read_back: i64,
        deadline: Option<i64>,
        measured: &mut Measured,
    ) {
        let lag_max = i64::try_from(self.lag.max).expect("small");
        let t_s = i64::try_from(al.spec.t_s).expect("small");
        // Its own service alone: any other write, another alarm's service among them, takes a job's rule.
        let is_service = deadline.is_none();
        // A read is a whole tick of the counter, and the true time is up to `ρ` past it.
        let res = i64::try_from(self.plat.res).expect("small");
        let rho = res - 1;
        let seen = read.div_euclid(res) * res;
        let seen_back = read_back.div_euclid(res) * res;
        // A compare reached at this very instant has raised: a read there sees it, and no write undoes it.
        if al.fires.is_some_and(|f| read >= f) {
            al.fires = None;
            if al.request.is_some() {
                measured.merged += 1;
            } else {
                al.request = Some(read);
                al.last_arrival = Some(read);
                measured.arrivals[al.index].push(read);
            }
        }
        if let Some(d) = deadline {
            al.pending.push((d, land));
        }
        // What a value arrives by, at the latest, where it arrives at all: a delay lands late by its lag and what its read was behind.
        let latest = |value: i64| match self.device {
            Device::Compare => value,
            Device::Delay => value + lag_max + rho,
        };
        // A service: the compare standing, if at or before this read, is the one that raised it.
        if is_service {
            if let Some(standing) = al.shadow {
                if standing <= seen {
                    al.code_last = Some(
                        al.code_last
                            .map_or(latest(standing), |last| last.max(latest(standing))),
                    );
                    al.shadow = None;
                }
            }
        }
        // A compare standing that may match by the read and the lag is left: its service picks up what is pending.
        if al
            .shadow
            .is_some_and(|standing| standing <= seen + rho + lag_max)
        {
            return;
        }
        let next = al.pending.iter().map(|p| p.0).chain(al.wake_next).min();
        let value = next.map(|n| {
            let mut at = n.min(seen + al.spec.reach).max(seen);
            if let Some(last) = al.code_last {
                at = at.max(last + t_s);
            }
            match self.device {
                Device::Compare => ceil_to(at, self.plat.res),
                Device::Delay => seen + ceil_to(at - seen, self.plat.res),
            }
        });
        if let Some(v) = value {
            if v - seen > al.spec.reach + rho {
                measured.programmed_past_reach += 1;
            }
            match self.device {
                // Read again within the lag of the landing, still masked: at or before that, it has arrived or will at once, and nothing stands. `v` is the value rounded up, as the compare holds it, on one grid with the read.
                Device::Compare if v <= seen_back => {
                    let bound = seen_back + rho;
                    al.code_last = Some(al.code_last.map_or(bound, |last| last.max(bound)));
                    al.shadow = None;
                }
                _ => al.shadow = Some(v),
            }
        } else {
            al.shadow = None;
        }
        al.inflight = Some((land, value, seen));
    }

    fn land(&self, al: &mut Alarm, t: i64) {
        if let Some((land, value, read)) = al.inflight {
            if land <= t {
                al.fires = value.map(|v| match self.device {
                    Device::Compare => v,
                    Device::Delay => land + ceil_to((v - read).max(0), self.plat.res),
                });
                al.inflight = None;
            }
        }
    }

    pub fn go(&self) -> Measured {
        let lag_max = self.lag.max;
        let n_alarms = self.alarms.len();
        let mut tasks: Vec<Live> = self
            .tasks
            .iter()
            .map(|spec| {
                let mut segs = Vec::new();
                if spec.writes_per_job == 0 || spec.arms.is_empty() {
                    segs.push(Seg {
                        masked: false,
                        len: spec.own,
                        arm: false,
                    });
                } else {
                    let n = u64::try_from(spec.writes_per_job).expect("small");
                    let part = spec.own / n;
                    let first = spec.own - part * (n - 1);
                    for at in 0..n {
                        let len = if at == 0 { first } else { part };
                        if len > 0 {
                            segs.push(Seg {
                                masked: false,
                                len,
                                arm: false,
                            });
                        }
                        segs.push(Seg {
                            masked: true,
                            len: self.plat.arm,
                            arm: true,
                        });
                    }
                }
                segs.push(Seg {
                    masked: true,
                    len: self.plat.completion,
                    arm: false,
                });
                segs.retain(|s| s.len > 0);
                Live {
                    spec: spec.clone(),
                    segs,
                    jobs: VecDeque::new(),
                    next_release: 0,
                    worst: 0,
                    write_turn: 0,
                }
            })
            .collect();
        let mut alarms: Vec<Alarm> = self
            .alarms
            .iter()
            .enumerate()
            .map(|(index, spec)| Alarm {
                index,
                spec: *spec,
                pending: Vec::new(),
                wake_next: spec.wake,
                fires: None,
                request: None,
                serving_arrival: 0,
                last_arrival: None,
                inflight: None,
                code_last: None,
                shadow: None,
                last_read: None,
                being_delivered: Vec::new(),
                writes: 0,
            })
            .collect();
        for al in &mut alarms {
            if let Some(w) = al.wake_next {
                al.fires = Some(ceil_to(w, self.plat.res));
                al.shadow = al.fires;
            }
        }
        let mut order: Vec<usize> = (0..n_alarms).collect();
        order.sort_by_key(|a| self.alarms[*a].rank);
        let mut measured = Measured {
            arrivals: vec![Vec::new(); n_alarms],
            latest_delivery: vec![0; n_alarms],
            worst_delivery_case: vec![(0, 0, 0); n_alarms],
            longest_read_to_read: vec![0; n_alarms],
            worst_wait: vec![0; n_alarms],
            oldest_pending: vec![0; n_alarms],
            ..Measured::default()
        };
        let (mut doing, mut running): (Option<Doing>, Option<usize>) = (None, None);
        let top_of = |tasks: &[Live]| {
            tasks
                .iter()
                .enumerate()
                .filter(|(_, t)| !t.jobs.is_empty())
                .min_by_key(|(_, t)| t.spec.rank)
                .map(|(at, _)| at)
        };
        for t in 0..self.horizon {
            for al in &mut alarms {
                self.land(al, t);
                if al.fires.is_some_and(|f| t >= f) {
                    al.fires = None;
                    if al.request.is_some() {
                        // The level line is already up: no second request, and no second arrival to separate.
                        measured.merged += 1;
                    } else {
                        al.request = Some(t);
                        al.last_arrival = Some(t);
                        measured.arrivals[al.index].push(t);
                    }
                }
            }
            let due = tasks.iter().any(|task| {
                task.spec.released_by.is_none() && ceil_to(task.next_release, self.plat.res) <= t
            });
            if doing.is_none() {
                let mid_masked = running.is_some_and(|at| {
                    tasks[at].jobs.front().is_some_and(|j| {
                        tasks[at].segs[j.seg].masked && j.left < tasks[at].segs[j.seg].len
                    })
                });
                if !mid_masked {
                    let first_alarm = order.iter().copied().find(|a| alarms[*a].request.is_some());
                    let trap = match (first_alarm, due) {
                        (Some(a), true) => Some(if self.plat.external_first {
                            Trap::Alarm(a)
                        } else {
                            Trap::Timer
                        }),
                        (Some(a), false) => Some(Trap::Alarm(a)),
                        (None, true) => Some(Trap::Timer),
                        (None, false) => None,
                    };
                    if let Some(trap) = trap {
                        let idle = running.is_none();
                        let left = self.plat.delta + if idle { self.plat.wake } else { 0 };
                        doing = Some(Doing::Entry { left, trap });
                    }
                }
            }
            // An entry of no length starts its service in this unit.
            if let Some(Doing::Entry { left: 0, trap }) = doing {
                match trap {
                    Trap::Alarm(a) => {
                        let arrival = alarms[a].request.take().expect("pending");
                        alarms[a].serving_arrival = arrival;
                        measured.worst_wait[a] = measured.worst_wait[a].max(t - arrival);
                        doing = Some(Doing::Alarm {
                            a,
                            left: self.alarms[a].c_s,
                            done: 0,
                        });
                    }
                    Trap::Timer => {
                        doing = Some(Doing::Timer {
                            left: self.plat.c_rel,
                        })
                    }
                }
                if let Some(at) = running {
                    if let Some(j) = tasks[at].jobs.front_mut() {
                        j.debt += self.plat.gamma;
                    }
                }
            }
            let now = t + 1;
            match doing {
                Some(Doing::Entry { left, trap }) => {
                    doing = Some(Doing::Entry {
                        left: left - 1,
                        trap,
                    });
                }
                Some(Doing::Alarm { a, left, done }) => {
                    let (left, done) = (left - 1, done + 1);
                    let mut calls = None;
                    let al = &mut alarms[a];
                    if done == al.spec.read_at {
                        if let Some(last) = al.last_read {
                            measured.longest_read_to_read[a] =
                                measured.longest_read_to_read[a].max(now - last);
                        }
                        al.last_read = Some(now);
                        // What the code sees due: its read is a whole tick, under the protocol that says so.
                        let res = i64::try_from(self.plat.res).expect("small");
                        let seen = if self.know == Know::WhatItReadAndWrote {
                            now.div_euclid(res) * res
                        } else {
                            now
                        };
                        let (delivered, kept): (Vec<_>, Vec<_>) =
                            al.pending.iter().copied().partition(|(d, _)| *d <= seen);
                        al.pending = kept;
                        al.being_delivered.extend(delivered);
                        if let (Some(every), Some(next)) = (al.spec.wake, al.wake_next) {
                            if next <= seen {
                                al.wake_next = Some((now / every + 1) * every);
                            }
                        }
                        self.read_event(al, now, None, &mut measured);
                        calls = al.spec.calls;
                    }
                    if let Some((to, ahead)) = calls {
                        self.read_event(&mut alarms[to], now, Some(now + ahead), &mut measured);
                    }
                    let al = &mut alarms[a];
                    doing = (left > 0).then_some(Doing::Alarm { a, left, done });
                    if left == 0 {
                        let arrival = al.serving_arrival;
                        for (deadline, written) in al.being_delivered.drain(..) {
                            let late = now - deadline.max(written);
                            if late > measured.latest_delivery[a] {
                                measured.latest_delivery[a] = late;
                                measured.worst_delivery_case[a] = (deadline, written, now);
                            }
                        }
                        for task in &mut tasks {
                            if task.spec.released_by == Some(a) {
                                let first = task.segs[0].len;
                                task.jobs.push_back(Job {
                                    nominal: arrival,
                                    seg: 0,
                                    left: first,
                                    debt: 0,
                                });
                            }
                        }
                    }
                }
                Some(Doing::Timer { left }) => {
                    let left = left - 1;
                    doing = (left > 0).then_some(Doing::Timer { left });
                    if left == 0 {
                        let check = now - i64::try_from(self.plat.c_rel).expect("small");
                        for task in &mut tasks {
                            if task.spec.released_by.is_some() {
                                continue;
                            }
                            while ceil_to(task.next_release, self.plat.res) <= check {
                                let first = task.segs[0].len;
                                task.jobs.push_back(Job {
                                    nominal: task.next_release,
                                    seg: 0,
                                    left: first,
                                    debt: 0,
                                });
                                task.next_release +=
                                    i64::try_from(task.spec.period).expect("small");
                            }
                        }
                    }
                }
                Some(Doing::Switch { to, left }) => {
                    let left = left - 1;
                    if left == 0 {
                        running = to;
                        doing = None;
                    } else {
                        doing = Some(Doing::Switch { to, left });
                    }
                }
                None => {
                    if let Some(at) = running {
                        let task = &mut tasks[at];
                        let segs = task.segs.clone();
                        if let Some(j) = task.jobs.front_mut() {
                            if j.debt > 0 {
                                j.debt -= 1;
                            } else {
                                j.left -= 1;
                                let seg = segs[j.seg];
                                if seg.arm && j.left == lag_max {
                                    let (a, offset) =
                                        task.spec.arms[task.write_turn % task.spec.arms.len()];
                                    task.write_turn += 1;
                                    let nominal = j.nominal;
                                    self.read_event(
                                        &mut alarms[a],
                                        now,
                                        Some(nominal + offset),
                                        &mut measured,
                                    );
                                }
                                if j.left == 0 {
                                    if j.seg + 1 == segs.len() {
                                        let nominal = j.nominal;
                                        task.worst = task.worst.max(now - nominal);
                                        task.jobs.pop_front();
                                    } else {
                                        j.seg += 1;
                                        j.left = segs[j.seg].len;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if doing.is_none() {
                // A decision at a service's end or a job's completion: the switch is contiguous with it.
                let top = top_of(&tasks);
                if top != running {
                    // A preemption, or a transition to what runs next, here in the one masked run.
                    if let Some(at) = running {
                        let len0 = tasks[at].segs[0].len;
                        if let Some(j) = tasks[at].jobs.front_mut() {
                            if j.seg > 0 || j.left < len0 {
                                j.debt += self.plat.gamma;
                            }
                        }
                    }
                    // Only a masked stretch's own end, a service's or a completion's, decides. A switch that just ended never starts another
                    // unless the top changed during it, which the run reaches the same way.
                    let mid = running.is_some_and(|at| {
                        tasks[at].jobs.front().is_some_and(|j| {
                            tasks[at].segs[j.seg].masked && j.left < tasks[at].segs[j.seg].len
                        })
                    });
                    if !mid {
                        doing = Some(Doing::Switch {
                            to: top,
                            left: self.plat.s,
                        });
                    }
                }
            }
        }
        for (i, task) in tasks.iter().enumerate() {
            let owed = task.jobs.front().map_or(0, |j| self.horizon - j.nominal);
            let _ = i;
            measured.worst.push(task.worst.max(owed));
        }
        for (a, al) in alarms.iter().enumerate() {
            let oldest = al
                .pending
                .iter()
                .map(|(d, w)| self.horizon - (*d).max(*w))
                .max()
                .unwrap_or(0);
            measured.oldest_pending[a] = oldest.max(0);
        }
        measured
    }
}
