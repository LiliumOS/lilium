use core::cell::{LazyCell, UnsafeCell};

use lccc_siphash::{SipHashState, SipHasher};

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "rdrand")]
unsafe fn init_rand_state() -> ([u64; 2], [u64; 8]) {
    let mut keys = [0; 2];
    let mut arr = [0; 8];
    if crate::is_x86_feature_detected!("rdseed") {
        for arr in keys.iter_mut().chain(&mut arr) {
            while unsafe { core::arch::x86_64::_rdseed64_step(arr) } == 0 {}
        }
    } else {
        for arr in keys.iter_mut().chain(&mut arr) {
            while unsafe { core::arch::x86_64::_rdrand64_step(arr) } == 0 {}
        }
    }

    (keys, arr)
}

#[thread_local]
static TLS_RAND_STATE: LazyCell<UnsafeCell<(SipHashState, [u64; 8])>> = LazyCell::new(|| {
    let ([k0, k1], state) = unsafe { init_rand_state() };
    let siphash = SipHashState::from_keys(k0, k1);

    UnsafeCell::new((siphash, state))
});

pub fn rand_gen() -> u64 {
    let (state, arr) = unsafe { &mut *TLS_RAND_STATE.get() };

    for k in 0..8 {
        let key = arr[(k + 1) & 7];
        state.update_and_round::<2>(key);
        arr[k] = arr[k].wrapping_add(state.update_and_final::<2>());
    }
    state.update_and_round::<2>(0xDEADBEEF);
    state.update_and_final::<2>()
}
