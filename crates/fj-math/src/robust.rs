//! Robust geometric primitives
//!
//! This is an implementation of the geometric primitives that are required by
//! Fornjot and not already provided by [`robust`]. They are auto-transpiled and
//! then manually modified.
//!
//! Original source (public domain):
//! <http://www.cs.cmu.edu/~quake/robust.html>
//!
//! The following tools were used for translation to Rust:
//!
//! - Clang: <https://clang.llvm.org/>
//! - Bear: <https://github.com/rizsotto/Bear>
//! - C2Rust: <https://c2rust.com/>
//!
//! The following steps are used to translate original C code to Rust:
//!
//! 1. Prepare a C file that contains the functions you want to translate.
//! 2. Create compile commands file: `bear -- clang -c predicates.c`
//! 3. Transpile C code to Rust: `c2rust transpile compile_commands.json`
//! 4. Copy code from transpiled file here.

#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::zero_ptr)]

const SPLITTER: f64 = 134217729.0;
const RESULTERRBOUND: f64 = 3.3306690738754706e-16;
const O3DERRBOUNDA: f64 = 7.771561172376103e-16;
const O3DERRBOUNDB: f64 = 3.330669073875473e-16;
const O3DERRBOUNDC: f64 = 3.2047474274603644e-31;

/// Test a point's orientation against a plane
pub unsafe extern "C" fn orient3d(
    pa: *mut f64,
    pb: *mut f64,
    pc: *mut f64,
    pd: *mut f64,
) -> f64 {
    let adx: f64 = *pa.offset(0) - *pd.offset(0);
    let bdx: f64 = *pb.offset(0) - *pd.offset(0);
    let cdx: f64 = *pc.offset(0) - *pd.offset(0);
    let ady: f64 = *pa.offset(1) - *pd.offset(1);
    let bdy: f64 = *pb.offset(1) - *pd.offset(1);
    let cdy: f64 = *pc.offset(1) - *pd.offset(1);
    let adz: f64 = *pa.offset(2) - *pd.offset(2);
    let bdz: f64 = *pb.offset(2) - *pd.offset(2);
    let cdz: f64 = *pc.offset(2) - *pd.offset(2);
    let bdxcdy: f64 = bdx * cdy;
    let cdxbdy: f64 = cdx * bdy;
    let cdxady: f64 = cdx * ady;
    let adxcdy: f64 = adx * cdy;
    let adxbdy: f64 = adx * bdy;
    let bdxady: f64 = bdx * ady;
    let det: f64 = adz * (bdxcdy - cdxbdy)
        + bdz * (cdxady - adxcdy)
        + cdz * (adxbdy - bdxady);
    let permanent: f64 = ((if bdxcdy >= 0.0f64 { bdxcdy } else { -bdxcdy })
        + (if cdxbdy >= 0.0f64 { cdxbdy } else { -cdxbdy }))
        * (if adz >= 0.0f64 { adz } else { -adz })
        + ((if cdxady >= 0.0f64 { cdxady } else { -cdxady })
            + (if adxcdy >= 0.0f64 { adxcdy } else { -adxcdy }))
            * (if bdz >= 0.0f64 { bdz } else { -bdz })
        + ((if adxbdy >= 0.0f64 { adxbdy } else { -adxbdy })
            + (if bdxady >= 0.0f64 { bdxady } else { -bdxady }))
            * (if cdz >= 0.0f64 { cdz } else { -cdz });
    let errbound: f64 = O3DERRBOUNDA * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    orient3dadapt(pa, pb, pc, pd, permanent)
}

unsafe extern "C" fn orient3dadapt(
    pa: *mut f64,
    pb: *mut f64,
    pc: *mut f64,
    pd: *mut f64,
    permanent: f64,
) -> f64 {
    let mut det: f64;
    let mut errbound: f64;
    let mut bc: [f64; 4] = [0.; 4];
    let mut ca: [f64; 4] = [0.; 4];
    let mut ab: [f64; 4] = [0.; 4];
    let mut adet: [f64; 8] = [0.; 8];
    let mut bdet: [f64; 8] = [0.; 8];
    let mut cdet: [f64; 8] = [0.; 8];
    let mut abdet: [f64; 16] = [0.; 16];
    let mut finnow: *mut f64;
    let mut finother: *mut f64;
    let mut finswap: *mut f64;
    let mut fin1: [f64; 192] = [0.; 192];
    let mut fin2: [f64; 192] = [0.; 192];
    let mut finlength: i32;
    let at_blarge: f64;
    let at_clarge: f64;
    let bt_clarge: f64;
    let bt_alarge: f64;
    let ct_alarge: f64;
    let ct_blarge: f64;
    let mut at_b: [f64; 4] = [0.; 4];
    let mut at_c: [f64; 4] = [0.; 4];
    let mut bt_c: [f64; 4] = [0.; 4];
    let mut bt_a: [f64; 4] = [0.; 4];
    let mut ct_a: [f64; 4] = [0.; 4];
    let mut ct_b: [f64; 4] = [0.; 4];
    let at_blen: i32;
    let at_clen: i32;
    let bt_clen: i32;
    let bt_alen: i32;
    let ct_alen: i32;
    let ct_blen: i32;
    let bdxt_cdy1: f64;
    let cdxt_bdy1: f64;
    let cdxt_ady1: f64;
    let adxt_cdy1: f64;
    let adxt_bdy1: f64;
    let bdxt_ady1: f64;
    let bdxt_cdy0: f64;
    let cdxt_bdy0: f64;
    let cdxt_ady0: f64;
    let adxt_cdy0: f64;
    let adxt_bdy0: f64;
    let bdxt_ady0: f64;
    let bdyt_cdx1: f64;
    let cdyt_bdx1: f64;
    let cdyt_adx1: f64;
    let adyt_cdx1: f64;
    let adyt_bdx1: f64;
    let bdyt_adx1: f64;
    let bdyt_cdx0: f64;
    let cdyt_bdx0: f64;
    let cdyt_adx0: f64;
    let adyt_cdx0: f64;
    let adyt_bdx0: f64;
    let bdyt_adx0: f64;
    let mut bct: [f64; 8] = [0.; 8];
    let mut cat: [f64; 8] = [0.; 8];
    let mut abt: [f64; 8] = [0.; 8];
    let bdxt_cdyt1: f64;
    let cdxt_bdyt1: f64;
    let cdxt_adyt1: f64;
    let adxt_cdyt1: f64;
    let adxt_bdyt1: f64;
    let bdxt_adyt1: f64;
    let bdxt_cdyt0: f64;
    let cdxt_bdyt0: f64;
    let cdxt_adyt0: f64;
    let adxt_cdyt0: f64;
    let adxt_bdyt0: f64;
    let bdxt_adyt0: f64;
    let mut u: [f64; 4] = [0.; 4];
    let mut v: [f64; 12] = [0.; 12];
    let mut w: [f64; 16] = [0.; 16];
    let mut u3: f64;
    let mut vlength: i32;
    let mut wlength: i32;
    let mut negate: f64;
    let mut bvirt: f64;
    let mut avirt: f64;
    let mut bround: f64;
    let mut around: f64;
    let mut c: f64;
    let mut abig: f64;
    let mut ahi: f64;
    let mut alo: f64;
    let mut bhi: f64;
    let mut blo: f64;
    let mut err1: f64;
    let mut err2: f64;
    let mut err3: f64;
    let mut _i: f64 = 0.;
    let mut _j: f64 = 0.;
    let mut _k: f64 = 0.;
    let mut _0: f64 = 0.;
    let adx: f64 = *pa.offset(0) - *pd.offset(0);
    let bdx: f64 = *pb.offset(0) - *pd.offset(0);
    let cdx: f64 = *pc.offset(0) - *pd.offset(0);
    let ady: f64 = *pa.offset(1) - *pd.offset(1);
    let bdy: f64 = *pb.offset(1) - *pd.offset(1);
    let cdy: f64 = *pc.offset(1) - *pd.offset(1);
    let adz: f64 = *pa.offset(2) - *pd.offset(2);
    let bdz: f64 = *pb.offset(2) - *pd.offset(2);
    let cdz: f64 = *pc.offset(2) - *pd.offset(2);
    let bdxcdy1: f64 = bdx * cdy;
    c = SPLITTER * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = SPLITTER * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = bdxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let bdxcdy0: f64 = alo * blo - err3;
    let cdxbdy1: f64 = cdx * bdy;
    c = SPLITTER * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = SPLITTER * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = cdxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let cdxbdy0: f64 = alo * blo - err3;
    _i = bdxcdy0 - cdxbdy0;
    bvirt = bdxcdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cdxbdy0;
    around = bdxcdy0 - avirt;
    bc[0] = around + bround;
    _j = bdxcdy1 + _i;
    bvirt = _j - bdxcdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = bdxcdy1 - avirt;
    _0 = around + bround;
    _i = _0 - cdxbdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cdxbdy1;
    around = _0 - avirt;
    bc[1] = around + bround;
    let bc3: f64 = _j + _i;
    bvirt = bc3 - _j;
    avirt = bc3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2] = around + bround;
    bc[3] = bc3;
    let alen: i32 =
        scale_expansion_zeroelim(4, bc.as_mut_ptr(), adz, adet.as_mut_ptr());
    let cdxady1: f64 = cdx * ady;
    c = SPLITTER * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = SPLITTER * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = cdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let cdxady0: f64 = alo * blo - err3;
    let adxcdy1: f64 = adx * cdy;
    c = SPLITTER * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = SPLITTER * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = adxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let adxcdy0: f64 = alo * blo - err3;
    _i = cdxady0 - adxcdy0;
    bvirt = cdxady0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - adxcdy0;
    around = cdxady0 - avirt;
    ca[0] = around + bround;
    _j = cdxady1 + _i;
    bvirt = _j - cdxady1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = cdxady1 - avirt;
    _0 = around + bround;
    _i = _0 - adxcdy1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - adxcdy1;
    around = _0 - avirt;
    ca[1] = around + bround;
    let ca3: f64 = _j + _i;
    bvirt = ca3 - _j;
    avirt = ca3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ca[2] = around + bround;
    ca[3] = ca3;
    let blen: i32 =
        scale_expansion_zeroelim(4, ca.as_mut_ptr(), bdz, bdet.as_mut_ptr());
    let adxbdy1: f64 = adx * bdy;
    c = SPLITTER * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = SPLITTER * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = adxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let adxbdy0: f64 = alo * blo - err3;
    let bdxady1: f64 = bdx * ady;
    c = SPLITTER * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = SPLITTER * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = bdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let bdxady0: f64 = alo * blo - err3;
    _i = adxbdy0 - bdxady0;
    bvirt = adxbdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bdxady0;
    around = adxbdy0 - avirt;
    ab[0] = around + bround;
    _j = adxbdy1 + _i;
    bvirt = _j - adxbdy1;
    avirt = _j - bvirt;
    bround = _i - bvirt;
    around = adxbdy1 - avirt;
    _0 = around + bround;
    _i = _0 - bdxady1;
    bvirt = _0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bdxady1;
    around = _0 - avirt;
    ab[1] = around + bround;
    let ab3: f64 = _j + _i;
    bvirt = ab3 - _j;
    avirt = ab3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2] = around + bround;
    ab[3] = ab3;
    let clen: i32 =
        scale_expansion_zeroelim(4, ab.as_mut_ptr(), cdz, cdet.as_mut_ptr());
    let ablen: i32 = fast_expansion_sum_zeroelim(
        alen,
        adet.as_mut_ptr(),
        blen,
        bdet.as_mut_ptr(),
        abdet.as_mut_ptr(),
    );
    finlength = fast_expansion_sum_zeroelim(
        ablen,
        abdet.as_mut_ptr(),
        clen,
        cdet.as_mut_ptr(),
        fin1.as_mut_ptr(),
    );
    det = estimate(finlength, fin1.as_mut_ptr());
    errbound = O3DERRBOUNDB * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }
    bvirt = *pa.offset(0) - adx;
    avirt = adx + bvirt;
    bround = bvirt - *pd.offset(0);
    around = *pa.offset(0) - avirt;
    let adxtail: f64 = around + bround;
    bvirt = *pb.offset(0) - bdx;
    avirt = bdx + bvirt;
    bround = bvirt - *pd.offset(0);
    around = *pb.offset(0) - avirt;
    let bdxtail: f64 = around + bround;
    bvirt = *pc.offset(0) - cdx;
    avirt = cdx + bvirt;
    bround = bvirt - *pd.offset(0);
    around = *pc.offset(0) - avirt;
    let cdxtail: f64 = around + bround;
    bvirt = *pa.offset(1) - ady;
    avirt = ady + bvirt;
    bround = bvirt - *pd.offset(1);
    around = *pa.offset(1) - avirt;
    let adytail: f64 = around + bround;
    bvirt = *pb.offset(1) - bdy;
    avirt = bdy + bvirt;
    bround = bvirt - *pd.offset(1);
    around = *pb.offset(1) - avirt;
    let bdytail: f64 = around + bround;
    bvirt = *pc.offset(1) - cdy;
    avirt = cdy + bvirt;
    bround = bvirt - *pd.offset(1);
    around = *pc.offset(1) - avirt;
    let cdytail: f64 = around + bround;
    bvirt = *pa.offset(2) - adz;
    avirt = adz + bvirt;
    bround = bvirt - *pd.offset(2);
    around = *pa.offset(2) - avirt;
    let adztail: f64 = around + bround;
    bvirt = *pb.offset(2) - bdz;
    avirt = bdz + bvirt;
    bround = bvirt - *pd.offset(2);
    around = *pb.offset(2) - avirt;
    let bdztail: f64 = around + bround;
    bvirt = *pc.offset(2) - cdz;
    avirt = cdz + bvirt;
    bround = bvirt - *pd.offset(2);
    around = *pc.offset(2) - avirt;
    let cdztail: f64 = around + bround;
    if adxtail == 0.0f64
        && bdxtail == 0.0f64
        && cdxtail == 0.0f64
        && adytail == 0.0f64
        && bdytail == 0.0f64
        && cdytail == 0.0f64
        && adztail == 0.0f64
        && bdztail == 0.0f64
        && cdztail == 0.0f64
    {
        return det;
    }
    errbound = O3DERRBOUNDC * permanent
        + RESULTERRBOUND * (if det >= 0.0f64 { det } else { -det });
    det += adz
        * (bdx * cdytail + cdy * bdxtail - (bdy * cdxtail + cdx * bdytail))
        + adztail * (bdx * cdy - bdy * cdx)
        + (bdz
            * (cdx * adytail + ady * cdxtail
                - (cdy * adxtail + adx * cdytail))
            + bdztail * (cdx * ady - cdy * adx))
        + (cdz
            * (adx * bdytail + bdy * adxtail
                - (ady * bdxtail + bdx * adytail))
            + cdztail * (adx * bdy - ady * bdx));
    if det >= errbound || -det >= errbound {
        return det;
    }
    finnow = fin1.as_mut_ptr();
    finother = fin2.as_mut_ptr();
    if adxtail == 0.0f64 {
        if adytail == 0.0f64 {
            at_b[0] = 0.0f64;
            at_blen = 1;
            at_c[0] = 0.0f64;
            at_clen = 1;
        } else {
            negate = -adytail;
            at_blarge = negate * bdx;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = at_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_b[0] = alo * blo - err3;
            at_b[1] = at_blarge;
            at_blen = 2;
            at_clarge = adytail * cdx;
            c = SPLITTER * adytail;
            abig = c - adytail;
            ahi = c - abig;
            alo = adytail - ahi;
            c = SPLITTER * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = at_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_c[0] = alo * blo - err3;
            at_c[1] = at_clarge;
            at_clen = 2;
        }
    } else if adytail == 0.0f64 {
        at_blarge = adxtail * bdy;
        c = SPLITTER * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = at_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_b[0] = alo * blo - err3;
        at_b[1] = at_blarge;
        at_blen = 2;
        negate = -adxtail;
        at_clarge = negate * cdy;
        c = SPLITTER * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = at_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_c[0] = alo * blo - err3;
        at_c[1] = at_clarge;
        at_clen = 2;
    } else {
        adxt_bdy1 = adxtail * bdy;
        c = SPLITTER * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = adxt_bdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adxt_bdy0 = alo * blo - err3;
        adyt_bdx1 = adytail * bdx;
        c = SPLITTER * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = SPLITTER * bdx;
        abig = c - bdx;
        bhi = c - abig;
        blo = bdx - bhi;
        err1 = adyt_bdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adyt_bdx0 = alo * blo - err3;
        _i = adxt_bdy0 - adyt_bdx0;
        bvirt = adxt_bdy0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adyt_bdx0;
        around = adxt_bdy0 - avirt;
        at_b[0] = around + bround;
        _j = adxt_bdy1 + _i;
        bvirt = _j - adxt_bdy1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = adxt_bdy1 - avirt;
        _0 = around + bround;
        _i = _0 - adyt_bdx1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adyt_bdx1;
        around = _0 - avirt;
        at_b[1] = around + bround;
        at_blarge = _j + _i;
        bvirt = at_blarge - _j;
        avirt = at_blarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        at_b[2] = around + bround;
        at_b[3] = at_blarge;
        at_blen = 4;
        adyt_cdx1 = adytail * cdx;
        c = SPLITTER * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = SPLITTER * cdx;
        abig = c - cdx;
        bhi = c - abig;
        blo = cdx - bhi;
        err1 = adyt_cdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adyt_cdx0 = alo * blo - err3;
        adxt_cdy1 = adxtail * cdy;
        c = SPLITTER * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = adxt_cdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adxt_cdy0 = alo * blo - err3;
        _i = adyt_cdx0 - adxt_cdy0;
        bvirt = adyt_cdx0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adxt_cdy0;
        around = adyt_cdx0 - avirt;
        at_c[0] = around + bround;
        _j = adyt_cdx1 + _i;
        bvirt = _j - adyt_cdx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = adyt_cdx1 - avirt;
        _0 = around + bround;
        _i = _0 - adxt_cdy1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - adxt_cdy1;
        around = _0 - avirt;
        at_c[1] = around + bround;
        at_clarge = _j + _i;
        bvirt = at_clarge - _j;
        avirt = at_clarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        at_c[2] = around + bround;
        at_c[3] = at_clarge;
        at_clen = 4;
    }
    if bdxtail == 0.0f64 {
        if bdytail == 0.0f64 {
            bt_c[0] = 0.0f64;
            bt_clen = 1;
            bt_a[0] = 0.0f64;
            bt_alen = 1;
        } else {
            negate = -bdytail;
            bt_clarge = negate * cdx;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = bt_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_c[0] = alo * blo - err3;
            bt_c[1] = bt_clarge;
            bt_clen = 2;
            bt_alarge = bdytail * adx;
            c = SPLITTER * bdytail;
            abig = c - bdytail;
            ahi = c - abig;
            alo = bdytail - ahi;
            c = SPLITTER * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = bt_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_a[0] = alo * blo - err3;
            bt_a[1] = bt_alarge;
            bt_alen = 2;
        }
    } else if bdytail == 0.0f64 {
        bt_clarge = bdxtail * cdy;
        c = SPLITTER * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bt_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_c[0] = alo * blo - err3;
        bt_c[1] = bt_clarge;
        bt_clen = 2;
        negate = -bdxtail;
        bt_alarge = negate * ady;
        c = SPLITTER * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = bt_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_a[0] = alo * blo - err3;
        bt_a[1] = bt_alarge;
        bt_alen = 2;
    } else {
        bdxt_cdy1 = bdxtail * cdy;
        c = SPLITTER * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bdxt_cdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdxt_cdy0 = alo * blo - err3;
        bdyt_cdx1 = bdytail * cdx;
        c = SPLITTER * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = SPLITTER * cdx;
        abig = c - cdx;
        bhi = c - abig;
        blo = cdx - bhi;
        err1 = bdyt_cdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdyt_cdx0 = alo * blo - err3;
        _i = bdxt_cdy0 - bdyt_cdx0;
        bvirt = bdxt_cdy0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdyt_cdx0;
        around = bdxt_cdy0 - avirt;
        bt_c[0] = around + bround;
        _j = bdxt_cdy1 + _i;
        bvirt = _j - bdxt_cdy1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = bdxt_cdy1 - avirt;
        _0 = around + bround;
        _i = _0 - bdyt_cdx1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdyt_cdx1;
        around = _0 - avirt;
        bt_c[1] = around + bround;
        bt_clarge = _j + _i;
        bvirt = bt_clarge - _j;
        avirt = bt_clarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        bt_c[2] = around + bround;
        bt_c[3] = bt_clarge;
        bt_clen = 4;
        bdyt_adx1 = bdytail * adx;
        c = SPLITTER * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = SPLITTER * adx;
        abig = c - adx;
        bhi = c - abig;
        blo = adx - bhi;
        err1 = bdyt_adx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdyt_adx0 = alo * blo - err3;
        bdxt_ady1 = bdxtail * ady;
        c = SPLITTER * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = bdxt_ady1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdxt_ady0 = alo * blo - err3;
        _i = bdyt_adx0 - bdxt_ady0;
        bvirt = bdyt_adx0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdxt_ady0;
        around = bdyt_adx0 - avirt;
        bt_a[0] = around + bround;
        _j = bdyt_adx1 + _i;
        bvirt = _j - bdyt_adx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = bdyt_adx1 - avirt;
        _0 = around + bround;
        _i = _0 - bdxt_ady1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - bdxt_ady1;
        around = _0 - avirt;
        bt_a[1] = around + bround;
        bt_alarge = _j + _i;
        bvirt = bt_alarge - _j;
        avirt = bt_alarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        bt_a[2] = around + bround;
        bt_a[3] = bt_alarge;
        bt_alen = 4;
    }
    if cdxtail == 0.0f64 {
        if cdytail == 0.0f64 {
            ct_a[0] = 0.0f64;
            ct_alen = 1;
            ct_b[0] = 0.0f64;
            ct_blen = 1;
        } else {
            negate = -cdytail;
            ct_alarge = negate * adx;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = ct_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_a[0] = alo * blo - err3;
            ct_a[1] = ct_alarge;
            ct_alen = 2;
            ct_blarge = cdytail * bdx;
            c = SPLITTER * cdytail;
            abig = c - cdytail;
            ahi = c - abig;
            alo = cdytail - ahi;
            c = SPLITTER * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = ct_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_b[0] = alo * blo - err3;
            ct_b[1] = ct_blarge;
            ct_blen = 2;
        }
    } else if cdytail == 0.0f64 {
        ct_alarge = cdxtail * ady;
        c = SPLITTER * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = ct_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_a[0] = alo * blo - err3;
        ct_a[1] = ct_alarge;
        ct_alen = 2;
        negate = -cdxtail;
        ct_blarge = negate * bdy;
        c = SPLITTER * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = ct_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_b[0] = alo * blo - err3;
        ct_b[1] = ct_blarge;
        ct_blen = 2;
    } else {
        cdxt_ady1 = cdxtail * ady;
        c = SPLITTER * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = cdxt_ady1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdxt_ady0 = alo * blo - err3;
        cdyt_adx1 = cdytail * adx;
        c = SPLITTER * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = SPLITTER * adx;
        abig = c - adx;
        bhi = c - abig;
        blo = adx - bhi;
        err1 = cdyt_adx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdyt_adx0 = alo * blo - err3;
        _i = cdxt_ady0 - cdyt_adx0;
        bvirt = cdxt_ady0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdyt_adx0;
        around = cdxt_ady0 - avirt;
        ct_a[0] = around + bround;
        _j = cdxt_ady1 + _i;
        bvirt = _j - cdxt_ady1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = cdxt_ady1 - avirt;
        _0 = around + bround;
        _i = _0 - cdyt_adx1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdyt_adx1;
        around = _0 - avirt;
        ct_a[1] = around + bround;
        ct_alarge = _j + _i;
        bvirt = ct_alarge - _j;
        avirt = ct_alarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        ct_a[2] = around + bround;
        ct_a[3] = ct_alarge;
        ct_alen = 4;
        cdyt_bdx1 = cdytail * bdx;
        c = SPLITTER * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = SPLITTER * bdx;
        abig = c - bdx;
        bhi = c - abig;
        blo = bdx - bhi;
        err1 = cdyt_bdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdyt_bdx0 = alo * blo - err3;
        cdxt_bdy1 = cdxtail * bdy;
        c = SPLITTER * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = cdxt_bdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdxt_bdy0 = alo * blo - err3;
        _i = cdyt_bdx0 - cdxt_bdy0;
        bvirt = cdyt_bdx0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdxt_bdy0;
        around = cdyt_bdx0 - avirt;
        ct_b[0] = around + bround;
        _j = cdyt_bdx1 + _i;
        bvirt = _j - cdyt_bdx1;
        avirt = _j - bvirt;
        bround = _i - bvirt;
        around = cdyt_bdx1 - avirt;
        _0 = around + bround;
        _i = _0 - cdxt_bdy1;
        bvirt = _0 - _i;
        avirt = _i + bvirt;
        bround = bvirt - cdxt_bdy1;
        around = _0 - avirt;
        ct_b[1] = around + bround;
        ct_blarge = _j + _i;
        bvirt = ct_blarge - _j;
        avirt = ct_blarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        ct_b[2] = around + bround;
        ct_b[3] = ct_blarge;
        ct_blen = 4;
    }
    let bctlen: i32 = fast_expansion_sum_zeroelim(
        bt_clen,
        bt_c.as_mut_ptr(),
        ct_blen,
        ct_b.as_mut_ptr(),
        bct.as_mut_ptr(),
    );
    wlength =
        scale_expansion_zeroelim(bctlen, bct.as_mut_ptr(), adz, w.as_mut_ptr());
    finlength = fast_expansion_sum_zeroelim(
        finlength,
        finnow,
        wlength,
        w.as_mut_ptr(),
        finother,
    );
    finswap = finnow;
    finnow = finother;
    finother = finswap;
    let catlen: i32 = fast_expansion_sum_zeroelim(
        ct_alen,
        ct_a.as_mut_ptr(),
        at_clen,
        at_c.as_mut_ptr(),
        cat.as_mut_ptr(),
    );
    wlength =
        scale_expansion_zeroelim(catlen, cat.as_mut_ptr(), bdz, w.as_mut_ptr());
    finlength = fast_expansion_sum_zeroelim(
        finlength,
        finnow,
        wlength,
        w.as_mut_ptr(),
        finother,
    );
    finswap = finnow;
    finnow = finother;
    finother = finswap;
    let abtlen: i32 = fast_expansion_sum_zeroelim(
        at_blen,
        at_b.as_mut_ptr(),
        bt_alen,
        bt_a.as_mut_ptr(),
        abt.as_mut_ptr(),
    );
    wlength =
        scale_expansion_zeroelim(abtlen, abt.as_mut_ptr(), cdz, w.as_mut_ptr());
    finlength = fast_expansion_sum_zeroelim(
        finlength,
        finnow,
        wlength,
        w.as_mut_ptr(),
        finother,
    );
    finswap = finnow;
    finnow = finother;
    finother = finswap;
    if adztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(
            4,
            bc.as_mut_ptr(),
            adztail,
            v.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            vlength,
            v.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if bdztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(
            4,
            ca.as_mut_ptr(),
            bdztail,
            v.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            vlength,
            v.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if cdztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(
            4,
            ab.as_mut_ptr(),
            cdztail,
            v.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            vlength,
            v.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if adxtail != 0.0f64 {
        if bdytail != 0.0f64 {
            adxt_bdyt1 = adxtail * bdytail;
            c = SPLITTER * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = SPLITTER * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = adxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_bdyt0 = alo * blo - err3;
            c = SPLITTER * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            _i = adxt_bdyt0 * cdz;
            c = SPLITTER * adxt_bdyt0;
            abig = c - adxt_bdyt0;
            ahi = c - abig;
            alo = adxt_bdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            _j = adxt_bdyt1 * cdz;
            c = SPLITTER * adxt_bdyt1;
            abig = c - adxt_bdyt1;
            ahi = c - abig;
            alo = adxt_bdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2] = _k - bvirt;
            u[3] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if cdztail != 0.0f64 {
                c = SPLITTER * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                _i = adxt_bdyt0 * cdztail;
                c = SPLITTER * adxt_bdyt0;
                abig = c - adxt_bdyt0;
                ahi = c - abig;
                alo = adxt_bdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                _j = adxt_bdyt1 * cdztail;
                c = SPLITTER * adxt_bdyt1;
                abig = c - adxt_bdyt1;
                ahi = c - abig;
                alo = adxt_bdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2] = _k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
        if cdytail != 0.0f64 {
            negate = -adxtail;
            adxt_cdyt1 = negate * cdytail;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = adxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_cdyt0 = alo * blo - err3;
            c = SPLITTER * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            _i = adxt_cdyt0 * bdz;
            c = SPLITTER * adxt_cdyt0;
            abig = c - adxt_cdyt0;
            ahi = c - abig;
            alo = adxt_cdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            _j = adxt_cdyt1 * bdz;
            c = SPLITTER * adxt_cdyt1;
            abig = c - adxt_cdyt1;
            ahi = c - abig;
            alo = adxt_cdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2] = _k - bvirt;
            u[3] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if bdztail != 0.0f64 {
                c = SPLITTER * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                _i = adxt_cdyt0 * bdztail;
                c = SPLITTER * adxt_cdyt0;
                abig = c - adxt_cdyt0;
                ahi = c - abig;
                alo = adxt_cdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                _j = adxt_cdyt1 * bdztail;
                c = SPLITTER * adxt_cdyt1;
                abig = c - adxt_cdyt1;
                ahi = c - abig;
                alo = adxt_cdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2] = _k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
    }
    if bdxtail != 0.0f64 {
        if cdytail != 0.0f64 {
            bdxt_cdyt1 = bdxtail * cdytail;
            c = SPLITTER * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = SPLITTER * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = bdxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_cdyt0 = alo * blo - err3;
            c = SPLITTER * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            _i = bdxt_cdyt0 * adz;
            c = SPLITTER * bdxt_cdyt0;
            abig = c - bdxt_cdyt0;
            ahi = c - abig;
            alo = bdxt_cdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            _j = bdxt_cdyt1 * adz;
            c = SPLITTER * bdxt_cdyt1;
            abig = c - bdxt_cdyt1;
            ahi = c - abig;
            alo = bdxt_cdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2] = _k - bvirt;
            u[3] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if adztail != 0.0f64 {
                c = SPLITTER * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                _i = bdxt_cdyt0 * adztail;
                c = SPLITTER * bdxt_cdyt0;
                abig = c - bdxt_cdyt0;
                ahi = c - abig;
                alo = bdxt_cdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                _j = bdxt_cdyt1 * adztail;
                c = SPLITTER * bdxt_cdyt1;
                abig = c - bdxt_cdyt1;
                ahi = c - abig;
                alo = bdxt_cdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2] = _k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
        if adytail != 0.0f64 {
            negate = -bdxtail;
            bdxt_adyt1 = negate * adytail;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = bdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_adyt0 = alo * blo - err3;
            c = SPLITTER * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            _i = bdxt_adyt0 * cdz;
            c = SPLITTER * bdxt_adyt0;
            abig = c - bdxt_adyt0;
            ahi = c - abig;
            alo = bdxt_adyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            _j = bdxt_adyt1 * cdz;
            c = SPLITTER * bdxt_adyt1;
            abig = c - bdxt_adyt1;
            ahi = c - abig;
            alo = bdxt_adyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2] = _k - bvirt;
            u[3] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if cdztail != 0.0f64 {
                c = SPLITTER * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                _i = bdxt_adyt0 * cdztail;
                c = SPLITTER * bdxt_adyt0;
                abig = c - bdxt_adyt0;
                ahi = c - abig;
                alo = bdxt_adyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                _j = bdxt_adyt1 * cdztail;
                c = SPLITTER * bdxt_adyt1;
                abig = c - bdxt_adyt1;
                ahi = c - abig;
                alo = bdxt_adyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2] = _k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
    }
    if cdxtail != 0.0f64 {
        if adytail != 0.0f64 {
            cdxt_adyt1 = cdxtail * adytail;
            c = SPLITTER * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = SPLITTER * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = cdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_adyt0 = alo * blo - err3;
            c = SPLITTER * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            _i = cdxt_adyt0 * bdz;
            c = SPLITTER * cdxt_adyt0;
            abig = c - cdxt_adyt0;
            ahi = c - abig;
            alo = cdxt_adyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            _j = cdxt_adyt1 * bdz;
            c = SPLITTER * cdxt_adyt1;
            abig = c - cdxt_adyt1;
            ahi = c - abig;
            alo = cdxt_adyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2] = _k - bvirt;
            u[3] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if bdztail != 0.0f64 {
                c = SPLITTER * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                _i = cdxt_adyt0 * bdztail;
                c = SPLITTER * cdxt_adyt0;
                abig = c - cdxt_adyt0;
                ahi = c - abig;
                alo = cdxt_adyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                _j = cdxt_adyt1 * bdztail;
                c = SPLITTER * cdxt_adyt1;
                abig = c - cdxt_adyt1;
                ahi = c - abig;
                alo = cdxt_adyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2] = _k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
        if bdytail != 0.0f64 {
            negate = -cdxtail;
            cdxt_bdyt1 = negate * bdytail;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = cdxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_bdyt0 = alo * blo - err3;
            c = SPLITTER * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            _i = cdxt_bdyt0 * adz;
            c = SPLITTER * cdxt_bdyt0;
            abig = c - cdxt_bdyt0;
            ahi = c - abig;
            alo = cdxt_bdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            _j = cdxt_bdyt1 * adz;
            c = SPLITTER * cdxt_bdyt1;
            abig = c - cdxt_bdyt1;
            ahi = c - abig;
            alo = cdxt_bdyt1 - ahi;
            err1 = _j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            _0 = alo * blo - err3;
            _k = _i + _0;
            bvirt = _k - _i;
            avirt = _k - bvirt;
            bround = _0 - bvirt;
            around = _i - avirt;
            u[1] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2] = _k - bvirt;
            u[3] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if adztail != 0.0f64 {
                c = SPLITTER * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                _i = cdxt_bdyt0 * adztail;
                c = SPLITTER * cdxt_bdyt0;
                abig = c - cdxt_bdyt0;
                ahi = c - abig;
                alo = cdxt_bdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                _j = cdxt_bdyt1 * adztail;
                c = SPLITTER * cdxt_bdyt1;
                abig = c - cdxt_bdyt1;
                ahi = c - abig;
                alo = cdxt_bdyt1 - ahi;
                err1 = _j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                _0 = alo * blo - err3;
                _k = _i + _0;
                bvirt = _k - _i;
                avirt = _k - bvirt;
                bround = _0 - bvirt;
                around = _i - avirt;
                u[1] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2] = _k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4,
                    u.as_mut_ptr(),
                    finother,
                );
                finswap = finnow;
                finnow = finother;
                finother = finswap;
            }
        }
    }
    if adztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(
            bctlen,
            bct.as_mut_ptr(),
            adztail,
            w.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            wlength,
            w.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if bdztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(
            catlen,
            cat.as_mut_ptr(),
            bdztail,
            w.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            wlength,
            w.as_mut_ptr(),
            finother,
        );
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    if cdztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(
            abtlen,
            abt.as_mut_ptr(),
            cdztail,
            w.as_mut_ptr(),
        );
        finlength = fast_expansion_sum_zeroelim(
            finlength,
            finnow,
            wlength,
            w.as_mut_ptr(),
            finother,
        );
        finnow = finother;
    }
    *finnow.offset((finlength - 1) as isize)
}

unsafe extern "C" fn scale_expansion_zeroelim(
    elen: i32,
    e: *mut f64,
    b: f64,
    h: *mut f64,
) -> i32 {
    let mut q: f64;
    let mut sum: f64;
    let mut hh: f64;
    let mut product1: f64;
    let mut product0: f64;
    let mut eindex: i32;
    let mut hindex: i32;
    let mut enow: f64;
    let mut bvirt: f64;
    let mut avirt: f64;
    let mut bround: f64;
    let mut around: f64;
    let mut c: f64;
    let mut abig: f64;
    let mut ahi: f64;
    let mut alo: f64;
    let mut err1: f64;
    let mut err2: f64;
    let mut err3: f64;
    c = SPLITTER * b;
    abig = c - b;
    let bhi: f64 = c - abig;
    let blo: f64 = b - bhi;
    q = *e.offset(0) * b;
    c = SPLITTER * *e.offset(0);
    abig = c - *e.offset(0);
    ahi = c - abig;
    alo = *e.offset(0) - ahi;
    err1 = q - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    hh = alo * blo - err3;
    hindex = 0;
    if hh != 0 as f64 {
        let fresh12 = hindex;
        hindex += 1;
        *h.offset(fresh12 as isize) = hh;
    }
    eindex = 1;
    while eindex < elen {
        enow = *e.offset(eindex as isize);
        product1 = enow * b;
        c = SPLITTER * enow;
        abig = c - enow;
        ahi = c - abig;
        alo = enow - ahi;
        err1 = product1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        product0 = alo * blo - err3;
        sum = q + product0;
        bvirt = sum - q;
        avirt = sum - bvirt;
        bround = product0 - bvirt;
        around = q - avirt;
        hh = around + bround;
        if hh != 0 as f64 {
            let fresh13 = hindex;
            hindex += 1;
            *h.offset(fresh13 as isize) = hh;
        }
        q = product1 + sum;
        bvirt = q - product1;
        hh = sum - bvirt;
        if hh != 0 as f64 {
            let fresh14 = hindex;
            hindex += 1;
            *h.offset(fresh14 as isize) = hh;
        }
        eindex += 1;
    }
    if q != 0.0f64 || hindex == 0 {
        let fresh15 = hindex;
        hindex += 1;
        *h.offset(fresh15 as isize) = q;
    }
    hindex
}

unsafe extern "C" fn fast_expansion_sum_zeroelim(
    elen: i32,
    e: *mut f64,
    flen: i32,
    f: *mut f64,
    h: *mut f64,
) -> i32 {
    let mut q: f64;
    let mut q_new: f64;
    let mut hh: f64;
    let mut bvirt: f64;
    let mut avirt: f64;
    let mut bround: f64;
    let mut around: f64;
    let mut eindex: i32;
    let mut findex: i32;
    let mut hindex: i32;
    let mut enow: f64;
    let mut fnow: f64;
    enow = *e.offset(0);
    fnow = *f.offset(0);
    findex = 0;
    eindex = findex;
    if (fnow > enow) as i32 == (fnow > -enow) as i32 {
        q = enow;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        q = fnow;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    hindex = 0;
    if eindex < elen && findex < flen {
        if (fnow > enow) as i32 == (fnow > -enow) as i32 {
            q_new = enow + q;
            bvirt = q_new - enow;
            hh = q - bvirt;
            eindex += 1;
            enow = *e.offset(eindex as isize);
        } else {
            q_new = fnow + q;
            bvirt = q_new - fnow;
            hh = q - bvirt;
            findex += 1;
            fnow = *f.offset(findex as isize);
        }
        q = q_new;
        if hh != 0.0f64 {
            let fresh4 = hindex;
            hindex += 1;
            *h.offset(fresh4 as isize) = hh;
        }
        while eindex < elen && findex < flen {
            if (fnow > enow) as i32 == (fnow > -enow) as i32 {
                q_new = q + enow;
                bvirt = q_new - q;
                avirt = q_new - bvirt;
                bround = enow - bvirt;
                around = q - avirt;
                hh = around + bround;
                eindex += 1;
                enow = *e.offset(eindex as isize);
            } else {
                q_new = q + fnow;
                bvirt = q_new - q;
                avirt = q_new - bvirt;
                bround = fnow - bvirt;
                around = q - avirt;
                hh = around + bround;
                findex += 1;
                fnow = *f.offset(findex as isize);
            }
            q = q_new;
            if hh != 0.0f64 {
                let fresh5 = hindex;
                hindex += 1;
                *h.offset(fresh5 as isize) = hh;
            }
        }
    }
    while eindex < elen {
        q_new = q + enow;
        bvirt = q_new - q;
        avirt = q_new - bvirt;
        bround = enow - bvirt;
        around = q - avirt;
        hh = around + bround;
        eindex += 1;
        enow = *e.offset(eindex as isize);
        q = q_new;
        if hh != 0.0f64 {
            let fresh6 = hindex;
            hindex += 1;
            *h.offset(fresh6 as isize) = hh;
        }
    }
    while findex < flen {
        q_new = q + fnow;
        bvirt = q_new - q;
        avirt = q_new - bvirt;
        bround = fnow - bvirt;
        around = q - avirt;
        hh = around + bround;
        findex += 1;
        fnow = *f.offset(findex as isize);
        q = q_new;
        if hh != 0.0f64 {
            let fresh7 = hindex;
            hindex += 1;
            *h.offset(fresh7 as isize) = hh;
        }
    }
    if q != 0.0f64 || hindex == 0 {
        let fresh8 = hindex;
        hindex += 1;
        *h.offset(fresh8 as isize) = q;
    }
    hindex
}

unsafe extern "C" fn estimate(elen: i32, e: *mut f64) -> f64 {
    let mut q: f64;
    let mut eindex: i32;
    q = *e.offset(0);
    eindex = 1;
    while eindex < elen {
        q += *e.offset(eindex as isize);
        eindex += 1;
    }
    q
}
