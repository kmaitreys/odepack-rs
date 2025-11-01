use std::ffi::{c_double, c_int};

#[link(name = "gfortran")]
unsafe extern "C" {
    /// Call `DLSODE` subroutine from ODEPACK
    ///
    /// For info on passed arguments look inside ODEPACK.
    pub fn dlsode_(
        f: extern "C" fn(*const c_int, *const c_double, *mut c_double, *mut c_double),
        neq: &c_int,
        y: *mut c_double,
        t: &mut c_double,
        tout: &c_double,
        itol: &c_int,
        rtol: &c_double,
        atol: &c_double,
        itask: &c_int,
        istate: &mut c_int,
        iopt: &c_int,
        rwork: *mut c_double,
        lrw: &c_int,
        iwork: *mut c_int,
        liw: &c_int,
        jac: extern "C" fn(
            &c_int,
            &c_double,
            *mut c_double,
            &c_int,
            &c_int,
            *const c_double,
            &c_int,
        ),
        mf: &c_int,
    );
}

#[link(name = "gfortran")]
unsafe extern "C" {
    /// Call `DLSODES` subroutine from ODEPACK
    ///
    /// For info on passed arguments look inside ODEPACK.
    pub fn dlsodes_(
        f: unsafe extern "C" fn(*const c_int, *const c_double, *mut c_double, *mut c_double),
        neq: &c_int,
        y: *mut c_double,
        t: &mut c_double,
        tout: &c_double,
        itol: &c_int,
        rtol: *const c_double,
        atol: *const c_double,
        itask: &c_int,
        istate: &mut c_int,
        iopt: &c_int,
        rwork: *mut c_double,
        lrw: &c_int,
        iwork: *mut c_int,
        liw: &c_int,
        jac: unsafe extern "C" fn(
            &c_int,
            &c_double,
            *mut c_double,
            &c_int,
            *mut c_double,
            *mut c_double,
            *mut c_double,
        ),
        mf: &c_int,
    );
}
