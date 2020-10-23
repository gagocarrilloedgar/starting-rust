fn main() {
    println!("Newton rapson solver for the kepler's equation!");

    let e = 0.1;
    let  mut m_anom = 170.0;
    m_anom = (m_anom/180.0)* 3.14;
    let delta = 0.00001;
    let max = 20;

    let e_anom:f32 = newton_rapson(delta, m_anom, e, max);

    println!("The converged E for e:{} and M:{} is E:{}",e,m_anom,e_anom);

}

fn newton_rapson(delta: f32, m_anom: f32,e:f32, max:u32)->f32{

    let mut i =1;
    let mut t: f32 = 1.0;

    let mut e_anom = m_anom;

    while t.abs() > delta {
        let f = e_anom - e*e_anom.sin() - m_anom;
        let d_f: f32 = 1.0 - e*e_anom.cos(); 

        if d_f.abs()<delta{
            break;
        } else {
            t = f/d_f;
            e_anom = e_anom - t;
            i += 1;

            if i>max{
                println!("no convergence ather:{} iterarions",i);
                break;
            }
        }
    }

    e_anom
}
