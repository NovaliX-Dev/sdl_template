/*
App - Application
{% if license_headers and license == "MIT" %}
MIT License
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
{% endif %}{% if license_headers and license == "Apache v2" %}
Copyright {{year}} {{git-username}}

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

{% if logs %}use log::{debug, info};
{% endif %}use sdl2::{
    event::Event as SDLEvent,
    pixels::Color
};

use crate::window;

/// Launch the app
pub fn run() -> anyhow::Result<()> {
    {% if logs %}debug!("Initialize SDL modules...");
    {% endif %}let (video, mut event_pump) = window::init_sdl_modules()
        .map_err(|e| anyhow::anyhow!(e))?;

    {% if logs %}info!("Creating the window...");
    {% endif %}let mut canvas = window::create_window(
        &video, 
        "{{project-name}}", 
        800, 
        600, 
        true, 
        true
    )
        .map_err(|e| anyhow::anyhow!(e))?;

    {% if logs %}info!("Entering window loop...");
    
    {% endif %}'win_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                // window close event (if there is only one)
                SDLEvent::Quit { .. } => {
                    {% if logs %}info!("Window closed");

                    {% endif %}break 'win_loop;
                },

                // --- Add others event here ---

                _ => ()
            }
        }

        // --- Operations put here will be called every frame ---

        // clear the canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // --- Put the rendering stuff here ---

        // update the window
        canvas.present();
    }

    Ok(())
}