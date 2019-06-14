use nalgebra;
use rtsam;

use nalgebra::Vector6;
use rtsam::core::group::LieGroup;
use rtsam::geometry::SE3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dummy Example PRoGraM");

    let w = Vector6::new(1., 1.2, 1.3, 1., 1.4, 1.3);

    let exp = SE3::expmap_with_derivative(&w, None);

    let log = SE3::logmap(&exp, None);

    println!("w: {}", w.transpose());
    println!("Expmap: {}", exp);
    println!("Logmap: {}", log.transpose());

    Ok(())
}
