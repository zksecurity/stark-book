use std::collections::HashMap; // For generating random numbers

#[derive(Debug, Clone)]
struct Polynomial {
    coefficients: Vec<i32>,
}

impl Polynomial {
    // Constructor for the Polynomial struct
    pub fn new(coefficients: Vec<i32>) -> Self {
        Polynomial { coefficients }
    }

    // Function to evaluate the polynomial at a given value of x
    pub fn evaluate(&self, x: i32) -> i32 {
        self.coefficients
            .iter()
            .enumerate()
            .fold(0, |acc, (power, &coeff)| acc + coeff * x.pow(power as u32))
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = usize::max(self.coefficients.len(), other.coefficients.len());
        let mut result = vec![0; max_len];

        for i in 0..max_len {
            let a = *self.coefficients.get(i).unwrap_or(&0);
            let b = *other.coefficients.get(i).unwrap_or(&0);
            result[i] = a + b;
        }

        Polynomial::new(result)
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.coefficients == other.coefficients
    }
}

// Trait for displaying a polynomial
trait DisplayPolynomial {
    fn format(&self) -> String;
}

// Implementing DisplayPolynomial for Polynomial
impl DisplayPolynomial for Polynomial {
    fn format(&self) -> String {
        let mut formatted_string = String::new();
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            if coeff != 0 {
                if i == 0 {
                    // First coefficient
                    formatted_string.push_str(&format!("{}", coeff));
                } else {
                    // Add + sign for positive coefficients, except the first term
                    if formatted_string.len() > 0 && coeff > 0 {
                        formatted_string.push_str(" + ");
                    } else if coeff < 0 {
                        formatted_string.push_str(" - ");
                    }

                    // Add coefficient (absolute value) and variable part
                    let abs_coeff = coeff.abs();
                    if abs_coeff != 1 {
                        formatted_string.push_str(&format!("{}", abs_coeff));
                    }
                    formatted_string.push_str(&format!("x^{}", i));
                }
            }
        }
        formatted_string
    }
}

fn fold_polynomial(poly: &Polynomial, beta: i32) -> Polynomial {
    let even_coef: Vec<i32> = poly.coefficients.iter().step_by(2).cloned().collect();
    let odd_coef: Vec<i32> = poly
        .coefficients
        .iter()
        .skip(1)
        .step_by(2)
        .map(|&coef| coef * beta) // Multiply each odd coefficient by beta
        .collect();

    let even_poly = Polynomial::new(even_coef);
    let odd_poly = Polynomial::new(odd_coef);

    even_poly.add(&odd_poly)
}

fn recursively_fold_polynomials(poly: Polynomial, beta: i32) -> Vec<Polynomial> {
    let mut folded_polynomials = Vec::new();
    let mut current_poly = poly;

    loop {
        folded_polynomials.push(current_poly.clone());

        // Check if the degree of the current polynomial is 0
        if current_poly.coefficients.len() <= 1 {
            break;
        }

        // Fold the polynomial
        current_poly = fold_polynomial(&current_poly, beta);
    }

    folded_polynomials
}

fn create_layers(x_values: &[i32], poly_by_layer: Vec<Polynomial>) -> Vec<HashMap<i32, i32>> {
    let mut layer_evals: Vec<HashMap<i32, i32>> = Vec::new();

    for &x in x_values {
        println!("----\nz = {}\n", x);

        for (i, poly) in poly_by_layer.iter().enumerate() {
            let degree = poly.coefficients.len() - 1;
            let exponent = 2i32.pow(i as u32);
            let elm_point = x.pow(exponent as u32);
            let symmetric_elm_point = -elm_point;
            let elm = poly.evaluate(elm_point);
            let symmetric_elm = poly.evaluate(symmetric_elm_point);

            let poly_info = format!(
                "p(x) at layer {}: {:?}, degree: {}",
                i,
                poly.format(),
                degree
            );
            let evaluations = if i == 0 {
                format!(
                    "y_{} = z^{}, p(y_{}) = {}, p(-y_{}) = {}",
                    i, exponent, i, elm, i, symmetric_elm
                )
            } else {
                format!(
                    "y_{} = y_{}^2 = z^{}, p(y_{}) = {}, p(-y_{}) = {}",
                    i,
                    i - 1,
                    exponent,
                    i,
                    elm,
                    i,
                    symmetric_elm
                )
            };

            println!("{}\n{}\n", poly_info, evaluations);

            if layer_evals.get(i).is_none() {
                layer_evals.push(HashMap::new());
            }

            let point_to_eval = layer_evals.get_mut(i).unwrap();
            point_to_eval.insert(elm_point, elm);
            point_to_eval.insert(symmetric_elm_point, symmetric_elm);
        }
    }

    layer_evals
}

fn check_layers(layer_evals: &[HashMap<i32, i32>], query: i32, beta: i32) {
    for (i, layer) in layer_evals.iter().enumerate() {
        println!("current layer: {}", i);

        // Skip first layer
        if i == 0 {
            continue;
        }

        let prev_exponent = 2i32.pow((i - 1) as u32);
        let prev_query_point = query.pow(prev_exponent as u32);
        let prev_symmetric_query_point = -(query).pow(prev_exponent as u32);

        let prev_layer = layer_evals.get(i - 1).unwrap();
        let prev_elm = prev_layer.get(&prev_query_point).unwrap();
        let prev_symmetric_elm = prev_layer.get(&prev_symmetric_query_point).unwrap();

        let current_query_point = prev_query_point.pow(2);
        println!("prev_query_point: {}", current_query_point);

        let current_elm = layer.get(&current_query_point).unwrap();

        let expected = (prev_elm + prev_symmetric_elm) / 2
            + beta * (prev_elm - prev_symmetric_elm) / (2 * prev_query_point);

        assert_eq!(expected, *current_elm);
    }

    // Assert the values in the last layer are the same
    let last_layer = layer_evals.last().unwrap();
    let mut values = std::collections::HashSet::new();
    for (_, &value) in last_layer.iter() {
        values.insert(value);
    }
    assert_eq!(values.len(), 1);
}

#[test]
fn naive_query() {
    let beta = 1;
    let poly = Polynomial::new(vec![1, 2, 3]);
    let poly_by_layer = recursively_fold_polynomials(poly, beta);

    let layer_evals = create_layers(&[1, 2, 3], poly_by_layer);

    let query = 2;
    check_layers(&layer_evals, query, beta);
}
