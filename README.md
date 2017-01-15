# png4snowy

This tool converts PNG images to the binary format expected by the FPGA inside the Pebble Time (codename Snowy) and Pebble Time Steel.  
The input PNG has to have a bit depth of 8 bit and has to either be RGB or RGBA.  
When directly piping the data into the FPGA without any postprocessing, the image has to be RGB and has to have a resolution of 144x168. Everything else will result in a crappy looking image!
