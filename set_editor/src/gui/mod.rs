mod column_type;

use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::{self, Builder, Window, TreeView, TreeViewColumn, ListStore, CellRendererText, MenuItem, FileChooserDialog, FileChooserAction, ResponseType, TreeViewGridLines};
use sa2_set::{SetFile, SetObject, Object, Platform, Dreamcast, GameCube, Pc};

use obj_table::OBJ_TABLE;
use self::column_type::{ColumnType, ObjectID, XRotation, YRotation, ZRotation, XPosition, YPosition, ZPosition, Attribute1, Attribute2, Attribute3};

const GLADE_SRC: &'static str = include_str!("gui.glade");

pub struct SetEditorGui {
    set_objs: Rc<RefCell<SetFile>>,
}

impl SetEditorGui {
    pub fn new(set_objs: Option<SetFile>) -> SetEditorGui {
        SetEditorGui {
            set_objs: Rc::new(RefCell::new(set_objs.unwrap_or(SetFile(Vec::new())))),
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

        let window: Window = builder.get_object("Set Editor").unwrap();
        window.set_default_size(800, 500);

        let set_list: ListStore = builder.get_object("Set Objects").unwrap();

        let mut columns =  set_grid.get_columns().into_iter();
        self.connect_renderer::<ObjectID>(columns.next().unwrap(), &set_list);
        columns.next(); // Object Name
        self.connect_renderer::<XRotation>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<YRotation>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<ZRotation>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<XPosition>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<YPosition>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<ZPosition>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<Attribute1>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<Attribute2>(columns.next().unwrap(), &set_list);
        self.connect_renderer::<Attribute3>(columns.next().unwrap(), &set_list);
        self.connect_menu(&builder);

        let window: Window = builder.get_object("Set Editor").unwrap();
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        window.show_all();

        gtk::main();
        Ok(())
    }

    fn load_file(self_set_objs: &Rc<RefCell<SetFile>>, filename: &Path, set_list: &ListStore) {
        let mut file = File::open(filename).unwrap();
        let set_objs = SetFile::from_read::<Pc, _>(&mut file).unwrap();

        set_list.clear();
    
        for obj in set_objs.0.iter() {
            let empty = "";

            let obj_id = format!("{:04X}", obj.object.0);
            let obj_name = OBJ_TABLE.get(&(13, obj.object.0)).unwrap_or(&empty);
            let rot_x = format!("{:04X}", obj.rotation.x);
            let rot_y = format!("{:04X}", obj.rotation.y);
            let rot_z = format!("{:04X}", obj.rotation.z);
            let pos_x = obj.position.x.to_string();
            let pos_y = obj.position.y.to_string();
            let pos_z = obj.position.z.to_string();
            let attr_1 = format!("{:08X}", obj.attr1);
            let attr_2 = format!("{:08X}", obj.attr2);
            let attr_3 = format!("{:08X}", obj.attr3);
    
            set_list.insert_with_values(None, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &[&obj_id, &obj_name, &rot_x, &rot_y, &rot_z, &pos_x, &pos_y, &pos_z, &attr_1, &attr_2, &attr_3]);
        }

        *self_set_objs.borrow_mut() = set_objs;
    }

    fn save_file(set_objs: &Rc<RefCell<SetFile>>, filename: &Path) -> Result<(), &'static str> {
        let mut set_file = File::create(filename).map_err(|_| "Could not create set file.")?;
        set_objs.borrow_mut().write_data::<Pc, _>(&mut set_file).map_err(|_| "Could not write set data.")?;
        Ok(())
    }

    fn connect_renderer<T>(&self, column: TreeViewColumn, set_list: &ListStore)
        where T: ColumnType
    {
        // XXX: Technically should be fine because all renderers are CellRendererText,
        // but downcasting is ugly.
        let renderer: CellRendererText = column.get_cells()[0].clone().downcast().unwrap();
        let set_list: ListStore = set_list.clone();
        let set_objs = self.set_objs.clone();
        renderer.connect_edited(move |_, tree_path, text| {
            if let Ok(value) = T::from_str(text) {
                value.update_column(&set_list, &tree_path);
                value.update_obj(&set_objs, &tree_path);
            }
        });
    }

    fn connect_menu(&self, builder: &Builder) {
        let window: Window = builder.get_object("Set Editor").unwrap();

        {
            let open: MenuItem = builder.get_object("Open").unwrap();
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let set_objs = self.set_objs.clone();
            let window = window.clone();
            open.connect_activate(move |_| {
                let file_chooser = FileChooserDialog::new(Some("Open File"), Some(&window), FileChooserAction::Open);
                file_chooser.add_button("_Cancel", ResponseType::Cancel.into());
                file_chooser.add_button("_Open", ResponseType::Accept.into());

                let response = file_chooser.run();

                if response == Into::<i32>::into(ResponseType::Accept) {
                    if let Some(path) = file_chooser.get_filename() {
                        Self::load_file(&set_objs, &path, &set_list);
                        println!("Opened: {:#?}", path);
                    }
                }

                file_chooser.destroy();
            });
        }

        {
            let save: MenuItem = builder.get_object("Save As").unwrap();
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
                        Self::save_file(&set_objs, &path).unwrap();
                        println!("Saved: {:#?}", path);
                    }
                }

                file_chooser.destroy();
            });
        }

        {
            let add_object: MenuItem = builder.get_object("Add Object").unwrap();
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            let set_objs = self.set_objs.clone();
            add_object.connect_activate(move |_| {
                let (paths, model) = set_grid.get_selection().get_selected_rows();
                let iter_opt = paths.get(0).map(|path| model.get_iter(path).unwrap());
                let object = SetObject::default();

                let idx = paths.get(0).map(|path| path.get_indices()[0] as usize).unwrap_or(0);
                set_objs.borrow_mut().0.insert(idx, object);

                let inserted_iter = set_list.insert_after(iter_opt.as_ref());
                set_list.set(&inserted_iter, &[0, 2, 3, 4, 5, 6, 7, 8, 9, 10], &[&"0000", &"0000", &"0000", &"0000", &"0", &"0", &"0", &"00000000", &"00000000", &"00000000"]);
            });
        }

        {
            let add_object: MenuItem = builder.get_object("Remove Object").unwrap();
            let set_list: ListStore = builder.get_object("Set Objects").unwrap();
            let set_grid: TreeView = builder.get_object("Set Grid").unwrap();
            let set_objs = self.set_objs.clone();
            add_object.connect_activate(move |_| {
                let (paths, model) = set_grid.get_selection().get_selected_rows();
                for path in paths {
                    let iter = model.get_iter(&path).unwrap();
                    set_list.remove(&iter);

                    let idx = path.get_indices()[0] as usize;
                    set_objs.borrow_mut().0.remove(idx);
                }
            });
        }
    }
}
