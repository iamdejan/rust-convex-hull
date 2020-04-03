use num_complex::Complex64;
use std::vec::Vec;

use rust_convex_hull::convex_hull;

fn are_polygons_same(mut left: Vec<Complex64>, mut right: Vec<Complex64>) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let eps = 1e-9;
    left.sort_unstable_by(|a, b| {
        if (a.re - b.re).abs() > eps {
            return a.re.partial_cmp(&b.re).unwrap();
        }
        return a.im.partial_cmp(&b.im).unwrap();
    });
    right.sort_unstable_by(|a, b| {
        if (a.re - b.re).abs() > eps {
            return a.re.partial_cmp(&b.re).unwrap();
        }
        return a.im.partial_cmp(&b.im).unwrap();
    });

    for i in 0..left.len() {
        if left[i] != right[i] {
            return false;
        }
    }

    return true;
}

#[test]
fn no_point() {
    let polygon: Vec<Complex64> = Vec::new();
    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.err(),
        Some("only 0 Point(s), should be at least 3".to_string())
    );
}

#[test]
fn one_point() {
    let point: Complex64 = Complex64::new(0.0, 0.0);
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(point);

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.err(),
        Some("only 1 Point(s), should be at least 3".to_string())
    );
}

#[test]
fn two_points() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.err(),
        Some("only 2 Point(s), should be at least 3".to_string())
    );
}

#[test]
fn three_points_convex() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));
    polygon.push(Complex64::new(0.0, 0.1));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn three_points_colinear_horizontal() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));
    polygon.push(Complex64::new(0.2, 0.0));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn three_points_colinear_vertical() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.0, 0.1));
    polygon.push(Complex64::new(0.0, 0.2));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn four_points_colinear_horizontal() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.1, 0.0));
    polygon.push(Complex64::new(0.2, 0.0));
    polygon.push(Complex64::new(0.3, 0.0));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn four_points_colinear_vertical() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.0, 0.0));
    polygon.push(Complex64::new(0.0, 0.1));
    polygon.push(Complex64::new(0.0, 0.2));
    polygon.push(Complex64::new(0.0, 0.3));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn five_points() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(41.0, -6.0));
    polygon.push(Complex64::new(-24.0, -74.0));
    polygon.push(Complex64::new(-51.0, -6.0));
    polygon.push(Complex64::new(73.0, 17.0));
    polygon.push(Complex64::new(-30.0, -34.0));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);

    let mut expected_result: Vec<Complex64> = Vec::new();
    expected_result.push(Complex64::new(-51.0, -6.0));
    expected_result.push(Complex64::new(-24.0, -74.0));
    expected_result.push(Complex64::new(73.0, 17.0));

    let actual_result: Vec<Complex64> = result.unwrap();
    assert_eq!(actual_result.len(), expected_result.len());
    assert_eq!(are_polygons_same(actual_result, expected_result), true);
}

#[test]
fn forty_points() {
    let mut polygon: Vec<Complex64> = Vec::new();
    polygon.push(Complex64::new(0.3215348546593775, 0.03629583077160248));
    polygon.push(Complex64::new(0.02402358131857918, -0.2356728797179394));
    polygon.push(Complex64::new(0.04590851212470659, -0.4156409924995536));
    polygon.push(Complex64::new(0.3218384001607433, 0.1379850698988746));
    polygon.push(Complex64::new(0.11506479756447, -0.1059521474930943));
    polygon.push(Complex64::new(0.2622539999543261, -0.29702873322836));
    polygon.push(Complex64::new(-0.161920957418085, -0.4055339716426413));
    polygon.push(Complex64::new(0.1905378631228002, 0.3698601009043493));
    polygon.push(Complex64::new(0.2387090918968516, -0.01629827079949742));
    polygon.push(Complex64::new(0.07495888748668034, -0.1659825110491202));
    polygon.push(Complex64::new(0.3319341836794598, -0.1821814101954749));
    polygon.push(Complex64::new(0.07703635755650362, -0.2499430638271785));
    polygon.push(Complex64::new(0.2069242999022122, -0.2232970760420869));
    polygon.push(Complex64::new(0.04604079532068295, -0.1923573186549892));
    polygon.push(Complex64::new(0.05054295812784038, 0.4754929463150845));
    polygon.push(Complex64::new(-0.3900589168910486, 0.2797829520700341));
    polygon.push(Complex64::new(0.3120693385713448, -0.0506329867529059));
    polygon.push(Complex64::new(0.01138812723698857, 0.4002504701728471));
    polygon.push(Complex64::new(0.009645149586391732, 0.1060251100976254));
    polygon.push(Complex64::new(-0.03597933197019559, 0.2953639456959105));
    polygon.push(Complex64::new(0.1818290866742182, 0.001454397571696298));
    polygon.push(Complex64::new(0.444056063372694, 0.2502497166863175));
    polygon.push(Complex64::new(-0.05301752458607545, -0.06553921621808712));
    polygon.push(Complex64::new(0.4823896228171788, -0.4776170002088109));
    polygon.push(Complex64::new(-0.3089226845734964, -0.06356112199235814));
    polygon.push(Complex64::new(-0.271780741188471, 0.1810810595574612));
    polygon.push(Complex64::new(0.4293626522918815, 0.2980897964891882));
    polygon.push(Complex64::new(-0.004796652127799228, 0.382663812844701));
    polygon.push(Complex64::new(0.430695573269106, -0.2995073500084759));
    polygon.push(Complex64::new(0.1799668387323309, -0.2973467472915973));
    polygon.push(Complex64::new(0.4932166845474547, 0.4928094162538735));
    polygon.push(Complex64::new(-0.3521487911717489, 0.4352656197131292));
    polygon.push(Complex64::new(-0.4907368011686362, 0.1865826865533206));
    polygon.push(Complex64::new(-0.1047924716070224, -0.247073392148198));
    polygon.push(Complex64::new(0.4374961861758457, -0.001606279519951237));
    polygon.push(Complex64::new(0.003256207800708899, -0.2729194320486108));
    polygon.push(Complex64::new(0.04310378203457577, 0.4452604050238248));
    polygon.push(Complex64::new(0.4916198379282093, -0.345391701297268));
    polygon.push(Complex64::new(0.001675087028811806, 0.1531837672490476));
    polygon.push(Complex64::new(-0.4404289572876217, -0.2894855991839297));

    let result: Result<Vec<Complex64>, String> = convex_hull(polygon);
    assert_eq!(result.is_ok(), true);

    let mut expected_result: Vec<Complex64> = Vec::new();
    expected_result.push(Complex64::new(-0.161920957418085, -0.4055339716426413));
    expected_result.push(Complex64::new(0.05054295812784038, 0.4754929463150845));
    expected_result.push(Complex64::new(0.4823896228171788, -0.4776170002088109));
    expected_result.push(Complex64::new(0.4932166845474547, 0.4928094162538735));
    expected_result.push(Complex64::new(-0.3521487911717489, 0.4352656197131292));
    expected_result.push(Complex64::new(-0.4907368011686362, 0.1865826865533206));
    expected_result.push(Complex64::new(0.4916198379282093, -0.345391701297268));
    expected_result.push(Complex64::new(-0.4404289572876217, -0.2894855991839297));

    let actual_result: Vec<Complex64> = result.unwrap();
    assert_eq!(actual_result.len(), expected_result.len());
    assert_eq!(are_polygons_same(actual_result, expected_result), true);
}
