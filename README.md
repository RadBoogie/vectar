# VECTAR

### -= A Vector Based 3D Game Engine (in the making) =-

## Overview
Vectar - a 3D vector based game engine, being scratch written from the ground up from first principles.

This project is never going to rival Unreal Engine 5 and is purely for fun and learning.

The 3D maths is basic trigonometry and linear algebra with a little bit of matrix maths thrown in for good measure. Yes
I could use quaternions to avoid gimbal lock but I'd rather use maths that I understand without having to copy paste
from elsewhere.

- **Author**: Richard Moore

## Getting Started

3D models can be imported from Blender and as wavefront `.obj` files with the standard orientation -Z Forward, Y Up.

Place the assets in the relevant folder in `assets/`.

To place objects in the scene add them to the `assets/maps/level1.json` file.

## Features
- **Cross-Platform**: Built with `eframe` for native and web (WASM) support.
- **Asset Embedding**: Uses `rust-embed` for assets so that they can be baked into the binary.
- **Lightweight**: Minimal dependencies for fast compilation and small binary sizes.

## Installation
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/RadBoogie/vectar.git
   cd vectar