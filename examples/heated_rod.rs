use cfd_solver::energy;

fn main() {
    let temp1: f32 = 273.15; // temperture on one side of the rod
    let temp2: f32 = 323.15; // temperature on the other side of the rod
    let temp_init: f32 = temp2; // initial temperature of rod
    let nx: usize = 10; // number of spatial points
    let delta_x: f32 = 1.0; // space discretization
    let t_duration: f32 = 10.0; // duration of simulation in seconds
    let delta_t: f32 = 0.1; // time step size
    let alpha: f32 = 1.0; // thermal diffusivity

    let temp_arr = energy::heat_equation_1d(
        temp1, temp2, temp_init, nx, delta_x, t_duration, delta_t, alpha,
    );

    // print temperature at position ni
    let ni: usize = 2;
    for t in 0..((t_duration / delta_t) as usize) {
        println!("Time step {}: T = {}", t, temp_arr[t][ni]);
    }
}
