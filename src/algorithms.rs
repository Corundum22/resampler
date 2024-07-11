#[inline(always)]
pub fn piecewise_constant(start: f32, end: f32, t: f32) -> f32 {
    if t < 0.5 {
        start
    } else {
        end
    }
}

#[inline(always)]
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    (1.0 - t) * start + t * end
}

#[inline(always)]
pub fn tangentless_hermite(start: f32, end: f32, t: f32) -> f32 {
    let hermite = t * t * (3.0 - 2.0 * t);
    (1.0 - hermite) * start + hermite * end
}

#[inline(always)]
pub fn even_hermite(pos_0: f32, pos_1: f32, m_0: f32, m_1: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h00 = -h01 + 1.0;
    let h11 = t3 - t2;
    let h10 = t3 - 2.0 * t2 + t;
    return h00 * pos_0 + h10 * m_0 + h01 * pos_1 + h11 * m_1;
}
