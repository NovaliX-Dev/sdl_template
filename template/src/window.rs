/*
Window - Window function

{% if license_headers and license == "MIT" %}MIT License
Copyright (c) {{year}} {{git-username}}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
{% endif %}{% if license_headers and license == "Apache v2" %}Copyright {{year}} {{git-username}}

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
{% endif %}{% if license_headers and license == "GNU v3" %}
Copyright (C) {{year}} {{git-username}}

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
{% endif %}*/

use log::trace;
use sdl2::{
    VideoSubsystem, 
    EventPump, 
    render::Canvas, 
    video::Window
};

/// Init all the needed SDL modules
pub fn init_sdl_modules() -> Result<(VideoSubsystem, EventPump), String> {
    let sdl_context = sdl2::init()?;

    let video = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;

    // --- initialize others modules here ---

    Ok((video, event_pump))
}

/// Create a SDL window
pub fn create_window(
    video: &VideoSubsystem, 
    title: &str, 
    width: u32, 
    height: u32, 
    resizable: bool, 
    accelerated: bool
) -> Result<Canvas<Window>, String> {
    trace!("Creating and configuring the window...");

    // configure the window builder
    let mut w_builder = video.window(title, width, height);
    if resizable {
        w_builder.resizable();
    }

    // --- Add others window configuration here (if needed) ---

    let window = w_builder.build()
        .map_err(|e| e.to_string())?;

    trace!("Creating and configuring the canvas...");

    // configure the canvas builder
    let mut c_builder = window.into_canvas();
    if accelerated {
        c_builder = c_builder.accelerated()
    };

    // --- Add others canvas configuration here (if needed) ---

    let canvas = c_builder.build()
        .map_err(|e| e.to_string())?;

    Ok(canvas)
}