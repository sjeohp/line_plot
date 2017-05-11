extern crate line_plot;

use line_plot::*;

fn main()
{
	// feature::plot(
 //        &[0.0, 300.0, 300.0, 600.0],
 //        &[0.0, 0.0, 600.0, 600.0],
	// 	0.0, 
	// 	1200.0,
	// 	0.0, 
	// 	1200.0);

	init(100, 100, 500, 500, || {
		PlotData {
			axis_x: vec![0.0, 1.0],
			axis_y: vec![-1.0, 1.0],
			values_x: vec![0.0, 0.5],//, 300.0, 400.0],
			values_y: vec![0.0, 0.5]//, 400.0, 400.0]
		}
	});
}