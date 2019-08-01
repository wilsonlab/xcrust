#[derive (Debug)]
pub struct PlotSpec {
    pub width  : u16,
    pub height : u16,
    pub v_low  : f32,
    pub v_high : f32,
}

pub const DEFAULT_PLOT : PlotSpec = PlotSpec {
    width:   16,
    height:  20,
    v_low:  -0.000_005,
    v_high:  0.000_005,
};


pub fn draw_spike(plot: PlotSpec, waveforms: &Vec<Vec<f32>>) {

    let n_channels = waveforms.len() as u16;
    let n_samps    = waveforms[0].len() as u16;

    // Total window width is the per-channel width, plus
    // one padding per channel, plus a final single padding
    let window_width  = (plot.width + 1) * n_channels + 1;
    let window_height = plot.height + 2;

    // Linear index for the r_th row and c_th column of matrix
    // stored in a flat array
    let row_col_ind = |(r,c) : (u16, u16)| ((r * window_width) + c) as usize;

    // Precompute this ratio once for v_row
    let v_range_inverse = 1.0/(plot.v_high - plot.v_low);

    // The row for a voltage is the bottom row, less a number
    // of rows proportional to (v - min)/(max - min) and the
    // window height
    let v_row =
        |v : f32| { plot.height as u16 -
                    ( (v - plot.v_low) *
                       v_range_inverse *
                       plot.height as f32) as u16 + 1};

    // The column (within that channel's window) for a voltage is
    // the index of its sample, times the ratio of window length
    // to waveform length
    let t_col = |t : u16| ( (t as f32 * plot.width as f32 /
                             n_samps as f32) as u16);

    // Initiate an empty character matrix
    let pixel_count = row_col_ind((plot.height+2, window_width)) as usize;
    let mut pixel = Vec::with_capacity( pixel_count );
    for _ in 0..pixel_count {
        pixel.push(' ');
    }; 

    // Draw the outer border
    for r in 1..window_height-1 {
        pixel[ row_col_ind((r,                0)) as usize ] = '║';
        pixel[ row_col_ind((r, window_width - 1)) as usize ] = '║';
    };
    for c in 1..window_width-1 {
        pixel[ row_col_ind((0,                 c)) as usize ] = '═';
        pixel[ row_col_ind((window_height - 1, c)) as usize ] = '═';
    };

    // Draw the outer border corners
    pixel[ row_col_ind((0,                               0)) ] = '╔';
    pixel[ row_col_ind((0,                  window_width-1)) ] = '╗';
    pixel[ row_col_ind((window_height-1,                 0)) ] = '╚';
    pixel[ row_col_ind((window_height - 1,window_width - 1)) ] = '╝';
    
    // Draw the diveders between channels
    for chan in 1..n_channels {
        let col = chan * (plot.width + 1);
        pixel [ row_col_ind((0,                 col)) ] = '╦';
        pixel [ row_col_ind((window_height - 1, col)) ] = '╩';
        for r in 1..window_height-1 {
            pixel[ row_col_ind((r, col)) ] = '║';
        };
    }

    // Draw the 0-voltage axis and the voltage traces
    for chan in 0..n_channels {
        let col0 = chan * (plot.width + 1)  + 1 ;
        for c in 0..plot.width {
            let row_0 = v_row(0.0);

            pixel[ row_col_ind((row_0, c + col0)) ] = '-';

        };
        for c in 0..n_samps {
            let row_v = v_row( waveforms[chan as usize][c as usize] );
            let col_v = t_col(c);
            pixel[ row_col_ind((row_v, col_v + col0)) ] = '*';
        };
    };

    // Print the pixel buffer out to the shell
    for r in 0..window_height {
        let i0 = row_col_ind((r,           0));
        let i1 = row_col_ind((r,window_width));
        let bytes : &[char] = &pixel[i0..i1];
        let s : String = bytes.into_iter().collect();
        println!("{}", s);
    };
    

}
