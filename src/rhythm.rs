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
        if self.beats.contains(&0) && (self.length as f64 - self.position()) < (window_size / self.duration) {
            return true
        } 

        self.beats.iter().any(
            |beat| ((*beat as f64) - self.position()).abs() < (window_size / self.duration)
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

    pub fn euclidean(measure_length: usize, n_beats: usize, tempo: BPM) -> Self {
        

        fn euclid(p: usize, n:usize) -> Vec<(usize,Vec<bool>)> {
            let a : Vec<bool> = vec![true];
            let n_a = p;
            let b : Vec<bool>= vec![false];
            let n_b = n-p;

            while n_b > 1{
                let mut  c = a.clone();
                c.append(&mut b.clone());

                let d = a.clone();
            
                if n_a > n_b {
                    let t = n_a;
                    let n_a = n_b;
                    let n_b = t - n_b;
                } else {
                    let t = n_b;
                    let n_a = n_a;
                    let n_b = t - n_a;
                    let d = b.clone();
                }

                let a = c;
                let b = d;
            }

            vec![(n_a,a),(n_b,b)]
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
    assert!(tr.in_window(1e-6));
    tr.update(0.04);
    assert!(tr.in_window(0.05));
    tr.update(0.5 );
    assert!(!tr.in_window(0.05));
    tr.update(0.5 );
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
