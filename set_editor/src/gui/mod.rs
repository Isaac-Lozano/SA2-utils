mod column_type;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::str::FromStr;

use gtk::prelude::*;
use gtk::{self, Builder, Window, Statusbar, Adjustment, TreeView, TreeViewColumn, TreeIter, ListStore, CellRendererText, MenuItem, FileChooserDialog, FileChooserAction, ResponseType, TreeViewGridLines, RadioButton, Entry, Button};
use sa2_set::{SetFile, SetObject, Object, Platform, Dreamcast, GameCube, Pc};

use obj_table::ObjectTable;
use self::column_type::{ColumnType, ObjectID, XRotation, YRotation, ZRotation, XPosition, YPosition, ZPosition, Attribute1, Attribute2, Attribute3};

const GLADE_SRC: &'static str = include_str!("gui.glade");

#[derive(Clone,Debug)]
pub struct SetEditorGui {
    set_objs: Rc<RefCell<SetFile>>,
    obj_table: Rc<RefCell<Option<ObjectTable>>>,
}

impl SetEditorGui {
    pub fn new(set_objs: Option<SetFile>) -> SetEditorGui {
        SetEditorGui {
            set_objs: Rc::new(RefCell::new(set_objs.unwrap_or(SetFile(Vec::new())))),
            obj_table: Rc::new(RefCell::new(None)),
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        gtk::init()?;

        let builder = Builder::new();
        builder.add_from_string(GLADE_SRC).unwrap();

        // TODO: set selectionmode to single for TreeSelection
        let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
        set_grid.set_headers_clickable(true);
        set_grid.set_property_enable_grid_lines(TreeViewGridLines::Both);

        let set_list: ListStore = builder.get_object("Set Objects").unwrap();
        let level_adjustment: Adjustment = builder.get_object("Level Adjustment").unwrap();

        let mut columns = set_grid.get_columns().into_iter();
        columns.next().unwrap().set_sort_column_id(0); // Index
        self.connect_renderer::<ObjectID>(columns.next().unwrap(), 1, &set_list, &level_adjustment);
        columns.next().unwrap().set_sort_column_id(2); // Object Name
        self.connect_renderer::<XRotation>(columns.next().unwrap(), 3, &set_list, &level_adjustment);
        self.connect_renderer::<YRotation>(columns.next().unwrap(), 4, &set_list, &level_adjustment);
        self.connect_renderer::<ZRotation>(columns.next().unwrap(), 5, &set_list, &level_adjustment);
        self.connect_renderer::<XPosition>(columns.next().unwrap(), 6, &set_list, &level_adjustment);
        self.connect_renderer::<YPosition>(columns.next().unwrap(), 7, &set_list, &level_adjustment);
        self.connect_renderer::<ZPosition>(columns.next().unwrap(), 8, &set_list, &level_adjustment);
        self.connect_renderer::<Attribute1>(columns.next().unwrap(), 9, &set_list, &level_adjustment);
        self.connect_renderer::<Attribute2>(columns.next().unwrap(), 10, &set_list, &level_adjustment);
        self.connect_renderer::<Attribute3>(columns.next().unwrap(), 11, &set_list, &level_adjustment);
        self.connect_menu(&builder);

        let statusbar: Statusbar = builder.get_object("Status Bar").unwrap();
        let obj_table_id = statusbar.get_context_id("Object Table Info");
        match ObjectTable::from_file(&PathBuf::from("obj_table.json")) {
            Ok(obj_table) => {
                statusbar.push(obj_table_id, "Successfully loaded object table file.");
                *self.obj_table.borrow_mut() = Some(obj_table);
            }
            Err(e) => {
                statusbar.push(obj_table_id, &format!("Error loading object table: {}", e));
            }
        }

        let window: Window = builder.get_object("Set Editor").unwrap();
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        window.show_all();

        gtk::main();
        Ok(())
    }

    fn load_file(&self, filename: &Path, set_list: &ListStore, level_adjustment: &Adjustment) -> Result<(), &'static str> {
        let mut file = File::open(filename).map_err(|_| "Could not open file.")?;
        let set_objs = SetFile::from_read::<Pc, _>(&mut file).map_err(|_| "Could not parse set file.")?;

        *self.set_objs.borrow_mut() = set_objs;

        self.update_grid(set_list, level_adjustment);
        Ok(())
    }

    fn save_file(set_objs: &Rc<RefCell<SetFile>>, filename: &Path) -> Result<(), &'static str> {
        let mut set_file = File::create(filename).map_err(|_| "Could not create set file.")?;
        set_objs.borrow_mut().write_data::<Pc, _>(&mut set_file).map_err(|_| "Could not write set data.")?;
        Ok(())
    }

    fn update_grid(&self, set_list: &ListStore, level_adjustment: &Adjustment) {
        set_list.clear();
    
        let level_id = level_adjustment.get_value() as u16;
        let mut index = 0;
        for obj in self.set_objs.borrow_mut().0.iter() {
            let empty = String::from("");
            let obj_id = format!("{:04X}", obj.object.0);
            let obj_table_borrow = self.obj_table.borrow();
            let obj_name = obj_table_borrow.as_ref().and_then(|ot| ot.lookup(level_id, obj.object.0)).unwrap_or(&empty);
            let rot_x = format!("{:04X}", obj.rotation.x);
            let rot_y = format!("{:04X}", obj.rotation.y);
            let rot_z = format!("{:04X}", obj.rotation.z);
            let pos_x = obj.position.x;
            let pos_y = obj.position.y;
            let pos_z = obj.position.z;
            let attr_1 = format!("{:08X}", obj.attr1);
            let attr_2 = format!("{:08X}", obj.attr2);
            let attr_3 = format!("{:08X}", obj.attr3);
    
            set_list.insert_with_values(None, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], &[&index, &obj_id, &obj_name, &rot_x, &rot_y, &rot_z, &pos_x, &pos_y, &pos_z, &attr_1, &attr_2, &attr_3]);
            index += 1;
        }
    }

    fn connect_renderer<T>(&self, column: TreeViewColumn, id: i32, set_list: &ListStore, level_adjustment: &Adjustment)
        where T: ColumnType
    {
        // XXX: Technically should be fine because all renderers are CellRendererText,
        // but downcasting is ugly.
        let renderer: CellRendererText = column.get_cells()[0].clone().downcast().unwrap();
        let set_list = set_list.clone();
        let level_adjustment= level_adjustment.clone();
        let self_clone = self.clone();
        renderer.connect_edited(move |_, tree_path, text| {
            if let Ok(value) = T::from_str(text) {
                let iter = set_list.get_iter(&tree_path).unwrap();
                let idx = set_list.get_value(&iter, 0).get::<u32>().unwrap() as usize;
                value.update_obj(&self_clone.set_objs, idx);

                let level_id = level_adjustment.get_value() as u16;
                value.update_column(&set_list, &tree_path, &self_clone.obj_table, level_id);
            }
        });

        column.set_sort_column_id(id);
    }

    fn connect_menu(&self, builder: &Builder) {
        let window: Window = builder.get_object("Set Editor").unwrap();

        {
            let open: MenuItem = builder.get_object("Open").unwrap();
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let level_adjustment: Adjustment = builder.get_object("Level Adjustment").unwrap();
            let statusbar: Statusbar = builder.get_object("Status Bar").unwrap();
            let open_id = statusbar.get_context_id("Open Info");
            let self_clone = self.clone();
            let window = window.clone();
            open.connect_activate(move |_| {
                let file_chooser = FileChooserDialog::new(Some("Open File"), Some(&window), FileChooserAction::Open);
                file_chooser.add_button("_Cancel", ResponseType::Cancel.into());
                file_chooser.add_button("_Open", ResponseType::Accept.into());

                let response = file_chooser.run();

                if response == Into::<i32>::into(ResponseType::Accept) {
                    if let Some(path) = file_chooser.get_filename() {
                        match self_clone.load_file(&path, &set_list, &level_adjustment) {
                            Ok(_) => {
                                statusbar.push(open_id, &format!("Successfully opened file: {}", path.to_str().unwrap_or("")));
                            }
                            Err(e) => {
                                statusbar.push(open_id, &format!("Error: {}", e));
                            }
                        }
                    }
                }

                file_chooser.destroy();
            });
        }

        {
            let save: MenuItem = builder.get_object("Save As").unwrap();
            let statusbar: Statusbar = builder.get_object("Status Bar").unwrap();
            let save_id = statusbar.get_context_id("Save Info");
            let set_objs = self.set_objs.clone();
            let window = window.clone();
            save.connect_activate(move |_| {
                let file_chooser = FileChooserDialog::new(Some("Save File"), Some(&window), FileChooserAction::Save);
                // TODO: Set current name based on type of input
                // TODO: Set file filter
                file_chooser.set_do_overwrite_confirmation(true);
                file_chooser.add_button("_Cancel", ResponseType::Cancel.into());
                file_chooser.add_button("_Save", ResponseType::Accept.into());

                let response = file_chooser.run();

                if response == Into::<i32>::into(ResponseType::Accept) {
                    if let Some(path) = file_chooser.get_filename() {
                        // TODO: error handling
                        match Self::save_file(&set_objs, &path) {
                            Ok(_) => {
                                statusbar.push(save_id, &format!("Successfully saved file: {}", path.to_str().unwrap_or("")));
                            }
                            Err(e) => {
                                statusbar.push(save_id, &format!("Error: {}", e));
                            }
                        }
                    }
                }

                file_chooser.destroy();
            });
        }

        {
            let add_object: MenuItem = builder.get_object("Add Object").unwrap();
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let level_adjustment: Adjustment = builder.get_object("Level Adjustment").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            let self_clone = self.clone();
            add_object.connect_activate(move |_| {
                let (paths, model) = set_grid.get_selection().get_selected_rows();
                let iter_opt = paths.get(0).map(|path| model.get_iter(path).unwrap());
                let object = SetObject::default();

                let idx = iter_opt.map(|iter| set_list.get_value(&iter, 0).get::<u32>().unwrap() as usize + 1).unwrap_or(0);
                self_clone.set_objs.borrow_mut().0.insert(idx, object);

                self_clone.update_grid(&set_list, &level_adjustment);
            });
        }

        {
            let add_object: MenuItem = builder.get_object("Remove Object").unwrap();
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let level_adjustment: Adjustment = builder.get_object("Level Adjustment").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            let self_clone = self.clone();
            add_object.connect_activate(move |_| {
                let (paths, _) = set_grid.get_selection().get_selected_rows();
                for path in paths {
                    let idx = path.get_indices()[0] as usize;
                    self_clone.set_objs.borrow_mut().0.remove(idx);
                }

                self_clone.update_grid(&set_list, &level_adjustment);
            });
        }

        {
            let column_search: MenuItem = builder.get_object("Column Search").unwrap();
            let search_window: Window = builder.get_object("Search Window").unwrap();
            search_window.connect_delete_event(|sw, _| {
                sw.hide();
                Inhibit(true)
            });
            column_search.connect_activate(move |_| {
                search_window.show_all();
            });
        }

        // Search dialog stuff
        {
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            let search_entry: Entry = builder.get_object("Search Entry").unwrap();
            set_grid.set_search_entry(&search_entry);
        }

        {
            let index_radio_button: RadioButton = builder.get_object("Index Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            index_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(0);
            });
        }

        {
            let object_id_radio_button: RadioButton = builder.get_object("Object ID Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            object_id_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(1);
            });
        }

        {
            let object_name_radio_button: RadioButton = builder.get_object("Object Name Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            object_name_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(2);
            });
        }

        {
            let x_rotation_radio_button: RadioButton = builder.get_object("X Rotation Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            x_rotation_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(3);
            });
        }

        {
            let y_rotation_radio_button: RadioButton = builder.get_object("Y Rotation Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            y_rotation_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(4);
            });
        }

        {
            let z_rotation_radio_button: RadioButton = builder.get_object("Z Rotation Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            z_rotation_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(5);
            });
        }

        {
            let x_position_radio_button: RadioButton = builder.get_object("X Position Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            x_position_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(6);
            });
        }

        {
            let y_position_radio_button: RadioButton = builder.get_object("Y Position Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            y_position_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(7);
            });
        }

        {
            let z_position_radio_button: RadioButton = builder.get_object("Z Position Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            z_position_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(8);
            });
        }

        {
            let attribute_1_radio_button: RadioButton = builder.get_object("Attribute 1 Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            attribute_1_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(9);
            });
        }

        {
            let attribute_2_radio_button: RadioButton = builder.get_object("Attribute 2 Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            attribute_2_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(10);
            });
        }

        {
            let attribute_3_radio_button: RadioButton = builder.get_object("Attribute 3 Radio Button").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            attribute_3_radio_button.connect_clicked(move |_| {
                set_grid.set_search_column(11);
            });
        }

        {
            let distance_search: MenuItem = builder.get_object("Distance Search").unwrap();
            let point_search_window: Window = builder.get_object("Point Search Window").unwrap();
            point_search_window.connect_delete_event(|sw, _| {
                sw.hide();
                Inhibit(true)
            });
            distance_search.connect_activate(move |_| {
                point_search_window.show_all();
            });
        }

        {
            let point_search_button: Button = builder.get_object("Point Search Button").unwrap();
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            let x_position_entry: Entry = builder.get_object("X Position Entry").unwrap();
            let y_position_entry: Entry = builder.get_object("Y Position Entry").unwrap();
            let z_position_entry: Entry = builder.get_object("Z Position Entry").unwrap();
            let statusbar: Statusbar = builder.get_object("Status Bar").unwrap();
            let search_id = statusbar.get_context_id("Search Info");
            point_search_button.connect_clicked(move |_| {
                let position_opt = x_position_entry.get_text().and_then(|text| f32::from_str(&text).ok())
                    .and_then(|x| y_position_entry.get_text().and_then(|text| f32::from_str(&text).ok()).map(|y| (x, y)))
                    .and_then(|(x, y)| z_position_entry.get_text().and_then(|text| f32::from_str(&text).ok()).map(|z| (x, y, z)));

                if let Some((x, y, z)) = position_opt {
                    let mut closest: Option<(f32, TreeIter)> = None;

                    let mut iter = set_list.get_iter_first();
                    while let Some(row) = iter {
                        let row_x = set_list.get_value(&row, 6).get::<f32>().unwrap();
                        let row_y = set_list.get_value(&row, 7).get::<f32>().unwrap();
                        let row_z = set_list.get_value(&row, 8).get::<f32>().unwrap();

                        let distance_squared = (row_x - x) * (row_x - x) + (row_y - y) * (row_y - y) + (row_z - z) * (row_z - z);
                        if closest.is_none() || distance_squared < closest.as_ref().unwrap().0 {
                            closest = Some((distance_squared, row.clone()));
                        }

                        if set_list.iter_next(&row) {
                            iter = Some(row);
                        }
                        else {
                            iter = None;
                        }
                    }

                    if let Some((_, iter)) = closest {
                        set_grid.get_selection().select_iter(&iter);
                        let path = set_list.get_path(&iter).unwrap();
                        set_grid.set_cursor(&path, None, false);
                    }
                }
                else {
                    statusbar.push(search_id, "Position values cannot be parsed as floats.");
                }
            });
        }

        {
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let level_adjustment: Adjustment = builder.get_object("Level Adjustment").unwrap();
            let self_clone = self.clone();
            level_adjustment.connect_value_changed(move |adj| {
                self_clone.update_grid(&set_list, &adj);
            });
        }
    }
}
