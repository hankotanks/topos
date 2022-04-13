#  OpenGL Render of National Elevation Dataset (NED)

Using data pulled from United States Geological Survey ([USGS](https://gdg.sc.egov.usda.gov/Catalog/ProductDescription/NED.html)), this project creates a 3D mesh of a given subregion. The resulting view allows the user to increase/decrease scale, pan, and zoom. 

Data is read from a P5 `.pgm` image, which was derived from Princeton's shaded relief map of the United States (https://maps.princeton.edu/catalog/stanford-zv090gb7559).

## Scope

- Creating a simple parser that can read a subregion with arbitrary bounds from a `.pgm` file. Since the dataset could have up to 10^8 values, not all of it can be held in memory at once (something most existing image libraries relied on).
- Generating the mesh from the parsed data by calculating vertex positions and surface normals (for lighting).
- Assemble these vertices into triangles, from which the mesh is rendered.
- Pass the triangles and normals as buffers into OpenGL using the `glium` crate (a Rust wrapper for OpenGL). Catch user input for panning, zooming, and scaling the currently-viewed region. Adjust the view accordingly.
- Implement shaders for the resulting render using GLSL (found in `topo.vert.gl` & `topo.frag.gl`).