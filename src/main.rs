static mut CHILD: Option<Child> = None;
use hyprland::data::Workspace;
use hyprland::event_listener::AsyncEventListener;

use hyprland::async_closure;
use hyprland::shared::{HyprDataActive, WorkspaceType};
use legionlight_rs::colors;
use tokio::process::{Child, Command};
async fn get_colours(skp: usize) -> String {
    let col = make_colours().await;
    let mut vec_cols: circular_vec::CircularVec<hex_color::HexColor> = [
        col.colors.color0,
        col.colors.color1,
        col.colors.color2,
        col.colors.color3,
        col.colors.color4,
        col.colors.color5,
        col.colors.color6,
        col.colors.color7,
        col.colors.color8,
        col.colors.color9,
        col.colors.color10,
        col.colors.color11,
        col.colors.color12,
        col.colors.color13,
        col.colors.color14,
        col.colors.color15,
    ]
    .into_iter()
    .collect();
    vec_cols.skip(skp);
    let mut colours = String::new();
    for _ in 0..4 {
        let i = vec_cols.next();
        colours.push_str(i.r.to_string().as_str());
        colours.push(',');
        colours.push_str(i.g.to_string().as_str());
        colours.push(',');
        colours.push_str(i.b.to_string().as_str());
        colours.push(',');
    }
    colours.pop();
    colours
}
async fn make_colours() -> colors::Root {
    let file = tokio::fs::read_to_string("/home/thulashitharan/.cache/wal/colors.json")
        .await
        .unwrap();
    serde_json::from_str(&file).unwrap()
}
async fn process_workspace_change(id: WorkspaceType) {
    if let WorkspaceType::Regular(_) = id {
        change_bl(Workspace::get_active_async().await.unwrap().id as usize).await
    }
}
async fn change_bl(id: usize) {
    let mut bl = Command::new("/bin/legion-kb-rgb");
    bl.arg("set").arg("-e").arg("Swipe").arg("-c");
    bl.arg(get_colours(id).await);
    unsafe {
        if let Some(child) = &mut CHILD {
            child.kill().await.unwrap();
        }
        CHILD = Some(bl.spawn().unwrap());
    }
}
#[tokio::main]
async fn main() -> hyprland::Result<()> {
    // Create a event listener
    let mut event_listener = AsyncEventListener::new();
    // This makes it so you can't turn on fullscreen lol

    event_listener.add_workspace_change_handler(async_closure! {
        move |id| {
            process_workspace_change(id).await
        }
    });

    event_listener.start_listener_async().await
}
