use crate::M2x2;
use crate::M2x1;


fn dot_2x2_2x1(a: &M2x2, b: &M2x1) -> M2x1 {
    (
        (a.0.0 * b.0.0 + a.0.1 * b.1.0, ), 
        (a.1.0 * b.0.0 + a.1.1 * b.1.0, )
    )
}

fn dot_2x2(a: &M2x2, b: &M2x2) -> M2x2 {
    (
        (a.0.0 * b.0.0 + a.0.1 * b.1.0, a.0.0 * b.0.1 + a.0.1 * b.1.1), 
        (a.1.0 * b.0.0 + a.1.1 * b.1.0, a.1.0 * b.0.1 + a.1.1 * b.1.1)
    )
}

fn transpose(a: &M2x2) -> M2x2 {
    (
        (a.0.0, a.1.0),
        (a.0.1, a.1.1)
    )
}

fn add_2x2(a: &M2x2, b: &M2x2) -> M2x2 {
    ((a.0.0 + b.0.0, a.0.1 + b.0.1), (a.1.0 + b.1.0, a.1.1 + b.1.1))
}

fn add_2x1(a: &M2x1, b: &M2x1) -> M2x1 {
    ((a.0.0 + b.0.0, ), (a.1.0 + b.1.0, ))
}

fn sub_2x1(a: &M2x1, b: &M2x1) -> M2x1 {
    ((a.0.0 - b.0.0, ), (a.1.0 - b.1.0, ))
}

fn sub_2x2(a: &M2x2, b: &M2x2) -> M2x2 {
    ((a.0.0 - b.0.0, a.0.1 - b.0.1), (a.1.0 - b.1.0, a.1.1 - b.1.1))
}

fn inv_2x2(a: &M2x2) -> M2x2 {
    let det = a.0.0 * a.1.1 - a.1.0 * a.0.1;
    
    if det == 0.0 {panic!("Could not inverse matrix with determinant equal to zero");}

    (
        (a.1.1 / det, -a.1.0 / det),
        (-a.0.1 / det, a.0.0 / det)
    )
}

/**
x : The mean state estimate of the previous step (k −1)
p : The state covariance of previous step (k −1).
a : The transition n n × matrix.
*/

pub fn predict(x: &M2x1, p: &M2x2, a: &M2x2) -> (M2x1, M2x2) {    
    let x = dot_2x2_2x1(&a, &x);
    let p = dot_2x2(&a, &dot_2x2(&p, &transpose(&a)));
    let p_diag = (
        (p.0.0, 0.0),
        (0.0, p.1.1)
    );
    (x, p_diag)
}

/**
x : The mean state estimate of the previous step (k −1).
p : The state covariance of previous step (k −1).
y : The measurement matrix.
h : The state matrix.
r : The measurement noise covariance matrix.
*/
pub fn update(x: &M2x1, p: &M2x2, y: &M2x1, h: &M2x2, r: &M2x2) -> (M2x1, M2x2) {
    let k_num = dot_2x2(&p, &transpose(&h));
    let k_den =
        &add_2x2(
            &dot_2x2(
                &dot_2x2(&h, &p), 
                &transpose(&h)
            ), 
            &r
        );

    let k = dot_2x2(&k_num, &inv_2x2(&k_den));

    let x = add_2x1(
        &x, 
        &dot_2x2_2x1(
            &k, 
            &sub_2x1(
                &y, 
                &dot_2x2_2x1(&h, &x)
            )
        )
    );

    let p = sub_2x2(
        &p,
        &dot_2x2(
            &k,
            &dot_2x2(&h, &p)
        )
    );

    (x, p)
}

#[cfg(test)]
mod test {

    use super::{predict, dot_2x2_2x1, dot_2x2, transpose, add_2x2, sub_2x1, add_2x1, sub_2x2, inv_2x2};

    #[test]
    fn test_transpose() {
        let a = (
            (1.0, 2.0),
            (3.0, 4.0)
        );
        let res = (
            (1.0, 3.0),
            (2.0, 4.0)
        );
        assert_eq!(transpose(&a), res);
    }

    #[test]
    fn test_dot_to_2x1() {
        let a = (
            (1.0, 2.0),
            (3.0, 4.0)
        );
        let b = (
            (5.0,),
            (6.0,)
        );
        let res = (
            (17.0,),
            (39.0,)
        );
        assert_eq!(dot_2x2_2x1(&a, &b), res);
    }

    #[test]
    fn test_dot_to_2x2() {
        let a = (
            (1.0, 2.0),
            (3.0, 4.0)
        );
        let b = (
            (5.0, 6.0),
            (7.0, 8.0)
        );
        let res = (
            (19.0, 22.0),
            (43.0, 50.0)
        );
        assert_eq!(dot_2x2(&a, &b), res);
    }

    #[test]
    fn test_add_2x2() {
        let a = (
            (1.0, 2.0),
            (3.0, 4.0)
        );
        let b = (
            (5.0, 6.0),
            (7.0, 8.0)
        );
        let res = (
            (6.0, 8.0),
            (10.0, 12.0)
        );
        assert_eq!(add_2x2(&a, &b), res);
    }
    
    #[test]
    fn test_sub_2x1() {
        let a = (
            (1.0,),
            (2.0,)
        );
        let b = (
            (3.0,),
            (4.0,)
        );
        let res = (
            (-2.0,),
            (-2.0,)
        );
        assert_eq!(sub_2x1(&a, &b), res);
    }

    #[test]
    fn test_add_2x1() {
        let a = (
            (1.0,),
            (2.0,)
        );
        let b = (
            (3.0,),
            (4.0,)
        );
        let res = (
            (4.0,),
            (6.0,)
        );
        assert_eq!(add_2x1(&a, &b), res);
    }
    #[test]
    fn test_sub_2x2() {
        let a = (
            (1.0, 2.0),
            (3.0, 4.0)
        );
        let b = (
            (5.0, 6.0),
            (7.0, 8.0)
        );
        let res = (
            (-4.0, -4.0),
            (-4.0, -4.0)
        );
        assert_eq!(sub_2x2(&a, &b), res);
    }

    #[test]
    #[should_panic]
    fn test_inv_2x2_should_panic_if_determinant_is_0() {
        let a = (
            (1.0, 1.0),
            (2.0, 2.0)
        );        
        inv_2x2(&a);
    }
    

    #[test]
    fn test_predict() {

        let x = (
            (1.0,),
            (2.0,)
        );

        let p = (
            (3.0, 0.0),
            (0.0, 4.0)
        );

        let a = (
            (2.0, 2.0),
            (0.0, 2.0)
        );
        let res = (
            ((6.0,), (4.0,)),
            ((28.0, 0.0), (0.0, 16.0)),
        );

        assert_eq!(predict(&x, &p, &a), res);
    }
}
    

