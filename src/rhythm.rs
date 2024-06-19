use serde::{Serialize,Deserialize};
use std::{hash::Hash, collections::HashSet};

pub type Sec = f64;
pub type BPM = f64;

pub fn beat_length(tempo: BPM) -> Sec {
    60_f64 / tempo as Sec
}



#[derive(Debug,Default,Clone,PartialEq, Serialize,Deserialize)]
pub struct Rhythm {
    /// Number of beats in a measure
    pub length: usize,
    /// which beats to play; zero-indexed
    pub beats: HashSet<usize>,
    /// Length of a beat, in seconds
    pub duration: Sec,
    /// the current time within the measure
    #[serde(skip)]
    time: Sec,
}

fn canonical(v: f64) -> i64{
    (v*1024.0*1024.0).round() as i64
}

impl Hash for Rhythm{

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.length.hash(state);
        canonical(self.duration).hash(state);
        for b in &self.beats{
            b.hash(state);
        }
        canonical(self.time).hash(state);
    }
}

impl Rhythm{
    pub fn update(&mut self, delta: Sec){
        self.time += delta;
        self.time %= self.duration * self.length as Sec;
    }

    pub fn on(&self) -> bool {
        self.beats.contains(& (self.position().trunc() as usize))
    }

    pub fn position(&self) -> f64 {
        self.time / self.duration
    }

    pub fn beat(&self) -> f64 {
        self.position().trunc()
    }

    pub fn in_window(&self, window_size: Sec)-> bool {
        self.beats.iter().any(
            |beat| {
                let distance = (*beat as f64 - self.position()).abs();
                f64::min(distance, self.length as f64 - distance) < window_size / self.duration
            }
        )
    }

    pub fn on_beat(self, window_size: Sec) -> bool {
        self.beats.iter().any(
            |beat| {
                let beat = *beat as f64;
                let window_rel = window_size / self.duration;
                let start = (beat - window_rel) % self.length as f64;
                let end = (beat + 1.0 + window_rel) % self.length as f64;
                
                (start <= self.position() && self.position() < end) || 
                    (start > end && (start <= self.position() || self.position() < end))
            }
        )
    }

    pub fn new<T>(length: usize, tempo: BPM, beats: T) -> Self 
    where T: IntoIterator<Item=usize> {
        Rhythm{
            length, 
            duration: beat_length(tempo),
            beats: beats.into_iter().collect(),
            time: 0.0
        }
    }
}

#[test]
fn test_rhythm(){
    let mut r = Rhythm::new(2, 120.0, vec![1]);
    assert_eq!(r.on(), false);
    r.update(0.15);
    assert_eq!(r.on(), false);
    assert!((r.time - 0.15).abs() < 1e-6);
    r.update(r.duration);
    assert!(r.on());
    assert!((1.15 - r.position()) < 1e-6);
    r.update(r.duration);
    assert_eq!(r.on(), false);
    assert!((r.time - 0.15).abs() < 1e-6);
}

#[test]
fn rhythm_window(){
    let mut tr = Rhythm::new(2,120., vec![0]);
    assert!((tr.duration-0.5) < 1e-6);
    assert!(tr.in_window(1e-6));
    tr.time = 0.04;
    assert!(tr.in_window(0.05));
    tr.time = 0.54;
    assert!(!tr.in_window(0.05));
    tr.time = 0.96;
    assert!(tr.in_window(0.05));


    let mut tr = Rhythm::new(4, 120., vec![0,2]);
    tr.update(0.04);
    assert!(tr.in_window(0.05));
    tr.update(0.5 );
    assert!(!tr.in_window(0.05));
    tr.update(0.5 );
    assert!(tr.in_window(0.05));
    tr.update(0.5 );
    assert!(!tr.in_window(0.05));

}
