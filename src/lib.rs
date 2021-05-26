#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::{palCaldj, palMap, palMappa};

    #[test]
    fn test_caldj() {
        let mut j: ::std::os::raw::c_int = -1;
        let mut djm: f64 = 0.0;

        unsafe {
            palCaldj(1999, 12, 31, &mut djm, &mut j);
        }

        assert_eq!(djm, 51543.0);
        assert_eq!(j, 0);
    }

    // TODO: relative tolerance macro
    // macro_rules! assert_in_rel_tolerance {}

    macro_rules! assert_in_abs_tolerance {
        ($left:expr, $right:expr, $abs:expr) => {
            assert!(($right - $left).abs() < $abs);
        };
    }

    #[test]
    fn test_map() {
        let mut ra: f64 = 0.0;
        let mut da: f64 = 0.0;
        unsafe {
            palMap(
                6.123, -0.999, 1.23e-5, -0.987e-5, 0.123, 32.1, 1999.0, 43210.9, &mut ra, &mut da,
            );
        }

        /* These are the SLA tests but and they agree to 0.1 arcsec
        with PAL/SOFA/ERFA. We expect a slight difference from the change
        to nutation models. */

        assert_in_abs_tolerance!(ra, 6.117130429775647, 1e-6);
        assert_in_abs_tolerance!(da, -1.000880769038632, 1e-8);
    }

    #[test]
    fn test_mappa() {
        let mut amprms: [f64; 21] = [0.0; 21];
        let mut expected: [f64; 21] = [
            1.9986310746064646082,
            -0.1728200754134739392,
            0.88745394651412767839,
            0.38472374350184274094,
            -0.17245634725219796679,
            0.90374808622520386159,
            0.3917884696321610738,
            2.0075929387510784968e-08,
            -9.9464149073251757597e-05,
            -1.6125306981057062306e-05,
            -6.9897255793245634435e-06,
            0.99999999489900059935,
            0.99999983777998024959,
            -0.00052248206600935195865,
            -0.00022683144398381763045,
            0.00052248547063364874764,
            0.99999986339269864022,
            1.4950491424992534218e-05,
            0.00022682360163333854623,
            -1.5069005133483779417e-05,
            0.99999997416198904698,
        ];

        unsafe {
            palMappa(2010.0, 55927.0, amprms.as_mut_ptr());
        }
        for (amprm, exp) in amprms.iter().zip(expected.iter()) {
            assert_eq!(amprm, exp);
        }
    }
}
