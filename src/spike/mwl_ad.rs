use super::Spike;

pub fn placeholder() {
    let my_test_spike: Spike<u32, u32> = Spike {
        waveforms: vec![vec![0, 1, 2]],
        time: 1,
    };
    println!("hi. it's {:?}", my_test_spike);
}
