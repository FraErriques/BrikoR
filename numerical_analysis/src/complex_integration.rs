///  \brief  Contour-Integrals in the Complex plane via Riemann partition
///  \author F. Erriques (Translated to Rust)

//the present namespace is intended to provide Contour-Integrals in the Complex plane, as in:
//Integrate[f(x+I*y)dz]==Integrate[ (u(x+I*y)+I*v(x+I*y))*(dx+I*dy)]==
//Integrate[(u,v)(dx,dy)]== Integrate[u_dx-v_dy +I*(u_dy+v_dx)]==
//Integrate[u(x,y)_dx]-Integrate[v(x,y)_dy] +I*{Integrate[u(x,y)_dy]+Integrate[v(x,y)_dx]}
//Oss. In the last two lines the notation Integrate[f()_dm] has been assumed, to indicate that the CoChain f()
//is integrated in the Measure dm.
//for the purposes of Numerical Analysis, a single Integrate[f(z)_dz] leads to four real integrals, as shown
//three lines above.
//The Chain has to be parametrized, in a single real variable (it's a one dimensional variety) and the CoChain has
//to be restricted to the Chain(on each smooth portion of it).
//The differentials of the coordinate-functions of the chain, will introduce measure terms.
//Eg. u(z)==u(x+I*y)==u(x[t]+I*y[t]) -> u(z)_dx==u(x[t]+I*y[t])_x'[t]dt
//where the factor x'[t] is a coonsequence of the Diffeomorphism of integrating in dx==d(x[t])==x'[t]dt
//The subject matter of the Diffeomorphism of u(z)_dx has to applied to all four Real integrals:
// { u_dx,u_dy, v_dx,v_dy}
//
//notes from Wiki:
//Calcolare gli integrali con metodi diretti significa calcolarli con metodi simili a quelli usati per gli
//integrali curvilinei di funzioni a più variabili. Ciò significa che usiamo il seguente metodo:
//    - parametrizzazione del contorno
//        Il contorno è parametrizzato da una funzione derivabile che manda numeri reali in numeri complessi, oppure il
//        contorno viene diviso in pezzi che vengono parametrizzati separatamente.
//    - sostituzione della parametrizzazione nell'integranda
//        Sostituire la parametrizzazione nell'integranda trasforma l'integrale in uno di una variabile reale.
// la nota riporta inoltre il metodo di cercare la primitiva ed applicare il Thm di Stokes; cosa non praticabile in un contesto
// di AnalisiNumerica, nel quale non sia disponibile un parser e l'algoritmo di Risch; cose queste che appartengono al calcolo simbolico,
// implementato da prodotti di Computer-Algebra, quali Wolfram-Mathematica. Nel contesto dell'Analisi Numerica si utilizzano invece discretizzazioni
// quali l'Integrale di Riemann.
//
//notes from Calculus2015 :
// Trapezium Integration. NB.: (u(x,y)+I*v(x,y) )*(dx+I*dy)==u*dx-v*dy + I*( u*dy+v*dx)
// Purpose of this routine -Trapezia- is to provide a numerical solution, suitable for the cases where no primitive can be found.
// The technique of Trapezia is to subdivide the domain in "n" equal Deltas and evaluate the integrand at the extrema of each trapezium
// instead of evaluating a primitive at the boundary, as in Stokes' theorems. Therefore the first step is to choose a contour, over which
// the integration will be performed. Due to Cauchy theorem, for olomorphic functions there's no involvment of the contour when integrating
// over omotopic paths. Nonetheless the utility of numerical methods -as Trapezia- is to allow the evaluation even of functions
// not endowed of an elementary primitive. For that purpose, we cannot rely on the behaviour (of the primitive) at the extrema. We have to
// explicitly follow a path that connects (in a Jordan way) the extrema. First step will be the choice of a contour and of its parametrization.
// In the C-plane the contour will be as L:{x(t)=f(t), y(t)=g(t)} and so dx=x'(t)dt , dy=y'(t)dt. Due to this, the two coordinate functions:
// u=u(x,y) and v(x,y) will have to be restricted to the path as in u=u(x(t),y(t)) , v=v(x(t),y(t))
// Therefore the classical:  (u(x,y)+I*v(x,y) )*(dx+I*dy)==u*dx-v*dy + I*( u*dy+v*dx)
// will become: u(x(t),y(t))*dx-v(x(t),y(t))*dy + I*( u(x(t),y(t))*dy+v(x(t),y(t))*dx) which will be
// u(x(t),y(t))*x'(t)dt-v(x(t),y(t))*y'(t)dt + I*( u(x(t),y(t))*y'(t)dt+v(x(t),y(t))*x'(t)dt)
// the two not-homogeneous quantities that must be calculated separately are:
//      u(x(t),y(t))*x'(t)dt-v(x(t),y(t))*y'(t)dt   for the Real-part
//      u(x(t),y(t))*y'(t)dt+v(x(t),y(t))*x'(t)dt   for the Immaginary-part
//
// The routine in this class is intended to be for elementary functions, so they will accept just the complex argument and no other parameter whatsoever.
// The user will have to provide function-pointers (i.e. delegates) to the following methods:
// double x(double t) it's the x-coordinate function -abscissa- of the parametrization for the Jordan curve, in the C-plane
// double y(double t) it's the y-coordinate function -ordinate- of the parametrization for the Jordan curve, in the C-plane
// double dx(double t) it's the x-differential; it has to contain everything that is a factor of dt es. dx=2*t*dt -> dx returns(2*t), accepting t.
// double dy(double t) it's the y-differential; it has to contain everything that is a factor of dt es. dy=3*Sin(t)*dt -> dy returns(3*Sin(t)), accepting t.
// The user will have to provide also function-pointers to:
// double u(double x, double y) which will be fed as u(x(t),y(t)) and the same for v(x(t),y(t)).
// This class instead will provide:
// a method double RealPartIntegral which performs: u*dx-v*dy in terms of u(x(t),y(t))*dx(t)-v(x(t),y(t))*dy(t)
// the RealPartIntegral method will accept: t_inf, t_sup, n_card_trapezia, f_pointer_x(t), f_pointer_y(t), f_pointer_dx(t), f_pointer_dy(t), f_pointer_u(x,y), f_pointer_v(x,y)
// a method double ImmaginaryPartIntegral which performs: I*( u*dy+v*dx) in terms of u(x(t),y(t))*dy(t)+v(x(t),y(t))*dx(t)
// the ImmaginaryPartIntegral method will accept: t_inf, t_sup, n_card_trapezia, f_pointer_x(t), f_pointer_y(t), f_pointer_dx(t), f_pointer_dy(t), f_pointer_u(x,y), f_pointer_v(x,y)
// NB. in future implementations the management method, which calls both {RealPartIntegral,ImmaginaryPartIntegral} could spawn separate threads for them and then join the results.
// So this class instead will provide a management method -the only public one- wich will have signature:
// ComplexField.Complex ContourIntegral(  t_inf, t_sup, n_card_trapezia, f_pointer_x(t), f_pointer_y(t), f_pointer_dx(t), f_pointer_dy(t), f_pointer_u(x,y), f_pointer_v(x,y) )
// It will call  both {Integrate_equi_trapezium_RealPart,Integrate_equi_trapezium_ImmaginaryPart}.
// Have fun.
//
// As an aid to successive implementation, I'm providing a set of the function that are needed by this algorithm. They are u,v,x,y,dx,dy
// the functions choosen for the example are f(z)=z which implies u(x,y)=x, v(x,y)=y; the choice for the contour is x(t)=t,y(t)=2*t+1,dx=dt which means dx=1
// dy=2*dt which means dy=2.



use crate::complex::Complex;
use crate::real::fabs;
use crate::real_matrix::RealMatrix;
use common::common_data::{Couple, Crash};

pub type FPtrJordanParametriz = fn(f64) -> f64;
pub type FPtrUOriV = fn(f64, f64) -> f64;
pub type FPtrComplexAsScalar = fn(f64, f64) -> Complex;

// Example functions
pub fn x_coord(t: f64) -> f64 { t }
pub fn y_coord(t: f64) -> f64 { 2.0 * t + 1.0 }
pub fn dx_diff(_t: f64) -> f64 { 1.0 }
pub fn dy_diff(_t: f64) -> f64 { 2.0 }

pub fn generic_integrand_u_part(x: f64, _y: f64) -> f64 { x }
pub fn generic_integrand_v_part(_x: f64, y: f64) -> f64 { y }

pub fn integrand_complex_as_scalar(x: f64, y: f64) -> Complex {
    Complex::new(x, y)
}

pub fn integrate_equi_trapezium_complex_image_as_scalar(
    t0: f64,
    tn: f64,
    complex_as_scalar: FPtrComplexAsScalar,
    x_coordinate: FPtrJordanParametriz,
    y_coordinate: FPtrJordanParametriz,
    dx_differential: FPtrJordanParametriz,
    dy_differential: FPtrJordanParametriz,
    n: u64,
) -> Complex {
    let delta_t = (tn - t0) / (n as f64);
    let mut t = t0 + delta_t;
    let mut res = Complex::new(0.0, 0.0);

    while t < tn {
        res = res + complex_as_scalar(x_coordinate(t), y_coordinate(t))
            * Complex::new(dx_differential(t), dy_differential(t));
        t += delta_t;
    }
    res = res * delta_t;
    let term0 = complex_as_scalar(x_coordinate(t0), y_coordinate(t0))
        * Complex::new(dx_differential(t0), dy_differential(t0));
    let termn = complex_as_scalar(x_coordinate(tn), y_coordinate(tn))
        * Complex::new(dx_differential(tn), dy_differential(tn));
    res = res + (term0 + termn) * 0.5 * delta_t;
    res
}

pub fn integrate_equi_trapezium_real_part(
    t0: f64,
    tn: f64,
    u_part: FPtrUOriV,
    v_part: FPtrUOriV,
    x_coordinate: FPtrJordanParametriz,
    y_coordinate: FPtrJordanParametriz,
    dx_differential: FPtrJordanParametriz,
    dy_differential: FPtrJordanParametriz,
    n: u64,
) -> f64 {
    let delta_t = (tn - t0) / (n as f64);
    let mut res = 0.0;
    let mut t = t0 + delta_t;

    while t < tn {
        res += u_part(x_coordinate(t), y_coordinate(t)) * dx_differential(t)
            - v_part(x_coordinate(t), y_coordinate(t)) * dy_differential(t);
        t += delta_t;
    }
    res *= delta_t;
    res += (u_part(x_coordinate(t0), y_coordinate(t0)) * dx_differential(t0)
        - v_part(x_coordinate(t0), y_coordinate(t0)) * dy_differential(t0)
        + u_part(x_coordinate(tn), y_coordinate(tn)) * dx_differential(tn)
        - v_part(x_coordinate(tn), y_coordinate(tn)) * dy_differential(tn))
        * 0.5
        * delta_t;
    res
}

pub fn integrate_equi_trapezium_imaginary_part(
    t0: f64,
    tn: f64,
    u_part: FPtrUOriV,
    v_part: FPtrUOriV,
    x_coordinate: FPtrJordanParametriz,
    y_coordinate: FPtrJordanParametriz,
    dx_differential: FPtrJordanParametriz,
    dy_differential: FPtrJordanParametriz,
    n: u64,
) -> f64 {
    let delta_t = (tn - t0) / (n as f64);
    let mut res = 0.0;
    let mut t = t0 + delta_t;

    while t < tn {
        res += u_part(x_coordinate(t), y_coordinate(t)) * dy_differential(t)
            + v_part(x_coordinate(t), y_coordinate(t)) * dx_differential(t);
        t += delta_t;
    }
    res *= delta_t;
    res += (u_part(x_coordinate(t0), y_coordinate(t0)) * dy_differential(t0)
        + v_part(x_coordinate(t0), y_coordinate(t0)) * dx_differential(t0)
        + u_part(x_coordinate(tn), y_coordinate(tn)) * dy_differential(tn)
        + v_part(x_coordinate(tn), y_coordinate(tn)) * dx_differential(tn))
        * 0.5
        * delta_t;
    res
}

pub fn extrema_check(
    z0: Complex,
    z1: Complex,
    t0: f64,
    tn: f64,
    x_coordinate: FPtrJordanParametriz,
    y_coordinate: FPtrJordanParametriz,
) -> bool {
    let local_epsilon = 1.0e-05;
    let z0_x_by_jordan = x_coordinate(t0);
    let z0_y_by_jordan = y_coordinate(t0);
    let z1_x_by_jordan = x_coordinate(tn);
    let z1_y_by_jordan = y_coordinate(tn);

    let delta_z0x = fabs(z0_x_by_jordan - z0.re());
    let delta_z0y = fabs(z0_y_by_jordan - z0.im());
    let delta_z1x = fabs(z1_x_by_jordan - z1.re());
    let delta_z1y = fabs(z1_y_by_jordan - z1.im());

    delta_z0x < local_epsilon
        && delta_z0y < local_epsilon
        && delta_z1x < local_epsilon
        && delta_z1y < local_epsilon
}

pub fn contour_integral_management_method(
    z0: Complex,
    z1: Complex,
    t0: f64,
    tn: f64,
    u_part: FPtrUOriV,
    v_part: FPtrUOriV,
    x_coordinate: FPtrJordanParametriz,
    y_coordinate: FPtrJordanParametriz,
    dx_differential: FPtrJordanParametriz,
    dy_differential: FPtrJordanParametriz,
    n: u64,
) -> Option<Complex> {
    if !extrema_check(z0, z1, t0, tn, x_coordinate, y_coordinate) {
        return None;
    }
    let real_res = integrate_equi_trapezium_real_part(
        t0,
        tn,
        u_part,
        v_part,
        x_coordinate,
        y_coordinate,
        dx_differential,
        dy_differential,
        n,
    );
    let imag_res = integrate_equi_trapezium_imaginary_part(
        t0,
        tn,
        u_part,
        v_part,
        x_coordinate,
        y_coordinate,
        dx_differential,
        dy_differential,
        n,
    );
    Some(Complex::new(real_res, imag_res))
}

pub fn contour_integral_as_scalar_management_method(
    z0: Complex,
    z1: Complex,
    t0: f64,
    tn: f64,
    complex_as_scalar: FPtrComplexAsScalar,
    x_coordinate: FPtrJordanParametriz,
    y_coordinate: FPtrJordanParametriz,
    dx_differential: FPtrJordanParametriz,
    dy_differential: FPtrJordanParametriz,
    n: u64,
) -> Option<Complex> {
    if !extrema_check(z0, z1, t0, tn, x_coordinate, y_coordinate) {
        return None;
    }
    Some(integrate_equi_trapezium_complex_image_as_scalar(
        t0,
        tn,
        complex_as_scalar,
        x_coordinate,
        y_coordinate,
        dx_differential,
        dy_differential,
        n,
    ))
}

pub fn contour_integral_as_scalar_jordan_linear_auto_detect(
    z0: Complex,
    z1: Complex,
    complex_as_scalar: FPtrComplexAsScalar,
    n: u64,
) -> Result<Complex, Crash> {
    let left = Couple {
        argument: z0.re(),
        image: z0.im(),
    };
    let right = Couple {
        argument: z1.re(),
        image: z1.im(),
    };

    let parametric_giacitura =
        crate::real::two_points_interpolation::linear_parametric(left, right);
    let mut mat_parametric_layout = RealMatrix::new(2, 2);
    let _ = mat_parametric_layout.insert(parametric_giacitura.alpha_x, 0, 0);
    let _ = mat_parametric_layout.insert(parametric_giacitura.beta_x, 0, 1);
    let _ = mat_parametric_layout.insert(parametric_giacitura.alpha_y, 1, 0);
    let _ = mat_parametric_layout.insert(parametric_giacitura.beta_y, 1, 1);

    let mut pullback_alpha;
    let mut pullback_beta;
    let mut res = Complex::new(0.0, 0.0);

    let m00 = mat_parametric_layout.get_at(0, 0).unwrap_or(0.0);
    let m11 = mat_parametric_layout.get_at(1, 1).unwrap_or(0.0);

    if fabs(m00) < 1e-20 && fabs(m11) < 1e-20 {
        pullback_alpha = left.image;
        pullback_beta = right.image;
    } else if fabs(m00 - 1.0) < 1e-20 {
        pullback_alpha = left.argument;
        pullback_beta = right.argument;
    } else {
        return Err(Crash::new("DEBUG: unknown matrix layout."));
    }

    if pullback_alpha > pullback_beta {
        std::mem::swap(&mut pullback_alpha, &mut pullback_beta);
        res = res * -1.0;
    }

    let t0x_t0y = crate::real::two_points_interpolation::parametric_linear_image(
        parametric_giacitura,
        pullback_alpha,
    )
    .unwrap_or([0.0, 0.0]);
    let tnx_tny = crate::real::two_points_interpolation::parametric_linear_image(
        parametric_giacitura,
        pullback_beta,
    )
    .unwrap_or([0.0, 0.0]);

    let local_epsilon = 1.0e-05;
    let extrema_adequacy = (t0x_t0y[0] - z0.re()).abs() < local_epsilon
        && (t0x_t0y[1] - z0.im()).abs() < local_epsilon
        && (tnx_tny[0] - z1.re()).abs() < local_epsilon
        && (tnx_tny[1] - z1.im()).abs() < local_epsilon;

    if !extrema_adequacy {
        return Err(Crash::new("Integration extrema do not match, between coChain and Jordan-path."));
    }

    let t0 = pullback_alpha;
    let tn = pullback_beta;
    let delta_t = (tn - t0) / (n as f64);
    let mut t = t0 + delta_t;

    while t < tn {
        let tix_tiy = crate::real::two_points_interpolation::parametric_linear_image(
            parametric_giacitura,
            t,
        );
        if let Some(pt) = tix_tiy {
            res = res
                + complex_as_scalar(pt[0], pt[1])
                    * Complex::new(
                        parametric_giacitura.alpha_x,
                        parametric_giacitura.alpha_y,
                    );
        }
        t += delta_t;
    }

    res = res * delta_t;
    let term0 = complex_as_scalar(t0x_t0y[0], t0x_t0y[1])
        * Complex::new(
            parametric_giacitura.alpha_x,
            parametric_giacitura.alpha_y,
        );
    let termn = complex_as_scalar(tnx_tny[0], tnx_tny[1])
        * Complex::new(
            parametric_giacitura.alpha_x,
            parametric_giacitura.alpha_y,
        );
    res = res + (term0 + termn) * 0.5 * delta_t;

    Ok(res)
}

pub fn exp_integral_ei_co_chain(x: f64, y: f64) -> Result<Complex, Crash> {
    let z = Complex::new(x, y);
    if z.length() < 1.0e-15 {
        return Err(Crash::new("too near to the Pole in z==0+i*0"));
    }
    let exp_z = z.exp_c();
    Ok(exp_z / z)
}

pub fn log_integral_co_chain(x: f64, y: f64) -> Result<Complex, Crash> {
    let z = Complex::new(x, y);
    if (z.im() < 1.0e-15 && z.re() < 0.0)
        || (z - Complex::new(1.0, 0.0)).length() < 1.0e-15
    {
        return Err(Crash::new("too near to the branch-cut R_minus or to the Essential-singularity (1,0)"));
    }
    Ok(Complex::new(1.0, 0.0) / z.ln_c().unwrap_or(Complex::new(0.0, 0.0)))
}