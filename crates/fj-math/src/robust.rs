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

#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_return)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::zero_ptr)]

static mut splitter: f64 = 134217729.0;
static mut resulterrbound: f64 = 3.3306690738754706e-16;
static mut o3derrboundA: f64 = 7.771561172376103e-16;
static mut o3derrboundB: f64 = 3.330669073875473e-16;
static mut o3derrboundC: f64 = 3.2047474274603644e-31;

pub unsafe extern "C" fn orient3d(
    mut pa: *mut f64,
    mut pb: *mut f64,
    mut pc: *mut f64,
    mut pd: *mut f64,
) -> f64 {
    let mut adx: f64 = 0.;
    let mut bdx: f64 = 0.;
    let mut cdx: f64 = 0.;
    let mut ady: f64 = 0.;
    let mut bdy: f64 = 0.;
    let mut cdy: f64 = 0.;
    let mut adz: f64 = 0.;
    let mut bdz: f64 = 0.;
    let mut cdz: f64 = 0.;
    let mut bdxcdy: f64 = 0.;
    let mut cdxbdy: f64 = 0.;
    let mut cdxady: f64 = 0.;
    let mut adxcdy: f64 = 0.;
    let mut adxbdy: f64 = 0.;
    let mut bdxady: f64 = 0.;
    let mut det: f64 = 0.;
    let mut permanent: f64 = 0.;
    let mut errbound: f64 = 0.;
    adx = *pa.offset(0 as i32 as isize) - *pd.offset(0 as i32 as isize);
    bdx = *pb.offset(0 as i32 as isize) - *pd.offset(0 as i32 as isize);
    cdx = *pc.offset(0 as i32 as isize) - *pd.offset(0 as i32 as isize);
    ady = *pa.offset(1 as i32 as isize) - *pd.offset(1 as i32 as isize);
    bdy = *pb.offset(1 as i32 as isize) - *pd.offset(1 as i32 as isize);
    cdy = *pc.offset(1 as i32 as isize) - *pd.offset(1 as i32 as isize);
    adz = *pa.offset(2 as i32 as isize) - *pd.offset(2 as i32 as isize);
    bdz = *pb.offset(2 as i32 as isize) - *pd.offset(2 as i32 as isize);
    cdz = *pc.offset(2 as i32 as isize) - *pd.offset(2 as i32 as isize);
    bdxcdy = bdx * cdy;
    cdxbdy = cdx * bdy;
    cdxady = cdx * ady;
    adxcdy = adx * cdy;
    adxbdy = adx * bdy;
    bdxady = bdx * ady;
    det = adz * (bdxcdy - cdxbdy)
        + bdz * (cdxady - adxcdy)
        + cdz * (adxbdy - bdxady);
    permanent = ((if bdxcdy >= 0.0f64 { bdxcdy } else { -bdxcdy })
        + (if cdxbdy >= 0.0f64 { cdxbdy } else { -cdxbdy }))
        * (if adz >= 0.0f64 { adz } else { -adz })
        + ((if cdxady >= 0.0f64 { cdxady } else { -cdxady })
            + (if adxcdy >= 0.0f64 { adxcdy } else { -adxcdy }))
            * (if bdz >= 0.0f64 { bdz } else { -bdz })
        + ((if adxbdy >= 0.0f64 { adxbdy } else { -adxbdy })
            + (if bdxady >= 0.0f64 { bdxady } else { -bdxady }))
            * (if cdz >= 0.0f64 { cdz } else { -cdz });
    errbound = o3derrboundA * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    return orient3dadapt(pa, pb, pc, pd, permanent);
}

unsafe extern "C" fn orient3dadapt(
    mut pa: *mut f64,
    mut pb: *mut f64,
    mut pc: *mut f64,
    mut pd: *mut f64,
    mut permanent: f64,
) -> f64 {
    let mut adx: f64 = 0.;
    let mut bdx: f64 = 0.;
    let mut cdx: f64 = 0.;
    let mut ady: f64 = 0.;
    let mut bdy: f64 = 0.;
    let mut cdy: f64 = 0.;
    let mut adz: f64 = 0.;
    let mut bdz: f64 = 0.;
    let mut cdz: f64 = 0.;
    let mut det: f64 = 0.;
    let mut errbound: f64 = 0.;
    let mut bdxcdy1: f64 = 0.;
    let mut cdxbdy1: f64 = 0.;
    let mut cdxady1: f64 = 0.;
    let mut adxcdy1: f64 = 0.;
    let mut adxbdy1: f64 = 0.;
    let mut bdxady1: f64 = 0.;
    let mut bdxcdy0: f64 = 0.;
    let mut cdxbdy0: f64 = 0.;
    let mut cdxady0: f64 = 0.;
    let mut adxcdy0: f64 = 0.;
    let mut adxbdy0: f64 = 0.;
    let mut bdxady0: f64 = 0.;
    let mut bc: [f64; 4] = [0.; 4];
    let mut ca: [f64; 4] = [0.; 4];
    let mut ab: [f64; 4] = [0.; 4];
    let mut bc3: f64 = 0.;
    let mut ca3: f64 = 0.;
    let mut ab3: f64 = 0.;
    let mut adet: [f64; 8] = [0.; 8];
    let mut bdet: [f64; 8] = [0.; 8];
    let mut cdet: [f64; 8] = [0.; 8];
    let mut alen: i32 = 0;
    let mut blen: i32 = 0;
    let mut clen: i32 = 0;
    let mut abdet: [f64; 16] = [0.; 16];
    let mut ablen: i32 = 0;
    let mut finnow: *mut f64 = 0 as *mut f64;
    let mut finother: *mut f64 = 0 as *mut f64;
    let mut finswap: *mut f64 = 0 as *mut f64;
    let mut fin1: [f64; 192] = [0.; 192];
    let mut fin2: [f64; 192] = [0.; 192];
    let mut finlength: i32 = 0;
    let mut adxtail: f64 = 0.;
    let mut bdxtail: f64 = 0.;
    let mut cdxtail: f64 = 0.;
    let mut adytail: f64 = 0.;
    let mut bdytail: f64 = 0.;
    let mut cdytail: f64 = 0.;
    let mut adztail: f64 = 0.;
    let mut bdztail: f64 = 0.;
    let mut cdztail: f64 = 0.;
    let mut at_blarge: f64 = 0.;
    let mut at_clarge: f64 = 0.;
    let mut bt_clarge: f64 = 0.;
    let mut bt_alarge: f64 = 0.;
    let mut ct_alarge: f64 = 0.;
    let mut ct_blarge: f64 = 0.;
    let mut at_b: [f64; 4] = [0.; 4];
    let mut at_c: [f64; 4] = [0.; 4];
    let mut bt_c: [f64; 4] = [0.; 4];
    let mut bt_a: [f64; 4] = [0.; 4];
    let mut ct_a: [f64; 4] = [0.; 4];
    let mut ct_b: [f64; 4] = [0.; 4];
    let mut at_blen: i32 = 0;
    let mut at_clen: i32 = 0;
    let mut bt_clen: i32 = 0;
    let mut bt_alen: i32 = 0;
    let mut ct_alen: i32 = 0;
    let mut ct_blen: i32 = 0;
    let mut bdxt_cdy1: f64 = 0.;
    let mut cdxt_bdy1: f64 = 0.;
    let mut cdxt_ady1: f64 = 0.;
    let mut adxt_cdy1: f64 = 0.;
    let mut adxt_bdy1: f64 = 0.;
    let mut bdxt_ady1: f64 = 0.;
    let mut bdxt_cdy0: f64 = 0.;
    let mut cdxt_bdy0: f64 = 0.;
    let mut cdxt_ady0: f64 = 0.;
    let mut adxt_cdy0: f64 = 0.;
    let mut adxt_bdy0: f64 = 0.;
    let mut bdxt_ady0: f64 = 0.;
    let mut bdyt_cdx1: f64 = 0.;
    let mut cdyt_bdx1: f64 = 0.;
    let mut cdyt_adx1: f64 = 0.;
    let mut adyt_cdx1: f64 = 0.;
    let mut adyt_bdx1: f64 = 0.;
    let mut bdyt_adx1: f64 = 0.;
    let mut bdyt_cdx0: f64 = 0.;
    let mut cdyt_bdx0: f64 = 0.;
    let mut cdyt_adx0: f64 = 0.;
    let mut adyt_cdx0: f64 = 0.;
    let mut adyt_bdx0: f64 = 0.;
    let mut bdyt_adx0: f64 = 0.;
    let mut bct: [f64; 8] = [0.; 8];
    let mut cat: [f64; 8] = [0.; 8];
    let mut abt: [f64; 8] = [0.; 8];
    let mut bctlen: i32 = 0;
    let mut catlen: i32 = 0;
    let mut abtlen: i32 = 0;
    let mut bdxt_cdyt1: f64 = 0.;
    let mut cdxt_bdyt1: f64 = 0.;
    let mut cdxt_adyt1: f64 = 0.;
    let mut adxt_cdyt1: f64 = 0.;
    let mut adxt_bdyt1: f64 = 0.;
    let mut bdxt_adyt1: f64 = 0.;
    let mut bdxt_cdyt0: f64 = 0.;
    let mut cdxt_bdyt0: f64 = 0.;
    let mut cdxt_adyt0: f64 = 0.;
    let mut adxt_cdyt0: f64 = 0.;
    let mut adxt_bdyt0: f64 = 0.;
    let mut bdxt_adyt0: f64 = 0.;
    let mut u: [f64; 4] = [0.; 4];
    let mut v: [f64; 12] = [0.; 12];
    let mut w: [f64; 16] = [0.; 16];
    let mut u3: f64 = 0.;
    let mut vlength: i32 = 0;
    let mut wlength: i32 = 0;
    let mut negate: f64 = 0.;
    let mut bvirt: f64 = 0.;
    let mut avirt: f64 = 0.;
    let mut bround: f64 = 0.;
    let mut around: f64 = 0.;
    let mut c: f64 = 0.;
    let mut abig: f64 = 0.;
    let mut ahi: f64 = 0.;
    let mut alo: f64 = 0.;
    let mut bhi: f64 = 0.;
    let mut blo: f64 = 0.;
    let mut err1: f64 = 0.;
    let mut err2: f64 = 0.;
    let mut err3: f64 = 0.;
    let mut _i: f64 = 0.;
    let mut _j: f64 = 0.;
    let mut _k: f64 = 0.;
    let mut _0: f64 = 0.;
    adx = *pa.offset(0 as i32 as isize) - *pd.offset(0 as i32 as isize);
    bdx = *pb.offset(0 as i32 as isize) - *pd.offset(0 as i32 as isize);
    cdx = *pc.offset(0 as i32 as isize) - *pd.offset(0 as i32 as isize);
    ady = *pa.offset(1 as i32 as isize) - *pd.offset(1 as i32 as isize);
    bdy = *pb.offset(1 as i32 as isize) - *pd.offset(1 as i32 as isize);
    cdy = *pc.offset(1 as i32 as isize) - *pd.offset(1 as i32 as isize);
    adz = *pa.offset(2 as i32 as isize) - *pd.offset(2 as i32 as isize);
    bdz = *pb.offset(2 as i32 as isize) - *pd.offset(2 as i32 as isize);
    cdz = *pc.offset(2 as i32 as isize) - *pd.offset(2 as i32 as isize);
    bdxcdy1 = bdx * cdy;
    c = splitter * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = bdxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bdxcdy0 = alo * blo - err3;
    cdxbdy1 = cdx * bdy;
    c = splitter * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = cdxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cdxbdy0 = alo * blo - err3;
    _i = bdxcdy0 - cdxbdy0;
    bvirt = bdxcdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - cdxbdy0;
    around = bdxcdy0 - avirt;
    bc[0 as i32 as usize] = around + bround;
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
    bc[1 as i32 as usize] = around + bround;
    bc3 = _j + _i;
    bvirt = bc3 - _j;
    avirt = bc3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    bc[2 as i32 as usize] = around + bround;
    bc[3 as i32 as usize] = bc3;
    alen = scale_expansion_zeroelim(
        4 as i32,
        bc.as_mut_ptr(),
        adz,
        adet.as_mut_ptr(),
    );
    cdxady1 = cdx * ady;
    c = splitter * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = cdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    cdxady0 = alo * blo - err3;
    adxcdy1 = adx * cdy;
    c = splitter * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = splitter * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = adxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    adxcdy0 = alo * blo - err3;
    _i = cdxady0 - adxcdy0;
    bvirt = cdxady0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - adxcdy0;
    around = cdxady0 - avirt;
    ca[0 as i32 as usize] = around + bround;
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
    ca[1 as i32 as usize] = around + bround;
    ca3 = _j + _i;
    bvirt = ca3 - _j;
    avirt = ca3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ca[2 as i32 as usize] = around + bround;
    ca[3 as i32 as usize] = ca3;
    blen = scale_expansion_zeroelim(
        4 as i32,
        ca.as_mut_ptr(),
        bdz,
        bdet.as_mut_ptr(),
    );
    adxbdy1 = adx * bdy;
    c = splitter * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = splitter * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = adxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    adxbdy0 = alo * blo - err3;
    bdxady1 = bdx * ady;
    c = splitter * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = splitter * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = bdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    bdxady0 = alo * blo - err3;
    _i = adxbdy0 - bdxady0;
    bvirt = adxbdy0 - _i;
    avirt = _i + bvirt;
    bround = bvirt - bdxady0;
    around = adxbdy0 - avirt;
    ab[0 as i32 as usize] = around + bround;
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
    ab[1 as i32 as usize] = around + bround;
    ab3 = _j + _i;
    bvirt = ab3 - _j;
    avirt = ab3 - bvirt;
    bround = _i - bvirt;
    around = _j - avirt;
    ab[2 as i32 as usize] = around + bround;
    ab[3 as i32 as usize] = ab3;
    clen = scale_expansion_zeroelim(
        4 as i32,
        ab.as_mut_ptr(),
        cdz,
        cdet.as_mut_ptr(),
    );
    ablen = fast_expansion_sum_zeroelim(
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
    errbound = o3derrboundB * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }
    bvirt = *pa.offset(0 as i32 as isize) - adx;
    avirt = adx + bvirt;
    bround = bvirt - *pd.offset(0 as i32 as isize);
    around = *pa.offset(0 as i32 as isize) - avirt;
    adxtail = around + bround;
    bvirt = *pb.offset(0 as i32 as isize) - bdx;
    avirt = bdx + bvirt;
    bround = bvirt - *pd.offset(0 as i32 as isize);
    around = *pb.offset(0 as i32 as isize) - avirt;
    bdxtail = around + bround;
    bvirt = *pc.offset(0 as i32 as isize) - cdx;
    avirt = cdx + bvirt;
    bround = bvirt - *pd.offset(0 as i32 as isize);
    around = *pc.offset(0 as i32 as isize) - avirt;
    cdxtail = around + bround;
    bvirt = *pa.offset(1 as i32 as isize) - ady;
    avirt = ady + bvirt;
    bround = bvirt - *pd.offset(1 as i32 as isize);
    around = *pa.offset(1 as i32 as isize) - avirt;
    adytail = around + bround;
    bvirt = *pb.offset(1 as i32 as isize) - bdy;
    avirt = bdy + bvirt;
    bround = bvirt - *pd.offset(1 as i32 as isize);
    around = *pb.offset(1 as i32 as isize) - avirt;
    bdytail = around + bround;
    bvirt = *pc.offset(1 as i32 as isize) - cdy;
    avirt = cdy + bvirt;
    bround = bvirt - *pd.offset(1 as i32 as isize);
    around = *pc.offset(1 as i32 as isize) - avirt;
    cdytail = around + bround;
    bvirt = *pa.offset(2 as i32 as isize) - adz;
    avirt = adz + bvirt;
    bround = bvirt - *pd.offset(2 as i32 as isize);
    around = *pa.offset(2 as i32 as isize) - avirt;
    adztail = around + bround;
    bvirt = *pb.offset(2 as i32 as isize) - bdz;
    avirt = bdz + bvirt;
    bround = bvirt - *pd.offset(2 as i32 as isize);
    around = *pb.offset(2 as i32 as isize) - avirt;
    bdztail = around + bround;
    bvirt = *pc.offset(2 as i32 as isize) - cdz;
    avirt = cdz + bvirt;
    bround = bvirt - *pd.offset(2 as i32 as isize);
    around = *pc.offset(2 as i32 as isize) - avirt;
    cdztail = around + bround;
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
    errbound = o3derrboundC * permanent
        + resulterrbound * (if det >= 0.0f64 { det } else { -det });
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
            at_b[0 as i32 as usize] = 0.0f64;
            at_blen = 1 as i32;
            at_c[0 as i32 as usize] = 0.0f64;
            at_clen = 1 as i32;
        } else {
            negate = -adytail;
            at_blarge = negate * bdx;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = at_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_b[0 as i32 as usize] = alo * blo - err3;
            at_b[1 as i32 as usize] = at_blarge;
            at_blen = 2 as i32;
            at_clarge = adytail * cdx;
            c = splitter * adytail;
            abig = c - adytail;
            ahi = c - abig;
            alo = adytail - ahi;
            c = splitter * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = at_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_c[0 as i32 as usize] = alo * blo - err3;
            at_c[1 as i32 as usize] = at_clarge;
            at_clen = 2 as i32;
        }
    } else if adytail == 0.0f64 {
        at_blarge = adxtail * bdy;
        c = splitter * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = splitter * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = at_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_b[0 as i32 as usize] = alo * blo - err3;
        at_b[1 as i32 as usize] = at_blarge;
        at_blen = 2 as i32;
        negate = -adxtail;
        at_clarge = negate * cdy;
        c = splitter * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = splitter * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = at_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_c[0 as i32 as usize] = alo * blo - err3;
        at_c[1 as i32 as usize] = at_clarge;
        at_clen = 2 as i32;
    } else {
        adxt_bdy1 = adxtail * bdy;
        c = splitter * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = splitter * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = adxt_bdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adxt_bdy0 = alo * blo - err3;
        adyt_bdx1 = adytail * bdx;
        c = splitter * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = splitter * bdx;
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
        at_b[0 as i32 as usize] = around + bround;
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
        at_b[1 as i32 as usize] = around + bround;
        at_blarge = _j + _i;
        bvirt = at_blarge - _j;
        avirt = at_blarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        at_b[2 as i32 as usize] = around + bround;
        at_b[3 as i32 as usize] = at_blarge;
        at_blen = 4 as i32;
        adyt_cdx1 = adytail * cdx;
        c = splitter * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = splitter * cdx;
        abig = c - cdx;
        bhi = c - abig;
        blo = cdx - bhi;
        err1 = adyt_cdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adyt_cdx0 = alo * blo - err3;
        adxt_cdy1 = adxtail * cdy;
        c = splitter * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = splitter * cdy;
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
        at_c[0 as i32 as usize] = around + bround;
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
        at_c[1 as i32 as usize] = around + bround;
        at_clarge = _j + _i;
        bvirt = at_clarge - _j;
        avirt = at_clarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        at_c[2 as i32 as usize] = around + bround;
        at_c[3 as i32 as usize] = at_clarge;
        at_clen = 4 as i32;
    }
    if bdxtail == 0.0f64 {
        if bdytail == 0.0f64 {
            bt_c[0 as i32 as usize] = 0.0f64;
            bt_clen = 1 as i32;
            bt_a[0 as i32 as usize] = 0.0f64;
            bt_alen = 1 as i32;
        } else {
            negate = -bdytail;
            bt_clarge = negate * cdx;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = bt_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_c[0 as i32 as usize] = alo * blo - err3;
            bt_c[1 as i32 as usize] = bt_clarge;
            bt_clen = 2 as i32;
            bt_alarge = bdytail * adx;
            c = splitter * bdytail;
            abig = c - bdytail;
            ahi = c - abig;
            alo = bdytail - ahi;
            c = splitter * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = bt_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_a[0 as i32 as usize] = alo * blo - err3;
            bt_a[1 as i32 as usize] = bt_alarge;
            bt_alen = 2 as i32;
        }
    } else if bdytail == 0.0f64 {
        bt_clarge = bdxtail * cdy;
        c = splitter * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = splitter * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bt_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_c[0 as i32 as usize] = alo * blo - err3;
        bt_c[1 as i32 as usize] = bt_clarge;
        bt_clen = 2 as i32;
        negate = -bdxtail;
        bt_alarge = negate * ady;
        c = splitter * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = splitter * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = bt_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_a[0 as i32 as usize] = alo * blo - err3;
        bt_a[1 as i32 as usize] = bt_alarge;
        bt_alen = 2 as i32;
    } else {
        bdxt_cdy1 = bdxtail * cdy;
        c = splitter * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = splitter * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bdxt_cdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdxt_cdy0 = alo * blo - err3;
        bdyt_cdx1 = bdytail * cdx;
        c = splitter * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = splitter * cdx;
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
        bt_c[0 as i32 as usize] = around + bround;
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
        bt_c[1 as i32 as usize] = around + bround;
        bt_clarge = _j + _i;
        bvirt = bt_clarge - _j;
        avirt = bt_clarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        bt_c[2 as i32 as usize] = around + bround;
        bt_c[3 as i32 as usize] = bt_clarge;
        bt_clen = 4 as i32;
        bdyt_adx1 = bdytail * adx;
        c = splitter * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = splitter * adx;
        abig = c - adx;
        bhi = c - abig;
        blo = adx - bhi;
        err1 = bdyt_adx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdyt_adx0 = alo * blo - err3;
        bdxt_ady1 = bdxtail * ady;
        c = splitter * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = splitter * ady;
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
        bt_a[0 as i32 as usize] = around + bround;
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
        bt_a[1 as i32 as usize] = around + bround;
        bt_alarge = _j + _i;
        bvirt = bt_alarge - _j;
        avirt = bt_alarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        bt_a[2 as i32 as usize] = around + bround;
        bt_a[3 as i32 as usize] = bt_alarge;
        bt_alen = 4 as i32;
    }
    if cdxtail == 0.0f64 {
        if cdytail == 0.0f64 {
            ct_a[0 as i32 as usize] = 0.0f64;
            ct_alen = 1 as i32;
            ct_b[0 as i32 as usize] = 0.0f64;
            ct_blen = 1 as i32;
        } else {
            negate = -cdytail;
            ct_alarge = negate * adx;
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = ct_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_a[0 as i32 as usize] = alo * blo - err3;
            ct_a[1 as i32 as usize] = ct_alarge;
            ct_alen = 2 as i32;
            ct_blarge = cdytail * bdx;
            c = splitter * cdytail;
            abig = c - cdytail;
            ahi = c - abig;
            alo = cdytail - ahi;
            c = splitter * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = ct_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_b[0 as i32 as usize] = alo * blo - err3;
            ct_b[1 as i32 as usize] = ct_blarge;
            ct_blen = 2 as i32;
        }
    } else if cdytail == 0.0f64 {
        ct_alarge = cdxtail * ady;
        c = splitter * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = splitter * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = ct_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_a[0 as i32 as usize] = alo * blo - err3;
        ct_a[1 as i32 as usize] = ct_alarge;
        ct_alen = 2 as i32;
        negate = -cdxtail;
        ct_blarge = negate * bdy;
        c = splitter * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = splitter * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = ct_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_b[0 as i32 as usize] = alo * blo - err3;
        ct_b[1 as i32 as usize] = ct_blarge;
        ct_blen = 2 as i32;
    } else {
        cdxt_ady1 = cdxtail * ady;
        c = splitter * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = splitter * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = cdxt_ady1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdxt_ady0 = alo * blo - err3;
        cdyt_adx1 = cdytail * adx;
        c = splitter * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = splitter * adx;
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
        ct_a[0 as i32 as usize] = around + bround;
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
        ct_a[1 as i32 as usize] = around + bround;
        ct_alarge = _j + _i;
        bvirt = ct_alarge - _j;
        avirt = ct_alarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        ct_a[2 as i32 as usize] = around + bround;
        ct_a[3 as i32 as usize] = ct_alarge;
        ct_alen = 4 as i32;
        cdyt_bdx1 = cdytail * bdx;
        c = splitter * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = splitter * bdx;
        abig = c - bdx;
        bhi = c - abig;
        blo = bdx - bhi;
        err1 = cdyt_bdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdyt_bdx0 = alo * blo - err3;
        cdxt_bdy1 = cdxtail * bdy;
        c = splitter * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = splitter * bdy;
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
        ct_b[0 as i32 as usize] = around + bround;
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
        ct_b[1 as i32 as usize] = around + bround;
        ct_blarge = _j + _i;
        bvirt = ct_blarge - _j;
        avirt = ct_blarge - bvirt;
        bround = _i - bvirt;
        around = _j - avirt;
        ct_b[2 as i32 as usize] = around + bround;
        ct_b[3 as i32 as usize] = ct_blarge;
        ct_blen = 4 as i32;
    }
    bctlen = fast_expansion_sum_zeroelim(
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
    catlen = fast_expansion_sum_zeroelim(
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
    abtlen = fast_expansion_sum_zeroelim(
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
            4 as i32,
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
            4 as i32,
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
            4 as i32,
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
            c = splitter * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = splitter * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = adxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_bdyt0 = alo * blo - err3;
            c = splitter * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            _i = adxt_bdyt0 * cdz;
            c = splitter * adxt_bdyt0;
            abig = c - adxt_bdyt0;
            ahi = c - abig;
            alo = adxt_bdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as i32 as usize] = alo * blo - err3;
            _j = adxt_bdyt1 * cdz;
            c = splitter * adxt_bdyt1;
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
            u[1 as i32 as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as i32 as usize] = _k - bvirt;
            u[3 as i32 as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as i32,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if cdztail != 0.0f64 {
                c = splitter * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                _i = adxt_bdyt0 * cdztail;
                c = splitter * adxt_bdyt0;
                abig = c - adxt_bdyt0;
                ahi = c - abig;
                alo = adxt_bdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as i32 as usize] = alo * blo - err3;
                _j = adxt_bdyt1 * cdztail;
                c = splitter * adxt_bdyt1;
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
                u[1 as i32 as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as i32 as usize] = _k - bvirt;
                u[3 as i32 as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as i32,
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
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = adxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_cdyt0 = alo * blo - err3;
            c = splitter * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            _i = adxt_cdyt0 * bdz;
            c = splitter * adxt_cdyt0;
            abig = c - adxt_cdyt0;
            ahi = c - abig;
            alo = adxt_cdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as i32 as usize] = alo * blo - err3;
            _j = adxt_cdyt1 * bdz;
            c = splitter * adxt_cdyt1;
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
            u[1 as i32 as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as i32 as usize] = _k - bvirt;
            u[3 as i32 as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as i32,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if bdztail != 0.0f64 {
                c = splitter * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                _i = adxt_cdyt0 * bdztail;
                c = splitter * adxt_cdyt0;
                abig = c - adxt_cdyt0;
                ahi = c - abig;
                alo = adxt_cdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as i32 as usize] = alo * blo - err3;
                _j = adxt_cdyt1 * bdztail;
                c = splitter * adxt_cdyt1;
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
                u[1 as i32 as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as i32 as usize] = _k - bvirt;
                u[3 as i32 as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as i32,
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
            c = splitter * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = splitter * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = bdxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_cdyt0 = alo * blo - err3;
            c = splitter * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            _i = bdxt_cdyt0 * adz;
            c = splitter * bdxt_cdyt0;
            abig = c - bdxt_cdyt0;
            ahi = c - abig;
            alo = bdxt_cdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as i32 as usize] = alo * blo - err3;
            _j = bdxt_cdyt1 * adz;
            c = splitter * bdxt_cdyt1;
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
            u[1 as i32 as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as i32 as usize] = _k - bvirt;
            u[3 as i32 as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as i32,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if adztail != 0.0f64 {
                c = splitter * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                _i = bdxt_cdyt0 * adztail;
                c = splitter * bdxt_cdyt0;
                abig = c - bdxt_cdyt0;
                ahi = c - abig;
                alo = bdxt_cdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as i32 as usize] = alo * blo - err3;
                _j = bdxt_cdyt1 * adztail;
                c = splitter * bdxt_cdyt1;
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
                u[1 as i32 as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as i32 as usize] = _k - bvirt;
                u[3 as i32 as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as i32,
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
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = bdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_adyt0 = alo * blo - err3;
            c = splitter * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            _i = bdxt_adyt0 * cdz;
            c = splitter * bdxt_adyt0;
            abig = c - bdxt_adyt0;
            ahi = c - abig;
            alo = bdxt_adyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as i32 as usize] = alo * blo - err3;
            _j = bdxt_adyt1 * cdz;
            c = splitter * bdxt_adyt1;
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
            u[1 as i32 as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as i32 as usize] = _k - bvirt;
            u[3 as i32 as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as i32,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if cdztail != 0.0f64 {
                c = splitter * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                _i = bdxt_adyt0 * cdztail;
                c = splitter * bdxt_adyt0;
                abig = c - bdxt_adyt0;
                ahi = c - abig;
                alo = bdxt_adyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as i32 as usize] = alo * blo - err3;
                _j = bdxt_adyt1 * cdztail;
                c = splitter * bdxt_adyt1;
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
                u[1 as i32 as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as i32 as usize] = _k - bvirt;
                u[3 as i32 as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as i32,
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
            c = splitter * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = splitter * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = cdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_adyt0 = alo * blo - err3;
            c = splitter * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            _i = cdxt_adyt0 * bdz;
            c = splitter * cdxt_adyt0;
            abig = c - cdxt_adyt0;
            ahi = c - abig;
            alo = cdxt_adyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as i32 as usize] = alo * blo - err3;
            _j = cdxt_adyt1 * bdz;
            c = splitter * cdxt_adyt1;
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
            u[1 as i32 as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as i32 as usize] = _k - bvirt;
            u[3 as i32 as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as i32,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if bdztail != 0.0f64 {
                c = splitter * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                _i = cdxt_adyt0 * bdztail;
                c = splitter * cdxt_adyt0;
                abig = c - cdxt_adyt0;
                ahi = c - abig;
                alo = cdxt_adyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as i32 as usize] = alo * blo - err3;
                _j = cdxt_adyt1 * bdztail;
                c = splitter * cdxt_adyt1;
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
                u[1 as i32 as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as i32 as usize] = _k - bvirt;
                u[3 as i32 as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as i32,
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
            c = splitter * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = splitter * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = cdxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_bdyt0 = alo * blo - err3;
            c = splitter * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            _i = cdxt_bdyt0 * adz;
            c = splitter * cdxt_bdyt0;
            abig = c - cdxt_bdyt0;
            ahi = c - abig;
            alo = cdxt_bdyt0 - ahi;
            err1 = _i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0 as i32 as usize] = alo * blo - err3;
            _j = cdxt_bdyt1 * adz;
            c = splitter * cdxt_bdyt1;
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
            u[1 as i32 as usize] = around + bround;
            u3 = _j + _k;
            bvirt = u3 - _j;
            u[2 as i32 as usize] = _k - bvirt;
            u[3 as i32 as usize] = u3;
            finlength = fast_expansion_sum_zeroelim(
                finlength,
                finnow,
                4 as i32,
                u.as_mut_ptr(),
                finother,
            );
            finswap = finnow;
            finnow = finother;
            finother = finswap;
            if adztail != 0.0f64 {
                c = splitter * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                _i = cdxt_bdyt0 * adztail;
                c = splitter * cdxt_bdyt0;
                abig = c - cdxt_bdyt0;
                ahi = c - abig;
                alo = cdxt_bdyt0 - ahi;
                err1 = _i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0 as i32 as usize] = alo * blo - err3;
                _j = cdxt_bdyt1 * adztail;
                c = splitter * cdxt_bdyt1;
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
                u[1 as i32 as usize] = around + bround;
                u3 = _j + _k;
                bvirt = u3 - _j;
                u[2 as i32 as usize] = _k - bvirt;
                u[3 as i32 as usize] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength,
                    finnow,
                    4 as i32,
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
        finswap = finnow;
        finnow = finother;
        finother = finswap;
    }
    return *finnow.offset((finlength - 1 as i32) as isize);
}

unsafe extern "C" fn scale_expansion_zeroelim(
    mut elen: i32,
    mut e: *mut f64,
    mut b: f64,
    mut h: *mut f64,
) -> i32 {
    let mut Q: f64 = 0.;
    let mut sum: f64 = 0.;
    let mut hh: f64 = 0.;
    let mut product1: f64 = 0.;
    let mut product0: f64 = 0.;
    let mut eindex: i32 = 0;
    let mut hindex: i32 = 0;
    let mut enow: f64 = 0.;
    let mut bvirt: f64 = 0.;
    let mut avirt: f64 = 0.;
    let mut bround: f64 = 0.;
    let mut around: f64 = 0.;
    let mut c: f64 = 0.;
    let mut abig: f64 = 0.;
    let mut ahi: f64 = 0.;
    let mut alo: f64 = 0.;
    let mut bhi: f64 = 0.;
    let mut blo: f64 = 0.;
    let mut err1: f64 = 0.;
    let mut err2: f64 = 0.;
    let mut err3: f64 = 0.;
    c = splitter * b;
    abig = c - b;
    bhi = c - abig;
    blo = b - bhi;
    Q = *e.offset(0 as i32 as isize) * b;
    c = splitter * *e.offset(0 as i32 as isize);
    abig = c - *e.offset(0 as i32 as isize);
    ahi = c - abig;
    alo = *e.offset(0 as i32 as isize) - ahi;
    err1 = Q - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    hh = alo * blo - err3;
    hindex = 0 as i32;
    if hh != 0 as i32 as f64 {
        let fresh12 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh12 as isize) = hh;
    }
    eindex = 1 as i32;
    while eindex < elen {
        enow = *e.offset(eindex as isize);
        product1 = enow * b;
        c = splitter * enow;
        abig = c - enow;
        ahi = c - abig;
        alo = enow - ahi;
        err1 = product1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        product0 = alo * blo - err3;
        sum = Q + product0;
        bvirt = sum - Q;
        avirt = sum - bvirt;
        bround = product0 - bvirt;
        around = Q - avirt;
        hh = around + bround;
        if hh != 0 as i32 as f64 {
            let fresh13 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh13 as isize) = hh;
        }
        Q = product1 + sum;
        bvirt = Q - product1;
        hh = sum - bvirt;
        if hh != 0 as i32 as f64 {
            let fresh14 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh14 as isize) = hh;
        }
        eindex += 1;
    }
    if Q != 0.0f64 || hindex == 0 as i32 {
        let fresh15 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh15 as isize) = Q;
    }
    return hindex;
}

unsafe extern "C" fn fast_expansion_sum_zeroelim(
    mut elen: i32,
    mut e: *mut f64,
    mut flen: i32,
    mut f: *mut f64,
    mut h: *mut f64,
) -> i32 {
    let mut Q: f64 = 0.;
    let mut Qnew: f64 = 0.;
    let mut hh: f64 = 0.;
    let mut bvirt: f64 = 0.;
    let mut avirt: f64 = 0.;
    let mut bround: f64 = 0.;
    let mut around: f64 = 0.;
    let mut eindex: i32 = 0;
    let mut findex: i32 = 0;
    let mut hindex: i32 = 0;
    let mut enow: f64 = 0.;
    let mut fnow: f64 = 0.;
    enow = *e.offset(0 as i32 as isize);
    fnow = *f.offset(0 as i32 as isize);
    findex = 0 as i32;
    eindex = findex;
    if (fnow > enow) as i32 == (fnow > -enow) as i32 {
        Q = enow;
        eindex += 1;
        enow = *e.offset(eindex as isize);
    } else {
        Q = fnow;
        findex += 1;
        fnow = *f.offset(findex as isize);
    }
    hindex = 0 as i32;
    if eindex < elen && findex < flen {
        if (fnow > enow) as i32 == (fnow > -enow) as i32 {
            Qnew = enow + Q;
            bvirt = Qnew - enow;
            hh = Q - bvirt;
            eindex += 1;
            enow = *e.offset(eindex as isize);
        } else {
            Qnew = fnow + Q;
            bvirt = Qnew - fnow;
            hh = Q - bvirt;
            findex += 1;
            fnow = *f.offset(findex as isize);
        }
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh4 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh4 as isize) = hh;
        }
        while eindex < elen && findex < flen {
            if (fnow > enow) as i32 == (fnow > -enow) as i32 {
                Qnew = Q + enow;
                bvirt = Qnew - Q;
                avirt = Qnew - bvirt;
                bround = enow - bvirt;
                around = Q - avirt;
                hh = around + bround;
                eindex += 1;
                enow = *e.offset(eindex as isize);
            } else {
                Qnew = Q + fnow;
                bvirt = Qnew - Q;
                avirt = Qnew - bvirt;
                bround = fnow - bvirt;
                around = Q - avirt;
                hh = around + bround;
                findex += 1;
                fnow = *f.offset(findex as isize);
            }
            Q = Qnew;
            if hh != 0.0f64 {
                let fresh5 = hindex;
                hindex = hindex + 1;
                *h.offset(fresh5 as isize) = hh;
            }
        }
    }
    while eindex < elen {
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = enow - bvirt;
        around = Q - avirt;
        hh = around + bround;
        eindex += 1;
        enow = *e.offset(eindex as isize);
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh6 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh6 as isize) = hh;
        }
    }
    while findex < flen {
        Qnew = Q + fnow;
        bvirt = Qnew - Q;
        avirt = Qnew - bvirt;
        bround = fnow - bvirt;
        around = Q - avirt;
        hh = around + bround;
        findex += 1;
        fnow = *f.offset(findex as isize);
        Q = Qnew;
        if hh != 0.0f64 {
            let fresh7 = hindex;
            hindex = hindex + 1;
            *h.offset(fresh7 as isize) = hh;
        }
    }
    if Q != 0.0f64 || hindex == 0 as i32 {
        let fresh8 = hindex;
        hindex = hindex + 1;
        *h.offset(fresh8 as isize) = Q;
    }
    return hindex;
}

unsafe extern "C" fn estimate(mut elen: i32, mut e: *mut f64) -> f64 {
    let mut Q: f64 = 0.;
    let mut eindex: i32 = 0;
    Q = *e.offset(0 as i32 as isize);
    eindex = 1 as i32;
    while eindex < elen {
        Q += *e.offset(eindex as isize);
        eindex += 1;
    }
    return Q;
}
