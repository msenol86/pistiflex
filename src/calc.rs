pub fn dvt(distance: f64, time: f64) -> f64 {
    let v = distance as f64 / time as f64;
    return v;
}

pub fn series_xy(startx: i32, endx: i32, starty: i32, endy: i32, time: f64) -> (Vec<i32>, Vec<i32>) {
    let lenx = (endx - startx).abs();
    let leny = (endy - starty).abs();
    let signx = if endx > startx { 1.0 } else { -1.0 };
    let signy = if endy > starty { 1.0 } else { -1.0 };
    let vx = dvt(lenx as f64, time);
    let vy = dvt(leny as f64, time);
    let incx = vx * signx;
    let incy = vy * signy;
    let mut tmp_series_x = Vec::new();
    let mut tmp_series_y = Vec::new();
    let mut next_x = startx as f64 + incx;
    let mut next_y = starty as f64 + incy;
    for _ in 0..time.floor() as usize {
        tmp_series_x.push(next_x as i32);
        tmp_series_y.push(next_y as i32);
        next_x += incx;
        next_y += incy;
    }
    tmp_series_x[(time.floor() as usize - 1) as usize] = endx;
    tmp_series_y[(time.floor() as usize - 1) as usize] = endy;
    return (tmp_series_x, tmp_series_y);
}
