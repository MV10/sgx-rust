//! UI callback implementations

use slint::{ComponentHandle, Model, StandardListViewItem};
use super::MainWindow;

/// Sets up callbacks for the Grid list controls
pub fn setup_grid_callbacks(ui: &MainWindow) {
    ui.on_btn_grid_add({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut items: Vec<StandardListViewItem> = ui.get_lst_grid_items().iter().collect();
            items.push(StandardListViewItem::from("(0,0), (1920x1080)"));
            ui.set_lst_grid_items(slint::ModelRc::new(slint::VecModel::from(items)));
        }
    });

    ui.on_btn_grid_remove({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_grid_selected();
            if selected >= 0 {
                let mut items: Vec<StandardListViewItem> = ui.get_lst_grid_items().iter().collect();
                if (selected as usize) < items.len() {
                    items.remove(selected as usize);
                    ui.set_lst_grid_items(slint::ModelRc::new(slint::VecModel::from(items)));
                    ui.set_lst_grid_selected(-1);
                }
            }
        }
    });

    ui.on_btn_grid_up({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_grid_selected();
            if selected > 0 {
                let mut items: Vec<StandardListViewItem> = ui.get_lst_grid_items().iter().collect();
                items.swap(selected as usize, (selected - 1) as usize);
                ui.set_lst_grid_items(slint::ModelRc::new(slint::VecModel::from(items)));
                ui.set_lst_grid_selected(selected - 1);
            }
        }
    });

    ui.on_btn_grid_down({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_grid_selected();
            let items: Vec<StandardListViewItem> = ui.get_lst_grid_items().iter().collect();
            if selected >= 0 && (selected as usize) < items.len().saturating_sub(1) {
                let mut items = items;
                items.swap(selected as usize, (selected + 1) as usize);
                ui.set_lst_grid_items(slint::ModelRc::new(slint::VecModel::from(items)));
                ui.set_lst_grid_selected(selected + 1);
            }
        }
    });
}

/// Sets up callbacks for the Content list controls
pub fn setup_content_callbacks(ui: &MainWindow) {
    ui.on_btn_content_add({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut items: Vec<StandardListViewItem> = ui.get_lst_content_items().iter().collect();
            items.push(StandardListViewItem::from("/path/to/image.jpg"));
            ui.set_lst_content_items(slint::ModelRc::new(slint::VecModel::from(items)));
        }
    });

    ui.on_btn_content_remove({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_content_selected();
            if selected >= 0 {
                let mut items: Vec<StandardListViewItem> = ui.get_lst_content_items().iter().collect();
                if (selected as usize) < items.len() {
                    items.remove(selected as usize);
                    ui.set_lst_content_items(slint::ModelRc::new(slint::VecModel::from(items)));
                    ui.set_lst_content_selected(-1);
                }
            }
        }
    });

    ui.on_btn_content_clear({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_lst_content_items(slint::ModelRc::new(slint::VecModel::<StandardListViewItem>::default()));
            ui.set_lst_content_selected(-1);
        }
    });

    ui.on_btn_content_up({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_content_selected();
            if selected > 0 {
                let mut items: Vec<StandardListViewItem> = ui.get_lst_content_items().iter().collect();
                items.swap(selected as usize, (selected - 1) as usize);
                ui.set_lst_content_items(slint::ModelRc::new(slint::VecModel::from(items)));
                ui.set_lst_content_selected(selected - 1);
            }
        }
    });

    ui.on_btn_content_down({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_content_selected();
            let items: Vec<StandardListViewItem> = ui.get_lst_content_items().iter().collect();
            if selected >= 0 && (selected as usize) < items.len().saturating_sub(1) {
                let mut items = items;
                items.swap(selected as usize, (selected + 1) as usize);
                ui.set_lst_content_items(slint::ModelRc::new(slint::VecModel::from(items)));
                ui.set_lst_content_selected(selected + 1);
            }
        }
    });
}

/// Sets up callbacks for the Highlight list controls
pub fn setup_highlight_callbacks(ui: &MainWindow) {
    ui.on_btn_hilight_add({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut items: Vec<StandardListViewItem> = ui.get_lst_highlight_items().iter().collect();
            items.push(StandardListViewItem::from("/path/to/highlight.jpg"));
            ui.set_lst_highlight_items(slint::ModelRc::new(slint::VecModel::from(items)));
        }
    });

    ui.on_btn_hilight_remove({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_highlight_selected();
            if selected >= 0 {
                let mut items: Vec<StandardListViewItem> = ui.get_lst_highlight_items().iter().collect();
                if (selected as usize) < items.len() {
                    items.remove(selected as usize);
                    ui.set_lst_highlight_items(slint::ModelRc::new(slint::VecModel::from(items)));
                    ui.set_lst_highlight_selected(-1);
                }
            }
        }
    });

    ui.on_btn_hilight_clear({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_lst_highlight_items(slint::ModelRc::new(slint::VecModel::<StandardListViewItem>::default()));
            ui.set_lst_highlight_selected(-1);
        }
    });

    ui.on_btn_hilight_up({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_highlight_selected();
            if selected > 0 {
                let mut items: Vec<StandardListViewItem> = ui.get_lst_highlight_items().iter().collect();
                items.swap(selected as usize, (selected - 1) as usize);
                ui.set_lst_highlight_items(slint::ModelRc::new(slint::VecModel::from(items)));
                ui.set_lst_highlight_selected(selected - 1);
            }
        }
    });

    ui.on_btn_hilight_down({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected = ui.get_lst_highlight_selected();
            let items: Vec<StandardListViewItem> = ui.get_lst_highlight_items().iter().collect();
            if selected >= 0 && (selected as usize) < items.len().saturating_sub(1) {
                let mut items = items;
                items.swap(selected as usize, (selected + 1) as usize);
                ui.set_lst_highlight_items(slint::ModelRc::new(slint::VecModel::from(items)));
                ui.set_lst_highlight_selected(selected + 1);
            }
        }
    });
}

/// Sets up callbacks for toolbar buttons
pub fn setup_toolbar_callbacks(ui: &MainWindow) {
    ui.on_btn_play_clicked(|| {
        println!("Play clicked");
        // TODO: Trigger playback via services::playback
    });

    ui.on_btn_new_clicked(|| {
        println!("New clicked");
        // TODO: Create new playlist via services::playlist
    });

    ui.on_btn_load_clicked(|| {
        println!("Load clicked");
        // TODO: Load playlist via services::playlist
    });

    ui.on_btn_save_clicked(|| {
        println!("Save clicked");
        // TODO: Save playlist via services::playlist
    });

    ui.on_btn_help_clicked(|| {
        println!("Help clicked");
    });

    ui.on_btn_github_clicked(|| {
        println!("Github clicked");
        // TODO: Open GitHub URL in browser
    });

    ui.on_btn_open_clicked(|| {
        println!("Open clicked");
        // TODO: Open file dialog for preview
    });
}
