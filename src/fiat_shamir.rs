//! Transcript-cost accounting. No verifier challenge is a proof message.
//!
//! The ledger records the actual absorb/read granularity of Spongefish's
//! BLAKE3 XOF, including repeated compressions for short OutputReader reads.
//! Fixed protocol/session identifiers are cached public parameters; their
//! absorption is included, their one-time SHA3 construction is not per-proof.
use crate::Fr;
use serde::Serialize;
use whir_ref::{algebra::embedding::Identity, protocols::whir::Config};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Event {
    pub operation: char,
    pub bytes: usize,
    pub count: usize,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Ledger {
    pub events: Vec<Event>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    pub finalizations: usize,
    pub absorb_bytes: usize,
    pub squeeze_bytes: usize,
    pub squeeze_calls: usize,
    pub compression_calls: usize,
}
impl Ledger {
    pub fn push(&mut self, operation: char, bytes: usize, count: usize) {
        if count == 0 {
            return;
        }
        if let Some(e) = self
            .events
            .last_mut()
            .filter(|e| e.operation == operation && e.bytes == bytes)
        {
            e.count += count;
        } else {
            self.events.push(Event {
                operation,
                bytes,
                count,
            });
        }
    }
    pub fn absorb(&mut self, bytes: usize) {
        self.push('a', bytes, 1);
    }
    pub fn fields(&mut self, count: usize) {
        self.push('a', 32, count);
    }
    pub fn challenges(&mut self, count: usize) {
        self.push('s', 32, count);
    }
    pub fn geometric(&mut self, count: usize) {
        if count > 1 {
            self.challenges(1);
        }
    }
    pub fn indices(&mut self, leaves: usize, draws: usize) {
        if leaves > 1 {
            self.push('s', 1, (leaves.ilog2() as usize).div_ceil(8) * draws);
        }
    }
    pub fn domain() -> Self {
        let mut l = Self::default();
        l.absorb(64);
        l.absorb(32);
        l.absorb(0);
        l
    }
    pub fn sumcheck(&mut self, rounds: usize, coefficients: usize) {
        for _ in 0..rounds {
            self.fields(coefficients);
            self.challenges(1);
        }
    }
    pub fn counts(&self) -> Counts {
        let mut c = Counts::default();
        let mut position = None;
        for e in &self.events {
            if e.operation == 'a' {
                // All production absorption calls are <=64 bytes. The
                // prefix formula models incremental chunk/CV-stack reuse.
                assert!(e.bytes <= 64);
                let before = update_compressions(c.absorb_bytes);
                c.absorb_bytes += e.bytes * e.count;
                c.compression_calls += update_compressions(c.absorb_bytes) - before;
                position = None;
            } else {
                assert_eq!(e.operation, 's');
                for _ in 0..e.count {
                    let p = position.get_or_insert_with(|| {
                        c.finalizations += 1;
                        c.compression_calls += c
                            .absorb_bytes
                            .saturating_sub(1)
                            .div_euclid(1024)
                            .count_ones() as usize;
                        0usize
                    });
                    if e.bytes > 0 {
                        c.compression_calls += (*p % 64 + e.bytes).div_ceil(64);
                    }
                    *p += e.bytes;
                    c.squeeze_bytes += e.bytes;
                    c.squeeze_calls += 1;
                }
            }
        }
        c
    }
    /// Normalize adjacent absorption of field-sized messages for comparison.
    pub fn normalized(&self) -> Vec<(char, usize)> {
        let mut out: Vec<(char, usize)> = Vec::new();
        for e in &self.events {
            if e.operation == 'a' {
                if let Some(x) = out.last_mut().filter(|x| x.0 == 'a') {
                    x.1 += e.bytes * e.count;
                } else {
                    out.push(('a', e.bytes * e.count));
                }
            } else {
                for _ in 0..e.count {
                    out.push(('s', e.bytes));
                }
            }
        }
        out
    }
}
fn update_compressions(bytes: usize) -> usize {
    if bytes == 0 {
        return 0;
    }
    let complete = (bytes - 1) / 1024;
    17 * complete - complete.count_ones() as usize + ((bytes - 1) % 1024) / 64
}
/// Full 32-byte digest, including BLAKE3's internal chunk tree.
pub fn hash_compressions(bytes: usize) -> usize {
    bytes.max(1).div_ceil(64) + bytes.max(1).div_ceil(1024) - 1
}

pub fn native_commit(l: &mut Ledger, cfg: &Config<Identity<Fr>>) {
    l.absorb(32);
    l.challenges(cfg.initial_out_domain_samples);
    l.fields(cfg.initial_out_domain_samples);
}
pub fn native_verify(l: &mut Ledger, cfg: &Config<Identity<Fr>>, sources: usize, forms: usize) {
    l.fields(sources * (sources - 1) * cfg.initial_out_domain_samples);
    l.geometric(sources);
    let constraints = sources * cfg.initial_out_domain_samples + forms;
    l.geometric(constraints);
    if constraints == 0 {
        l.challenges(cfg.initial_sumcheck.num_rounds());
    } else {
        l.sumcheck(cfg.initial_sumcheck.num_rounds(), 2);
    }
    let mut previous = &cfg.initial_committer;
    for r in &cfg.round_configs {
        l.absorb(32);
        l.challenges(r.out_domain_samples);
        l.fields(r.out_domain_samples);
        l.indices(previous.codeword_length(), previous.in_domain_samples());
        l.geometric(r.out_domain_samples + previous.in_domain_samples());
        l.sumcheck(r.sumcheck.num_rounds(), 2);
        previous = &r.irs_committer;
    }
    l.fields(cfg.final_sumcheck.initial_size());
    l.indices(previous.codeword_length(), previous.in_domain_samples());
    l.sumcheck(cfg.final_sumcheck.num_rounds(), 2);
}

#[cfg(feature = "fs-accounting")]
mod recording {
    use super::*;
    use spongefish::DuplexSpongeInterface;
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex},
    };
    thread_local! {static ACTIVE:RefCell<Option<Vec<Arc<Mutex<Ledger>>>>>=const {RefCell::new(None)};}
    #[derive(Clone)]
    pub struct RecordingBlake3 {
        inner: spongefish::instantiations::XOF<blake3::Hasher>,
        ledger: Option<Arc<Mutex<Ledger>>>,
    }
    impl Default for RecordingBlake3 {
        fn default() -> Self {
            let ledger = ACTIVE.with(|a| {
                a.borrow_mut().as_mut().map(|all| {
                    let l = Arc::new(Mutex::new(Ledger::default()));
                    all.push(l.clone());
                    l
                })
            });
            Self {
                inner: Default::default(),
                ledger,
            }
        }
    }
    impl DuplexSpongeInterface for RecordingBlake3 {
        type U = u8;
        fn absorb(&mut self, input: &[u8]) -> &mut Self {
            if let Some(l) = &self.ledger {
                l.lock().unwrap().push('a', input.len(), 1);
            }
            self.inner.absorb(input);
            self
        }
        fn squeeze(&mut self, output: &mut [u8]) -> &mut Self {
            if let Some(l) = &self.ledger {
                l.lock().unwrap().push('s', output.len(), 1);
            }
            self.inner.squeeze(output);
            self
        }
        fn ratchet(&mut self) -> &mut Self {
            self.inner.ratchet();
            self
        }
    }
    pub fn capture<T>(f: impl FnOnce() -> T) -> (T, Vec<Ledger>) {
        ACTIVE.with(|a| {
            assert!(a.borrow().is_none());
            *a.borrow_mut() = Some(Vec::new());
        });
        let value = f();
        let ledgers = ACTIVE.with(|a| a.borrow_mut().take().unwrap());
        (
            value,
            ledgers
                .into_iter()
                .map(|l| l.lock().unwrap().clone())
                .collect(),
        )
    }
}
#[cfg(feature = "fs-accounting")]
pub use recording::{capture, RecordingBlake3};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn streaming_prefix_cost_and_short_xof_reads() {
        for bytes in [0, 1, 32, 64, 65, 1024, 1025, 2048, 4096, 4097, 32000] {
            let mut l = Ledger::default();
            l.push('a', 32, bytes / 32);
            if bytes % 32 != 0 {
                l.absorb(bytes % 32);
            }
            l.push('s', 32, 1);
            assert_eq!(l.counts().compression_calls, hash_compressions(bytes));
        }
        let mut l = Ledger::default();
        l.absorb(32);
        l.push('s', 1, 64);
        assert_eq!(l.counts().finalizations, 1);
        assert_eq!(l.counts().compression_calls, 64); // OutputReader does not cache.
        l.absorb(32);
        l.challenges(1);
        assert_eq!(l.counts().finalizations, 2);
        assert_eq!(l.counts().compression_calls, 65);
    }
    #[cfg(feature = "fs-accounting")]
    #[test]
    fn recording_preserves_xof_bytes() {
        use spongefish::DuplexSpongeInterface;
        let (_, traces) = capture(|| {
            let mut a = RecordingBlake3::default();
            let mut b = spongefish::instantiations::XOF::<blake3::Hasher>::default();
            for i in 0..90u8 {
                a.absorb(&[i; 32]);
                b.absorb(&[i; 32]);
                for size in [1, 32, 64, 67] {
                    let mut x = vec![0; size];
                    let mut y = x.clone();
                    a.squeeze(&mut x);
                    b.squeeze(&mut y);
                    assert_eq!(x, y);
                }
            }
        });
        assert_eq!(traces[0].counts().finalizations, 90);
    }
}
