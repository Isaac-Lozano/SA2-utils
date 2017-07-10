use std::str::FromStr;
use std::u16;
use std::u32;
use std::f32;
use std::num;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::{ListStore, TreePath};
use sa2_set::{SetFile, Object};

use obj_table::ObjectTable;

pub trait ColumnType: FromStr {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, obj_table: &Rc<RefCell<Option<ObjectTable>>>, level: u16);
    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize);
}

pub struct ObjectID(pub u16);

impl FromStr for ObjectID {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ObjectID(u16::from_str_radix(s, 16)?))
    }
}

impl ColumnType for ObjectID {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, obj_table: &Rc<RefCell<Option<ObjectTable>>>, level: u16) {
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();

        let empty = String::from("");
        let obj_table_borrow = obj_table.borrow();
        let obj_name = obj_table_borrow.as_ref().and_then(|ot| ot.lookup(level, self.0)).unwrap_or(&empty);
        set_list.set(&iter, &[1, 2], &[&text, &obj_name]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].object = Object(self.0);
    }
}

pub struct XRotation(pub u16);

impl FromStr for XRotation {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XRotation(u16::from_str_radix(s, 16)?))
    }
}

impl ColumnType for XRotation {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[3], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].rotation.x = self.0;
    }
}

pub struct YRotation(pub u16);

impl FromStr for YRotation {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(YRotation(u16::from_str_radix(s, 16)?))
    }
}

impl ColumnType for YRotation {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[4], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].rotation.y = self.0;
    }
}

pub struct ZRotation(pub u16);

impl FromStr for ZRotation {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ZRotation(u16::from_str_radix(s, 16)?))
    }
}

impl ColumnType for ZRotation {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[5], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].rotation.z = self.0;
    }
}

pub struct XPosition(pub f32);

impl FromStr for XPosition {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XPosition(f32::from_str(s)?))
    }
}

impl ColumnType for XPosition {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[6], &[&self.0]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].position.x = self.0;
    }
}

pub struct YPosition(pub f32);

impl FromStr for YPosition {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(YPosition(f32::from_str(s)?))
    }
}

impl ColumnType for YPosition {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[7], &[&self.0]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].position.y = self.0;
    }
}

pub struct ZPosition(pub f32);

impl FromStr for ZPosition {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ZPosition(f32::from_str(s)?))
    }
}

impl ColumnType for ZPosition {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[8], &[&self.0]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].position.z = self.0;
    }
}

pub struct Attribute1(pub f32);

impl FromStr for Attribute1 {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Attribute1(f32::from_str(s)?))
    }
}

impl ColumnType for Attribute1 {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[9], &[&self.0]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].attr1 = self.0;
    }
}

pub struct Attribute2(pub f32);

impl FromStr for Attribute2 {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Attribute2(f32::from_str(s)?))
    }
}

impl ColumnType for Attribute2 {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[10], &[&self.0]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].attr2 = self.0;
    }
}

pub struct Attribute3(pub f32);

impl FromStr for Attribute3 {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Attribute3(f32::from_str(s)?))
    }
}

impl ColumnType for Attribute3 {
    fn update_column(&self, set_list: &ListStore, path: &TreePath, _obj_table: &Rc<RefCell<Option<ObjectTable>>>, _level: u16) {
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[11], &[&self.0]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, idx: usize) {
        set_objs.borrow_mut().0[idx].attr3 = self.0;
    }
}
