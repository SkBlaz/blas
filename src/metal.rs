//! Minimalistic wrappers for BLAS routines.

use libc::c_char;
use num::Complex;
use raw::{self, int};

pub enum Trans {
    /// Do not transponse.
    N = b'N' as isize,
    /// Transpose.
    T = b'T' as isize,
    /// Conjugate transpose.
    C = b'C' as isize,
}
pub enum Uplo {
    /// Upper triangular
    U = b'U' as isize,
    /// Lower triangular
    L = b'L' as isize,
}
pub enum Diag {
    /// Unit triangular
    N = b'N' as isize,
    /// Non-unit triangular
    U = b'U' as isize,
}
pub enum Side {
    /// Do the operation on the left
    L = b'L' as isize,
    /// Do the operation on the right.
    R = b'R' as isize,
}

#[inline]
pub fn srotg(a: &mut [f32], b: &mut [f32], c: &mut [f32], s: &mut [f32]) {
    unsafe {
        raw::srotg_(a.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _,
                    c.as_mut_ptr() as *mut _,
                    s.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn srotmg(d1: &mut [f32], d2: &mut [f32], x1: &mut [f32], y1: &[f32], param: &mut [f32]) {
    unsafe {
        raw::srotmg_(d1.as_mut_ptr() as *mut _,
                     d2.as_mut_ptr() as *mut _,
                     x1.as_mut_ptr() as *mut _,
                     y1.as_ptr() as *const _,
                     param.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn srot(n: usize, x: &mut [f32], incx: usize, y: &mut [f32], incy: usize, c: &[f32],
            s: &[f32]) {

    unsafe {
        raw::srot_(&(n as int) as *const _,
                   x.as_mut_ptr() as *mut _,
                   &(incx as int) as *const _,
                   y.as_mut_ptr() as *mut _,
                   &(incy as int) as *const _,
                   c.as_ptr() as *const _,
                   s.as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn srotm(n: usize, x: &mut [f32], incx: usize, y: &mut [f32], incy: usize, param: &[f32]) {
    unsafe {
        raw::srotm_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
                    param.as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn sswap(n: usize, x: &mut [f32], incx: usize, y: &mut [f32], incy: usize) {
    unsafe {
        raw::sswap_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn sscal(n: usize, a: &[f32], x: &mut [f32], incx: usize) {
    unsafe {
        raw::sscal_(&(n as int) as *const _,
                    a.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn scopy(n: usize, x: &[f32], incx: usize, y: &mut [f32], incy: usize) {
    unsafe {
        raw::scopy_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn saxpy(n: usize, alpha: f32, x: &[f32], incx: usize, y: &mut [f32], incy: usize) {
    unsafe {
        raw::saxpy_(&(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn sdot(n: usize, x: &[f32], incx: usize, y: &[f32], incy: usize) -> f32 {
    unsafe {
        raw::sdot_(&(n as int) as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   y.as_ptr() as *const _,
                   &(incy as int) as *const _,
        ) as f32
    }
}

#[inline]
pub fn sdsdot(n: usize, sb: &[f32], x: &[f32], incx: usize, y: &[f32], incy: usize) -> f32 {
    unsafe {
        raw::sdsdot_(&(n as int) as *const _,
                     sb.as_ptr() as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
                     y.as_ptr() as *const _,
                     &(incy as int) as *const _,
        ) as f32
    }
}

#[inline]
pub fn snrm2(n: usize, x: &[f32], incx: usize) -> f32 {
    unsafe {
        raw::snrm2_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
        ) as f32
    }
}

#[inline]
pub fn scnrm2(n: usize, x: &[f32], incx: usize) -> f32 {
    unsafe {
        raw::scnrm2_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as f32
    }
}

#[inline]
pub fn sasum(n: usize, x: &[f32], incx: usize) -> f32 {
    unsafe {
        raw::sasum_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
        ) as f32
    }
}

#[inline]
pub fn isamax(n: usize, x: &[f32], incx: usize) -> isize {
    unsafe {
        raw::isamax_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as isize
    }
}

#[inline]
pub fn drotg(a: &mut [f64], b: &mut [f64], c: &mut [f64], s: &mut [f64]) {
    unsafe {
        raw::drotg_(a.as_mut_ptr() as *mut _,
                    b.as_mut_ptr() as *mut _,
                    c.as_mut_ptr() as *mut _,
                    s.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn drotmg(d1: &mut [f64], d2: &mut [f64], x1: &mut [f64], y1: &[f64], param: &mut [f64]) {
    unsafe {
        raw::drotmg_(d1.as_mut_ptr() as *mut _,
                     d2.as_mut_ptr() as *mut _,
                     x1.as_mut_ptr() as *mut _,
                     y1.as_ptr() as *const _,
                     param.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn drot(n: usize, x: &mut [f64], incx: usize, y: &mut [f64], incy: usize, c: &[f64],
            s: &[f64]) {

    unsafe {
        raw::drot_(&(n as int) as *const _,
                   x.as_mut_ptr() as *mut _,
                   &(incx as int) as *const _,
                   y.as_mut_ptr() as *mut _,
                   &(incy as int) as *const _,
                   c.as_ptr() as *const _,
                   s.as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn drotm(n: usize, x: &mut [f64], incx: usize, y: &mut [f64], incy: usize, param: &[f64]) {
    unsafe {
        raw::drotm_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
                    param.as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn dswap(n: usize, x: &mut [f64], incx: usize, y: &mut [f64], incy: usize) {
    unsafe {
        raw::dswap_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn dscal(n: usize, a: &[f64], x: &mut [f64], incx: usize) {
    unsafe {
        raw::dscal_(&(n as int) as *const _,
                    a.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn dcopy(n: usize, x: &[f64], incx: usize, y: &mut [f64], incy: usize) {
    unsafe {
        raw::dcopy_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn daxpy(n: usize, alpha: f64, x: &[f64], incx: usize, y: &mut [f64], incy: usize) {
    unsafe {
        raw::daxpy_(&(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn ddot(n: usize, x: &[f64], incx: usize, y: &[f64], incy: usize) -> f64 {
    unsafe {
        raw::ddot_(&(n as int) as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   y.as_ptr() as *const _,
                   &(incy as int) as *const _,
        ) as f64
    }
}

#[inline]
pub fn dsdot(n: usize, x: &[f64], incx: usize, y: &[f64], incy: usize) -> f64 {
    unsafe {
        raw::dsdot_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
        ) as f64
    }
}

#[inline]
pub fn dnrm2(n: usize, x: &[f64], incx: usize) -> f64 {
    unsafe {
        raw::dnrm2_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
        ) as f64
    }
}

#[inline]
pub fn dznrm2(n: usize, x: &[f64], incx: usize) -> f64 {
    unsafe {
        raw::dznrm2_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as f64
    }
}

#[inline]
pub fn dasum(n: usize, x: &[f64], incx: usize) -> f64 {
    unsafe {
        raw::dasum_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
        ) as f64
    }
}

#[inline]
pub fn idamax(n: usize, x: &[f64], incx: usize) -> isize {
    unsafe {
        raw::idamax_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as isize
    }
}

#[inline]
pub fn crotg(a: &mut [Complex<f32>], b: &[Complex<f32>], c: &mut [Complex<f32>],
             s: &mut [Complex<f32>]) {

    unsafe {
        raw::crotg_(a.as_mut_ptr() as *mut _,
                    b.as_ptr() as *const _,
                    c.as_mut_ptr() as *mut _,
                    s.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn csrot(n: usize, x: &mut [Complex<f32>], incx: usize, y: &mut [Complex<f32>], incy: usize,
             c: &[Complex<f32>], s: &[Complex<f32>]) {

    unsafe {
        raw::csrot_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
                    c.as_ptr() as *const _,
                    s.as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn cswap(n: usize, x: &mut [Complex<f32>], incx: usize, y: &mut [Complex<f32>], incy: usize) {
    unsafe {
        raw::cswap_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn cscal(n: usize, a: &[Complex<f32>], x: &mut [Complex<f32>], incx: usize) {
    unsafe {
        raw::cscal_(&(n as int) as *const _,
                    a.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn csscal(n: usize, a: &[Complex<f32>], x: &mut [Complex<f32>], incx: usize) {
    unsafe {
        raw::csscal_(&(n as int) as *const _,
                     a.as_ptr() as *const _,
                     x.as_mut_ptr() as *mut _,
                     &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ccopy(n: usize, x: &[Complex<f32>], incx: usize, y: &mut [Complex<f32>], incy: usize) {
    unsafe {
        raw::ccopy_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn caxpy(n: usize, alpha: Complex<f32>, x: &[Complex<f32>], incx: usize,
             y: &mut [Complex<f32>], incy: usize) {

    unsafe {
        raw::caxpy_(&(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn cdotu(pres: &mut [Complex<f32>], n: usize, x: &[Complex<f32>], incx: usize,
             y: &[Complex<f32>], incy: usize) {

    unsafe {
        raw::cdotu_(pres.as_mut_ptr() as *mut _,
                    &(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn cdotc(pres: &mut [Complex<f32>], n: usize, x: &[Complex<f32>], incx: usize,
             y: &[Complex<f32>], incy: usize) {

    unsafe {
        raw::cdotc_(pres.as_mut_ptr() as *mut _,
                    &(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn scasum(n: usize, x: &[Complex<f32>], incx: usize) -> f32 {
    unsafe {
        raw::scasum_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as f32
    }
}

#[inline]
pub fn icamax(n: usize, x: &[Complex<f32>], incx: usize) -> isize {
    unsafe {
        raw::icamax_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as isize
    }
}

#[inline]
pub fn zrotg(a: &mut [Complex<f64>], b: &[Complex<f64>], c: &mut [Complex<f64>],
             s: &mut [Complex<f64>]) {

    unsafe {
        raw::zrotg_(a.as_mut_ptr() as *mut _,
                    b.as_ptr() as *const _,
                    c.as_mut_ptr() as *mut _,
                    s.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn zdrot(n: usize, x: &mut [Complex<f64>], incx: usize, y: &mut [Complex<f64>], incy: usize,
             c: &[Complex<f64>], s: &[Complex<f64>]) {

    unsafe {
        raw::zdrot_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
                    c.as_ptr() as *const _,
                    s.as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn zswap(n: usize, x: &mut [Complex<f64>], incx: usize, y: &mut [Complex<f64>], incy: usize) {
    unsafe {
        raw::zswap_(&(n as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zscal(n: usize, a: &[Complex<f64>], x: &mut [Complex<f64>], incx: usize) {
    unsafe {
        raw::zscal_(&(n as int) as *const _,
                    a.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn zdscal(n: usize, a: &[Complex<f64>], x: &mut [Complex<f64>], incx: usize) {
    unsafe {
        raw::zdscal_(&(n as int) as *const _,
                     a.as_ptr() as *const _,
                     x.as_mut_ptr() as *mut _,
                     &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn zcopy(n: usize, x: &[Complex<f64>], incx: usize, y: &mut [Complex<f64>], incy: usize) {
    unsafe {
        raw::zcopy_(&(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zaxpy(n: usize, alpha: Complex<f64>, x: &[Complex<f64>], incx: usize,
             y: &mut [Complex<f64>], incy: usize) {

    unsafe {
        raw::zaxpy_(&(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zdotu(pres: &mut [Complex<f64>], n: usize, x: &[Complex<f64>], incx: usize,
             y: &[Complex<f64>], incy: usize) {

    unsafe {
        raw::zdotu_(pres.as_mut_ptr() as *mut _,
                    &(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zdotc(pres: &mut [Complex<f64>], n: usize, x: &[Complex<f64>], incx: usize,
             y: &[Complex<f64>], incy: usize) {

    unsafe {
        raw::zdotc_(pres.as_mut_ptr() as *mut _,
                    &(n as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn dzasum(n: usize, x: &[Complex<f64>], incx: usize) -> f64 {
    unsafe {
        raw::dzasum_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as f64
    }
}

#[inline]
pub fn izamax(n: usize, x: &[Complex<f64>], incx: usize) -> isize {
    unsafe {
        raw::izamax_(&(n as int) as *const _,
                     x.as_ptr() as *const _,
                     &(incx as int) as *const _,
        ) as isize
    }
}

#[inline]
pub fn sgemv(trans: Trans, m: usize, n: usize, alpha: f32, a: &[f32], lda: usize, x: &[f32],
             incx: usize, beta: f32, y: &mut [f32], incy: usize) {

    unsafe {
        raw::sgemv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn sgbmv(trans: Trans, m: usize, n: usize, kl: usize, ku: usize, alpha: f32, a: &[f32],
             lda: usize, x: &[f32], incx: usize, beta: f32, y: &mut [f32], incy: usize) {

    unsafe {
        raw::sgbmv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(kl as int) as *const _,
                    &(ku as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn ssymv(uplo: Uplo, n: usize, alpha: f32, a: &[f32], lda: usize, x: &[f32], incx: usize,
             beta: f32, y: &mut [f32], incy: usize) {

    unsafe {
        raw::ssymv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn ssbmv(uplo: Uplo, n: usize, k: usize, alpha: f32, a: &[f32], lda: usize, x: &[f32],
             incx: usize, beta: f32, y: &mut [f32], incy: usize) {

    unsafe {
        raw::ssbmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn sspmv(uplo: Uplo, n: usize, alpha: f32, ap: &[f32], x: &[f32], incx: usize, beta: f32,
             y: &mut [f32], incy: usize) {

    unsafe {
        raw::sspmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    ap.as_ptr() as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn strmv(uplo: Uplo, transa: Trans, diag: Diag, n: usize, a: &[f32], lda: usize, b: &mut [f32],
             incx: usize) {

    unsafe {
        raw::strmv_(&(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn stbmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[f32], lda: usize,
             x: &mut [f32], incx: usize) {

    unsafe {
        raw::stbmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn stpmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[f32], x: &mut [f32],
             incx: usize) {

    unsafe {
        raw::stpmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn strsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, a: &[f32], lda: usize, x: &mut [f32],
             incx: usize) {

    unsafe {
        raw::strsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn stbsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[f32], lda: usize,
             x: &mut [f32], incx: usize) {

    unsafe {
        raw::stbsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn stpsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[f32], x: &mut [f32],
             incx: usize) {

    unsafe {
        raw::stpsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn sger(m: usize, n: usize, alpha: f32, x: &[f32], incx: usize, y: &[f32], incy: usize,
            a: &mut [f32], lda: usize) {

    unsafe {
        raw::sger_(&(m as int) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   y.as_ptr() as *const _,
                   &(incy as int) as *const _,
                   a.as_mut_ptr() as *mut _,
                   &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn ssyr(uplo: Uplo, n: usize, alpha: f32, x: &[f32], incx: usize, a: &mut [f32], lda: usize) {
    unsafe {
        raw::ssyr_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   a.as_mut_ptr() as *mut _,
                   &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn sspr(uplo: Uplo, n: usize, alpha: f32, x: &[f32], incx: usize, ap: &mut [f32]) {
    unsafe {
        raw::sspr_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn ssyr2(uplo: Uplo, n: usize, alpha: f32, x: &[f32], incx: usize, y: &[f32], incy: usize,
             a: &mut [f32], lda: usize) {

    unsafe {
        raw::ssyr2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn sspr2(uplo: Uplo, n: usize, alpha: f32, x: &[f32], incx: usize, y: &[f32], incy: usize,
             ap: &mut [f32]) {

    unsafe {
        raw::sspr2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn dgemv(trans: Trans, m: usize, n: usize, alpha: f64, a: &[f64], lda: usize, x: &[f64],
             incx: usize, beta: f64, y: &mut [f64], incy: usize) {

    unsafe {
        raw::dgemv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn dgbmv(trans: Trans, m: usize, n: usize, kl: usize, ku: usize, alpha: f64, a: &[f64],
             lda: usize, x: &[f64], incx: usize, beta: f64, y: &mut [f64], incy: usize) {

    unsafe {
        raw::dgbmv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(kl as int) as *const _,
                    &(ku as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn dsymv(uplo: Uplo, n: usize, alpha: f64, a: &[f64], lda: usize, x: &[f64], incx: usize,
             beta: f64, y: &mut [f64], incy: usize) {

    unsafe {
        raw::dsymv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn dsbmv(uplo: Uplo, n: usize, k: usize, alpha: f64, a: &[f64], lda: usize, x: &[f64],
             incx: usize, beta: f64, y: &mut [f64], incy: usize) {

    unsafe {
        raw::dsbmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn dspmv(uplo: Uplo, n: usize, alpha: f64, ap: &[f64], x: &[f64], incx: usize, beta: f64,
             y: &mut [f64], incy: usize) {

    unsafe {
        raw::dspmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    ap.as_ptr() as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn dtrmv(uplo: Uplo, transa: Trans, diag: Diag, n: usize, a: &[f64], lda: usize, b: &mut [f64],
             incx: usize) {

    unsafe {
        raw::dtrmv_(&(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn dtbmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[f64], lda: usize,
             x: &mut [f64], incx: usize) {

    unsafe {
        raw::dtbmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn dtpmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[f64], x: &mut [f64],
             incx: usize) {

    unsafe {
        raw::dtpmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn dtrsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, a: &[f64], lda: usize, x: &mut [f64],
             incx: usize) {

    unsafe {
        raw::dtrsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn dtbsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[f64], lda: usize,
             x: &mut [f64], incx: usize) {

    unsafe {
        raw::dtbsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn dtpsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[f64], x: &mut [f64],
             incx: usize) {

    unsafe {
        raw::dtpsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn dger(m: usize, n: usize, alpha: f64, x: &[f64], incx: usize, y: &[f64], incy: usize,
            a: &mut [f64], lda: usize) {

    unsafe {
        raw::dger_(&(m as int) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   y.as_ptr() as *const _,
                   &(incy as int) as *const _,
                   a.as_mut_ptr() as *mut _,
                   &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn dsyr(uplo: Uplo, n: usize, alpha: f64, x: &[f64], incx: usize, a: &mut [f64], lda: usize) {
    unsafe {
        raw::dsyr_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   a.as_mut_ptr() as *mut _,
                   &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn dspr(uplo: Uplo, n: usize, alpha: f64, x: &[f64], incx: usize, ap: &mut [f64]) {
    unsafe {
        raw::dspr_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn dsyr2(uplo: Uplo, n: usize, alpha: f64, x: &[f64], incx: usize, y: &[f64], incy: usize,
             a: &mut [f64], lda: usize) {

    unsafe {
        raw::dsyr2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn dspr2(uplo: Uplo, n: usize, alpha: f64, x: &[f64], incx: usize, y: &[f64], incy: usize,
             ap: &mut [f64]) {

    unsafe {
        raw::dspr2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn cgemv(trans: Trans, m: usize, n: usize, alpha: Complex<f32>, a: &[Complex<f32>], lda: usize,
             x: &[Complex<f32>], incx: usize, beta: Complex<f32>, y: &mut [Complex<f32>],
             incy: usize) {

    unsafe {
        raw::cgemv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn cgbmv(trans: Trans, m: usize, n: usize, kl: usize, ku: usize, alpha: Complex<f32>,
             a: &[Complex<f32>], lda: usize, x: &[Complex<f32>], incx: usize, beta: Complex<f32>,
             y: &mut [Complex<f32>], incy: usize) {

    unsafe {
        raw::cgbmv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(kl as int) as *const _,
                    &(ku as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn chemv(uplo: Uplo, n: usize, alpha: Complex<f32>, a: &[Complex<f32>], lda: usize,
             x: &[Complex<f32>], incx: usize, beta: Complex<f32>, y: &mut [Complex<f32>],
             incy: usize) {

    unsafe {
        raw::chemv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn chbmv(uplo: Uplo, n: usize, k: usize, alpha: Complex<f32>, a: &[Complex<f32>], lda: usize,
             x: &[Complex<f32>], incx: usize, beta: Complex<f32>, y: &mut [Complex<f32>],
             incy: usize) {

    unsafe {
        raw::chbmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn chpmv(uplo: Uplo, n: usize, alpha: Complex<f32>, ap: &[Complex<f32>], x: &[Complex<f32>],
             incx: usize, beta: Complex<f32>, y: &mut [Complex<f32>], incy: usize) {

    unsafe {
        raw::chpmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    ap.as_ptr() as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn ctrmv(uplo: Uplo, transa: Trans, diag: Diag, n: usize, a: &[Complex<f32>], lda: usize,
             b: &mut [Complex<f32>], incx: usize) {

    unsafe {
        raw::ctrmv_(&(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ctbmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[Complex<f32>],
             lda: usize, x: &mut [Complex<f32>], incx: usize) {

    unsafe {
        raw::ctbmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ctpmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[Complex<f32>],
             x: &mut [Complex<f32>], incx: usize) {

    unsafe {
        raw::ctpmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ctrsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, a: &[Complex<f32>], lda: usize,
             x: &mut [Complex<f32>], incx: usize) {

    unsafe {
        raw::ctrsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ctbsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[Complex<f32>],
             lda: usize, x: &mut [Complex<f32>], incx: usize) {

    unsafe {
        raw::ctbsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ctpsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[Complex<f32>],
             x: &mut [Complex<f32>], incx: usize) {

    unsafe {
        raw::ctpsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn cgeru(m: usize, n: usize, alpha: Complex<f32>, x: &[Complex<f32>], incx: usize,
             y: &[Complex<f32>], incy: usize, a: &mut [Complex<f32>], lda: usize) {

    unsafe {
        raw::cgeru_(&(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn cgerc(m: usize, n: usize, alpha: Complex<f32>, x: &[Complex<f32>], incx: usize,
             y: &[Complex<f32>], incy: usize, a: &mut [Complex<f32>], lda: usize) {

    unsafe {
        raw::cgerc_(&(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn cher(uplo: Uplo, n: usize, alpha: Complex<f32>, x: &[Complex<f32>], incx: usize,
            a: &mut [Complex<f32>], lda: usize) {

    unsafe {
        raw::cher_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   a.as_mut_ptr() as *mut _,
                   &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn chpr(uplo: Uplo, n: usize, alpha: Complex<f32>, x: &[Complex<f32>], incx: usize,
            ap: &mut [Complex<f32>]) {

    unsafe {
        raw::chpr_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn chpr2(uplo: Uplo, n: usize, alpha: Complex<f32>, x: &[Complex<f32>], incx: usize,
             y: &[Complex<f32>], incy: usize, ap: &mut [Complex<f32>]) {

    unsafe {
        raw::chpr2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn cher2(uplo: Uplo, n: usize, alpha: Complex<f32>, x: &[Complex<f32>], incx: usize,
             y: &[Complex<f32>], incy: usize, a: &mut [Complex<f32>], lda: usize) {

    unsafe {
        raw::cher2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn zgemv(trans: Trans, m: usize, n: usize, alpha: Complex<f64>, a: &[Complex<f64>], lda: usize,
             x: &[Complex<f64>], incx: usize, beta: Complex<f64>, y: &mut [Complex<f64>],
             incy: usize) {

    unsafe {
        raw::zgemv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zgbmv(trans: Trans, m: usize, n: usize, kl: usize, ku: usize, alpha: Complex<f64>,
             a: &[Complex<f64>], lda: usize, x: &[Complex<f64>], incx: usize, beta: Complex<f64>,
             y: &mut [Complex<f64>], incy: usize) {

    unsafe {
        raw::zgbmv_(&(trans as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(kl as int) as *const _,
                    &(ku as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zhemv(uplo: Uplo, n: usize, alpha: Complex<f64>, a: &[Complex<f64>], lda: usize,
             x: &[Complex<f64>], incx: usize, beta: Complex<f64>, y: &mut [Complex<f64>],
             incy: usize) {

    unsafe {
        raw::zhemv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zhbmv(uplo: Uplo, n: usize, k: usize, alpha: Complex<f64>, a: &[Complex<f64>], lda: usize,
             x: &[Complex<f64>], incx: usize, beta: Complex<f64>, y: &mut [Complex<f64>],
             incy: usize) {

    unsafe {
        raw::zhbmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn zhpmv(uplo: Uplo, n: usize, alpha: Complex<f64>, ap: &[Complex<f64>], x: &[Complex<f64>],
             incx: usize, beta: Complex<f64>, y: &mut [Complex<f64>], incy: usize) {

    unsafe {
        raw::zhpmv_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    ap.as_ptr() as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    &beta as *const _ as *const _,
                    y.as_mut_ptr() as *mut _,
                    &(incy as int) as *const _,
        )
    }
}

#[inline]
pub fn ztrmv(uplo: Uplo, transa: Trans, diag: Diag, n: usize, a: &[Complex<f64>], lda: usize,
             b: &mut [Complex<f64>], incx: usize) {

    unsafe {
        raw::ztrmv_(&(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ztbmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[Complex<f64>],
             lda: usize, x: &mut [Complex<f64>], incx: usize) {

    unsafe {
        raw::ztbmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ztpmv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[Complex<f64>],
             x: &mut [Complex<f64>], incx: usize) {

    unsafe {
        raw::ztpmv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ztrsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, a: &[Complex<f64>], lda: usize,
             x: &mut [Complex<f64>], incx: usize) {

    unsafe {
        raw::ztrsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ztbsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, k: usize, a: &[Complex<f64>],
             lda: usize, x: &mut [Complex<f64>], incx: usize) {

    unsafe {
        raw::ztbsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn ztpsv(uplo: Uplo, trans: Trans, diag: Diag, n: usize, ap: &[Complex<f64>],
             x: &mut [Complex<f64>], incx: usize) {

    unsafe {
        raw::ztpsv_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(n as int) as *const _,
                    ap.as_ptr() as *const _,
                    x.as_mut_ptr() as *mut _,
                    &(incx as int) as *const _,
        )
    }
}

#[inline]
pub fn zgeru(m: usize, n: usize, alpha: Complex<f64>, x: &[Complex<f64>], incx: usize,
             y: &[Complex<f64>], incy: usize, a: &mut [Complex<f64>], lda: usize) {

    unsafe {
        raw::zgeru_(&(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn zgerc(m: usize, n: usize, alpha: Complex<f64>, x: &[Complex<f64>], incx: usize,
             y: &[Complex<f64>], incy: usize, a: &mut [Complex<f64>], lda: usize) {

    unsafe {
        raw::zgerc_(&(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn zher(uplo: Uplo, n: usize, alpha: Complex<f64>, x: &[Complex<f64>], incx: usize,
            a: &mut [Complex<f64>], lda: usize) {

    unsafe {
        raw::zher_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   a.as_mut_ptr() as *mut _,
                   &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn zhpr(uplo: Uplo, n: usize, alpha: Complex<f64>, x: &[Complex<f64>], incx: usize,
            ap: &mut [Complex<f64>]) {

    unsafe {
        raw::zhpr_(&(uplo as c_char) as *const _,
                   &(n as int) as *const _,
                   &alpha as *const _ as *const _,
                   x.as_ptr() as *const _,
                   &(incx as int) as *const _,
                   ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn zher2(uplo: Uplo, n: usize, alpha: Complex<f64>, x: &[Complex<f64>], incx: usize,
             y: &[Complex<f64>], incy: usize, a: &mut [Complex<f64>], lda: usize) {

    unsafe {
        raw::zher2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    a.as_mut_ptr() as *mut _,
                    &(lda as int) as *const _,
        )
    }
}

#[inline]
pub fn zhpr2(uplo: Uplo, n: usize, alpha: Complex<f64>, x: &[Complex<f64>], incx: usize,
             y: &[Complex<f64>], incy: usize, ap: &mut [Complex<f64>]) {

    unsafe {
        raw::zhpr2_(&(uplo as c_char) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    x.as_ptr() as *const _,
                    &(incx as int) as *const _,
                    y.as_ptr() as *const _,
                    &(incy as int) as *const _,
                    ap.as_mut_ptr() as *mut _,
        )
    }
}

#[inline]
pub fn sgemm(transa: Trans, transb: Trans, m: usize, n: usize, k: usize, alpha: f32, a: &[f32],
             lda: usize, b: &[f32], ldb: usize, beta: f32, c: &mut [f32], ldc: usize) {

    unsafe {
        raw::sgemm_(&(transa as c_char) as *const _,
                    &(transb as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn ssymm(side: Side, uplo: Uplo, m: usize, n: usize, alpha: f32, a: &[f32], lda: usize,
             b: &[f32], ldb: usize, beta: f32, c: &mut [f32], ldc: usize) {

    unsafe {
        raw::ssymm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn ssyrk(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: f32, a: &[f32], lda: usize,
             beta: f32, c: &mut [f32], ldc: usize) {

    unsafe {
        raw::ssyrk_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn ssyr2k(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: f32, a: &[f32], lda: usize,
              b: &[f32], ldb: usize, beta: f32, c: &mut [f32], ldc: usize) {

    unsafe {
        raw::ssyr2k_(&(uplo as c_char) as *const _,
                     &(trans as c_char) as *const _,
                     &(n as int) as *const _,
                     &(k as int) as *const _,
                     &alpha as *const _ as *const _,
                     a.as_ptr() as *const _,
                     &(lda as int) as *const _,
                     b.as_ptr() as *const _,
                     &(ldb as int) as *const _,
                     &beta as *const _ as *const _,
                     c.as_mut_ptr() as *mut _,
                     &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn strmm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize, alpha: f32,
             a: &[f32], lda: usize, b: &mut [f32], ldb: usize) {

    unsafe {
        raw::strmm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}

#[inline]
pub fn strsm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize, alpha: f32,
             a: &[f32], lda: usize, b: &mut [f32], ldb: usize) {

    unsafe {
        raw::strsm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}

#[inline]
pub fn dgemm(transa: Trans, transb: Trans, m: usize, n: usize, k: usize, alpha: f64, a: &[f64],
             lda: usize, b: &[f64], ldb: usize, beta: f64, c: &mut [f64], ldc: usize) {

    unsafe {
        raw::dgemm_(&(transa as c_char) as *const _,
                    &(transb as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn dsymm(side: Side, uplo: Uplo, m: usize, n: usize, alpha: f64, a: &[f64], lda: usize,
             b: &[f64], ldb: usize, beta: f64, c: &mut [f64], ldc: usize) {

    unsafe {
        raw::dsymm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn dsyrk(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: f64, a: &[f64], lda: usize,
             beta: f64, c: &mut [f64], ldc: usize) {

    unsafe {
        raw::dsyrk_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn dsyr2k(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: f64, a: &[f64], lda: usize,
              b: &[f64], ldb: usize, beta: f64, c: &mut [f64], ldc: usize) {

    unsafe {
        raw::dsyr2k_(&(uplo as c_char) as *const _,
                     &(trans as c_char) as *const _,
                     &(n as int) as *const _,
                     &(k as int) as *const _,
                     &alpha as *const _ as *const _,
                     a.as_ptr() as *const _,
                     &(lda as int) as *const _,
                     b.as_ptr() as *const _,
                     &(ldb as int) as *const _,
                     &beta as *const _ as *const _,
                     c.as_mut_ptr() as *mut _,
                     &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn dtrmm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize, alpha: f64,
             a: &[f64], lda: usize, b: &mut [f64], ldb: usize) {

    unsafe {
        raw::dtrmm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}

#[inline]
pub fn dtrsm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize, alpha: f64,
             a: &[f64], lda: usize, b: &mut [f64], ldb: usize) {

    unsafe {
        raw::dtrsm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}

#[inline]
pub fn cgemm(transa: Trans, transb: Trans, m: usize, n: usize, k: usize, alpha: Complex<f32>,
             a: &[Complex<f32>], lda: usize, b: &[Complex<f32>], ldb: usize, beta: Complex<f32>,
             c: &mut [Complex<f32>], ldc: usize) {

    unsafe {
        raw::cgemm_(&(transa as c_char) as *const _,
                    &(transb as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn csymm(side: Side, uplo: Uplo, m: usize, n: usize, alpha: Complex<f32>, a: &[Complex<f32>],
             lda: usize, b: &[Complex<f32>], ldb: usize, beta: Complex<f32>,
             c: &mut [Complex<f32>], ldc: usize) {

    unsafe {
        raw::csymm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn chemm(side: Side, uplo: Uplo, m: usize, n: usize, alpha: Complex<f32>, a: &[Complex<f32>],
             lda: usize, b: &[Complex<f32>], ldb: usize, beta: Complex<f32>,
             c: &mut [Complex<f32>], ldc: usize) {

    unsafe {
        raw::chemm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn csyrk(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f32>, a: &[Complex<f32>],
             lda: usize, beta: Complex<f32>, c: &mut [Complex<f32>], ldc: usize) {

    unsafe {
        raw::csyrk_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn cherk(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f32>, a: &[Complex<f32>],
             lda: usize, beta: Complex<f32>, c: &mut [Complex<f32>], ldc: usize) {

    unsafe {
        raw::cherk_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn csyr2k(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f32>,
              a: &[Complex<f32>], lda: usize, b: &[Complex<f32>], ldb: usize, beta: Complex<f32>,
              c: &mut [Complex<f32>], ldc: usize) {

    unsafe {
        raw::csyr2k_(&(uplo as c_char) as *const _,
                     &(trans as c_char) as *const _,
                     &(n as int) as *const _,
                     &(k as int) as *const _,
                     &alpha as *const _ as *const _,
                     a.as_ptr() as *const _,
                     &(lda as int) as *const _,
                     b.as_ptr() as *const _,
                     &(ldb as int) as *const _,
                     &beta as *const _ as *const _,
                     c.as_mut_ptr() as *mut _,
                     &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn cher2k(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f32>,
              a: &[Complex<f32>], lda: usize, b: &[Complex<f32>], ldb: usize, beta: Complex<f32>,
              c: &mut [Complex<f32>], ldc: usize) {

    unsafe {
        raw::cher2k_(&(uplo as c_char) as *const _,
                     &(trans as c_char) as *const _,
                     &(n as int) as *const _,
                     &(k as int) as *const _,
                     &alpha as *const _ as *const _,
                     a.as_ptr() as *const _,
                     &(lda as int) as *const _,
                     b.as_ptr() as *const _,
                     &(ldb as int) as *const _,
                     &beta as *const _ as *const _,
                     c.as_mut_ptr() as *mut _,
                     &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn ctrmm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize,
             alpha: Complex<f32>, a: &[Complex<f32>], lda: usize, b: &mut [Complex<f32>],
             ldb: usize) {

    unsafe {
        raw::ctrmm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}

#[inline]
pub fn ctrsm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize,
             alpha: Complex<f32>, a: &[Complex<f32>], lda: usize, b: &mut [Complex<f32>],
             ldb: usize) {

    unsafe {
        raw::ctrsm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}

#[inline]
pub fn zgemm(transa: Trans, transb: Trans, m: usize, n: usize, k: usize, alpha: Complex<f64>,
             a: &[Complex<f64>], lda: usize, b: &[Complex<f64>], ldb: usize, beta: Complex<f64>,
             c: &mut [Complex<f64>], ldc: usize) {

    unsafe {
        raw::zgemm_(&(transa as c_char) as *const _,
                    &(transb as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn zsymm(side: Side, uplo: Uplo, m: usize, n: usize, alpha: Complex<f64>, a: &[Complex<f64>],
             lda: usize, b: &[Complex<f64>], ldb: usize, beta: Complex<f64>,
             c: &mut [Complex<f64>], ldc: usize) {

    unsafe {
        raw::zsymm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn zhemm(side: Side, uplo: Uplo, m: usize, n: usize, alpha: Complex<f64>, a: &[Complex<f64>],
             lda: usize, b: &[Complex<f64>], ldb: usize, beta: Complex<f64>,
             c: &mut [Complex<f64>], ldc: usize) {

    unsafe {
        raw::zhemm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_ptr() as *const _,
                    &(ldb as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn zsyrk(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f64>, a: &[Complex<f64>],
             lda: usize, beta: Complex<f64>, c: &mut [Complex<f64>], ldc: usize) {

    unsafe {
        raw::zsyrk_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn zherk(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f64>, a: &[Complex<f64>],
             lda: usize, beta: Complex<f64>, c: &mut [Complex<f64>], ldc: usize) {

    unsafe {
        raw::zherk_(&(uplo as c_char) as *const _,
                    &(trans as c_char) as *const _,
                    &(n as int) as *const _,
                    &(k as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    &beta as *const _ as *const _,
                    c.as_mut_ptr() as *mut _,
                    &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn zsyr2k(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f64>,
              a: &[Complex<f64>], lda: usize, b: &[Complex<f64>], ldb: usize, beta: Complex<f64>,
              c: &mut [Complex<f64>], ldc: usize) {

    unsafe {
        raw::zsyr2k_(&(uplo as c_char) as *const _,
                     &(trans as c_char) as *const _,
                     &(n as int) as *const _,
                     &(k as int) as *const _,
                     &alpha as *const _ as *const _,
                     a.as_ptr() as *const _,
                     &(lda as int) as *const _,
                     b.as_ptr() as *const _,
                     &(ldb as int) as *const _,
                     &beta as *const _ as *const _,
                     c.as_mut_ptr() as *mut _,
                     &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn zher2k(uplo: Uplo, trans: Trans, n: usize, k: usize, alpha: Complex<f64>,
              a: &[Complex<f64>], lda: usize, b: &[Complex<f64>], ldb: usize, beta: Complex<f64>,
              c: &mut [Complex<f64>], ldc: usize) {

    unsafe {
        raw::zher2k_(&(uplo as c_char) as *const _,
                     &(trans as c_char) as *const _,
                     &(n as int) as *const _,
                     &(k as int) as *const _,
                     &alpha as *const _ as *const _,
                     a.as_ptr() as *const _,
                     &(lda as int) as *const _,
                     b.as_ptr() as *const _,
                     &(ldb as int) as *const _,
                     &beta as *const _ as *const _,
                     c.as_mut_ptr() as *mut _,
                     &(ldc as int) as *const _,
        )
    }
}

#[inline]
pub fn ztrmm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize,
             alpha: Complex<f64>, a: &[Complex<f64>], lda: usize, b: &mut [Complex<f64>],
             ldb: usize) {

    unsafe {
        raw::ztrmm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}

#[inline]
pub fn ztrsm(side: Side, uplo: Uplo, transa: Trans, diag: Diag, m: usize, n: usize,
             alpha: Complex<f64>, a: &[Complex<f64>], lda: usize, b: &mut [Complex<f64>],
             ldb: usize) {

    unsafe {
        raw::ztrsm_(&(side as c_char) as *const _,
                    &(uplo as c_char) as *const _,
                    &(transa as c_char) as *const _,
                    &(diag as c_char) as *const _,
                    &(m as int) as *const _,
                    &(n as int) as *const _,
                    &alpha as *const _ as *const _,
                    a.as_ptr() as *const _,
                    &(lda as int) as *const _,
                    b.as_mut_ptr() as *mut _,
                    &(ldb as int) as *const _,
        )
    }
}