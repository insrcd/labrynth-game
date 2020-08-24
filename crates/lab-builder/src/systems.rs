use bevy::{prelude::*, 
    render::{camera::Camera}, input::mouse::MouseWheel,};

use lab_entities::prelude::*;
use lab_sprites::*;
use lab_entities::player;
use std::time::Duration;
use crate::TilePalette;
use lab_input::{Mouse, SelectedTile, State, ScrollState};


pub fn make_tile_palette_system(
    mut sprite_library: ResMut<SpriteLibrary>,
    mut palette: ResMut<TilePalette>
)  {
    println!("Making palette from {} sprites", sprite_library.len());
    for sprite in sprite_library.iter() {
        println!("Adding sprite {:?}", sprite);

        if let Some(comp) = palette.components.get(&sprite.name){
           // already added
            println!("Duplicate sprite detected sprite {:?}", sprite);
        } else {
            palette.components.insert(sprite.name.clone(), TileComponents {
                sprite: sprite.clone(),
                ..Default::default()
            });
        }

    }
}
pub fn update_tile_system (mut commands : Commands, 
    mouse : ResMut<Mouse>,
    mut tile_query: Query<(&FreeTile, &mut Translation, &Draw)>){
    
    for (_ft, mut t, _d) in &mut tile_query.iter(){
        // update the preview tile position
        //println!("Moving tile position {:?}", _d.is_visible);

        *t.0.x_mut() = mouse.position.x();
        *t.0.y_mut() = mouse.position.y();
    }
}
pub fn add_tiles_to_world_system (
    mut commands: Commands,
    selected_tile: Res<SelectedTile>, 
    scroll_state: Res<ScrollState>, 
    palette: Res<TilePalette>,
    mouse_input: Res<Input<MouseButton>>,
    mouse : ResMut<Mouse>,
    mut tile_query: Query<(Entity, &FreeTile, &mut Translation)>
) {    
    if mouse_input.just_pressed(MouseButton::Left) {
        let components = palette.tiles_in_category(&selected_tile.category)[selected_tile.tile as usize];
        
        /* snap to grid */

        let st = selected_tile.clone();                    

        let mut x = mouse.position.x() ;
        let mut y = mouse.position.y() ;
        

        let grid_x = x  / (components.sprite.size().x() * scroll_state.current_scale);
        let grid_y = y  / (components.sprite.size().y() * scroll_state.current_scale);
        
        x = grid_x.round() * components.sprite.size().x() * scroll_state.current_scale;
        y = grid_y.round() * components.sprite.size().y() * scroll_state.current_scale;
            
        println!("Placing tile at {:?},{:?}", x, y);

        let mut clone = components.clone();
        let sprite: SpriteInfo = clone.sprite.clone();

        clone.location = Location(x, y, st.level,  world::WorldLocation::World);
        
        commands
            .spawn(sprite.to_components( Vec3::new(x,y,st.level), Scale(scroll_state.current_scale)))
            .with_bundle(clone);      
        
        for (e, n,t) in &mut tile_query.iter(){
            commands.despawn(e);
        }
        
        commands                          
            .spawn(sprite.to_components(Vec3::new(mouse.position.x(), mouse.position.y(), 100.), Scale(scroll_state.current_scale)))
                .with(FreeTile);      
    }
}

pub struct FreeTile;


pub fn builder_keyboard_system (
    mut commands: Commands,
    windows : Res<Windows>,
    scroll : Res<ScrollState>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut selected_tile: ResMut<SelectedTile>, 
    mut palette: ResMut<TilePalette>, 
    lib : Res<SpriteLibrary>,
    mouse: Res<Mouse>,
    mut camera_query: Query<(&Camera, &Translation)>,
    mut free_tile: Query<(Entity, &FreeTile)>) {

    let mut camera_offset_x : f32 = 0.;
    let mut camera_offset_y : f32 = 0.;
    

    let window = windows.iter().last().unwrap();
    let count = palette.tiles_in_category(&selected_tile.category).len() as u32;

    for (c, t) in &mut camera_query.iter(){
        if *(c.name.as_ref()).unwrap_or(&"".to_string()) == "UiCamera" {
            camera_offset_x = t.x();
            camera_offset_y = t.y();
        }
    }
        
    if keyboard_input.just_pressed(KeyCode::Apostrophe) {
        let categories = palette.tile_categories();
        let pos = categories.iter().position(|&s| s == selected_tile.category).unwrap();

        selected_tile.tile = 0;
        selected_tile.category = palette.tile_categories()[(pos + 1) % categories.len()].to_string();
        println!("Selected category: {}", selected_tile.category);
    }
    
    if keyboard_input.just_pressed(KeyCode::Semicolon) {
        let categories = palette.tile_categories();
        
        let pos = categories.iter().position(|&s| s == selected_tile.category).unwrap();
        println!("{:?} {:?}",categories, pos);
        
        selected_tile.tile = 0;

        if pos != 0 {
            selected_tile.category = palette.tile_categories()[(pos - 1)].to_string();
        } else {            
            selected_tile.category = palette.tile_categories()[palette.tile_categories().len() -1].to_string();
        }
        println!("Selected category: {}", selected_tile.category);
    }

    if keyboard_input.just_pressed(KeyCode::RBracket) {
        let len = palette.tiles_in_category(&selected_tile.category).len();
        selected_tile.tile = (selected_tile.tile + 1) as usize % len;

        let mouse_tile = palette.tiles_in_category(&selected_tile.category)[selected_tile.tile as usize];
        for (n, _ft) in &mut free_tile.iter() {
            commands.despawn(n);
        }
        commands                          
            .spawn(mouse_tile.sprite.to_components(Vec3::new(mouse.position.x(), mouse.position.y(), 100.), Scale(scroll.current_scale)))
                .with(FreeTile);

    } else if keyboard_input.just_pressed(KeyCode::LBracket) {
        for (n, _ft) in &mut free_tile.iter() {
            commands.despawn(n);
        }
        if selected_tile.tile != 0 {
            selected_tile.tile = selected_tile.tile - 1;
        }
        let mouse_tile = palette.tiles_in_category(&selected_tile.category)[selected_tile.tile as usize];
        
        commands                
            .spawn(mouse_tile.sprite.to_components(Vec3::new(mouse.position.x(), mouse.position.y(), 100.), Scale(scroll.current_scale)))
                .with(FreeTile); 

    } else if keyboard_input.just_pressed(KeyCode::Add) {
        selected_tile.level += 1.;
        //write_message(format!("Level changed to {}",selected_tile.level.clone()));         
    } else if keyboard_input.just_pressed(KeyCode::Subtract) {
        selected_tile.level -= 1.;
       // write_message(format!("Level changed to {}",selected_tile.level.clone()));         
    }
}
