pub fn heat_equation_1d(
    temp1: f32,
    temp2: f32,
    temp_init: f32,
    nx: usize,
    delta_x: f32,
    t_duration: f32,
    delta_t: f32,
    alpha: f32,
) -> Vec<Vec<f32>> {
    // Finite difference diffusion equation solver
    // temp1: temperture on one side of the rod
    // temp2: temperature on the other side of the rod
    // nx: number of spatial points
    // delta_x: space discretization
    // t_duration: duration of simulation in seconds
    // delta_t: time step size
    // alpha: thermal diffusivity

    // number of timesteps to be solved
    let nt = (t_duration / delta_t) as usize;

    let mut temp = vec![vec![temp_init; nx]; nt];

    // Set boundary conditions
    for t in 0..nt {
        temp[t][0] = temp1;
        temp[t][nx - 1] = temp2;
    }

    // Solve in time
    // du/dt = alpha * d2u/dx2
    // du/dt = (u(n+1)_i - u(n)_i) / delta_t
    // d2u/dx2 = (u(n)_i+1 - 2 * u(n)_i + u(n)_i-1) / delta_x^2
    // (u(n+1)_i - u(n)_i) / delta_t = alpha * (u(n)_i+1 - 2 * u(n)_i + u(n)_i-1) / delta_x^2
    // u(n+1)_i = u(n)_i + (u(n)_i+1 - 2 * u(n)_i + u(n)_i-1) * delta_nt / delta_x^2
    for t in 0..nt - 1 {
        for i in 1..nx - 1 {
            temp[t + 1][i] = temp[t][i]
                + alpha * delta_t / (delta_x * delta_x)
                    * (temp[t][i + 1] - 2.0 * temp[t][i] + temp[t][i - 1])
        }
    }

    temp
}
