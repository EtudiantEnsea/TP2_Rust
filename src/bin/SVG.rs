use svg_fmt::*;

fn main() {
   let mut output = String::new();

   let red = Color { r: 255, g: 0, b: 0 };

   output.push_str(&format!("{}", BeginSvg { w: 300.0, h: 200.0 }));
   output.push_str("\n");

   output.push_str(&format!("{}\n", line_segment(100.0, 100.0, 200.0, 100.0).color(red)));
   output.push_str(&format!("{}\n", line_segment(200.0, 100.0, 200.0, 200.0).color(red)));
   output.push_str(&format!("{}\n", line_segment(200.0, 200.0, 100.0, 200.0).color(red)));
   output.push_str(&format!("{}\n", line_segment(100.0, 200.0, 100.0, 100.0).color(red)));

   output.push_str(&format!("{}", EndSvg));


   std::fs::write("square.svg", output).unwrap();
}