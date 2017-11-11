/*
   Copyright 2017 Miguel Peláez <kernelfreeze@greenlab.games>
   Copyright 2017 Raúl Salas <raulsalas.martin@greenlab.games>
   
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use allegro::core::BitmapDrawingFlags;
use allegro::{Flag, Color};
use client::Client;
use allegro::bitmap_like::BitmapLike;
use client::allegro_font::{FontDrawing, FontAlign};
use client::resourcemanager::TextureType;

pub trait Component {
    fn draw_2d(&self, client: &Client, x: f32, y: f32, w: f32, h: f32, texture: &TextureType) {
        let texture = client.resource_manager.get_texture(texture);

        client.core.draw_scaled_bitmap(
            texture,
            0f32, 0f32,                              // source origin
            texture.get_width() as f32,              // source width
            texture.get_height() as f32,             // source height
            x, y,                                    // target origin
            w, h,                                    // target dimensions
            BitmapDrawingFlags::zero()               // flags
        );
    }

    fn draw_text(&self, client: &Client, color: Color, text: &str, x: f32, y: f32) {
        client.core.draw_text(
            &client.resource_manager.get_minecraft_font(),
            color,
            x,
            y,
            FontAlign::Centre,
            text,
        );
    }

    fn draw_litecraft_text(&self, client: &Client, color: Color, text: &str, x: f32, y: f32) {
        client.core.draw_text(
            &client.resource_manager.get_litecraft_font(),
            color,
            x,
            y,
            FontAlign::Centre,
            text,
        );
    }
}

#[derive(Debug)]
pub enum ContainerPosition {
    UpLeft,
    UpCenter,
    UpRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottonCenter,
    BottonRight
}

#[derive(Debug)]
pub struct Container {
    position: ContainerPosition,
    buttons: Vec<Button>
}

pub trait Element : Component {
    fn get_position(&self, client: &Client, position: &ContainerPosition, cx: f32, cy: f32, x: f32, y: f32, w: f32, h: f32, scale: u8) -> (f32, f32, f32, f32) {
        let mut x = x;
        let mut y = y;

        match *position {
            ContainerPosition::UpLeft => (),
            ContainerPosition::UpCenter => {
                x += (client.display.get_width() / 2 - (w as i32 / 2)) as f32;
            },
            ContainerPosition::UpRight => {
                x += client.display.get_width() as f32 - w;
            },
            ContainerPosition::MiddleLeft => {
                y += (client.display.get_height() / 2 - (h as i32 / 2)) as f32;
            },
            ContainerPosition::MiddleCenter => {
                x += (client.display.get_width() / 2 - (w as i32 / 2)) as f32;
                y += (client.display.get_height() / 2 - (h as i32 / 2)) as f32;
            },
            ContainerPosition::MiddleRight => {
                x += client.display.get_width() as f32 - w;
                y += (client.display.get_height() / 2 - (h as i32 / 2)) as f32;
            },
            ContainerPosition::BottomLeft => {
                y += client.display.get_height() as f32 - h;
            },
            ContainerPosition::BottonCenter => {
                x += (client.display.get_width() / 2 - (w as i32 / 2)) as f32;
                y += client.display.get_height() as f32 - h;
            },
            ContainerPosition::BottonRight => {
                x += client.display.get_width() as f32 - w;
                y += client.display.get_height() as f32 - h;
            },
        };
        self.get_scale(position, cx + x, cy + y, cx + w, cy + h, scale)
    }

    fn get_scale(&self, position: &ContainerPosition, x: f32, y: f32, w: f32, h: f32, scale: u8) -> (f32, f32, f32, f32) {
        let scale = scale as f32 * 100f32;

        match *position {
            ContainerPosition::UpLeft => (x, y, w + scale, h + scale),
            ContainerPosition::UpCenter => (x + scale, y, w + scale, h + scale),
            ContainerPosition::UpRight => (x + scale, y, w, h + scale),
            ContainerPosition::MiddleLeft => (x, y + scale, w + scale, h + scale),
            ContainerPosition::MiddleCenter => (x + scale, y + scale, w + scale, h + scale),
            ContainerPosition::MiddleRight => (x, y + scale, w + scale, h + scale),
            ContainerPosition::BottomLeft => (x, y + scale, w + scale, h),
            ContainerPosition::BottonCenter => (x + scale, y + scale, w + scale, h),
            ContainerPosition::BottonRight => (x + scale, y + scale, w, h),
        }
    }

    fn draw(&self, client: &Client, position: &ContainerPosition, cx: f32, cy: f32, x: f32, y: f32, w: f32, h: f32, scale: u8);
}

#[derive(Debug)]
pub struct Button {
    texture: TextureType,   
}

impl Button {
    fn new(texture: TextureType) -> Button {
        Button {texture}
    }
}

impl Component for Button {}

impl Element for Button {
    fn draw(&self, client: &Client, position: &ContainerPosition, cx: f32, cy: f32, x: f32, y: f32, w: f32, h: f32, scale: u8) {
        let (x, y, w, h) = self.get_position(client, position, cx, cy, x, y, w, h, scale);

        self.draw_2d(client, x, y, w, h, &self.texture);
    }
}