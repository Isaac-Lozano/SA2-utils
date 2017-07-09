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

use obj_table::OBJ_TABLE;

pub trait ColumnType: FromStr {
    fn update_column(&self, set_list: &ListStore, path: &TreePath);
    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath);
}

pub struct ObjectID(pub u16);

impl FromStr for ObjectID {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ObjectID(u16::from_str_radix(s, 16)?))
    }
}

impl ColumnType for ObjectID {
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Object ID: {:04X}", self.0);
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();

        let empty = "";
        let obj_name = OBJ_TABLE.get(&(13, self.0)).unwrap_or(&empty);
        set_list.set(&iter, &[0, 1], &[&text, &obj_name]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].object = Object(self.0);
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
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("X Rotation: {:04X}", self.0);
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[2], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].rotation.x = self.0;
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
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Y Rotation: {:04X}", self.0);
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[3], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].rotation.y = self.0;
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
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Z Rotation: {:04X}", self.0);
        let text = format!("{:04X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[4], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].rotation.z = self.0;
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
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("X Position: {}", self.0);
        let text = format!("{}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[5], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].position.x = self.0;
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
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Y Position: {}", self.0);
        let text = format!("{}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[6], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].position.y = self.0;
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
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Z Position: {}", self.0);
        let text = format!("{}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[7], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].position.z = self.0;
    }
}

pub struct Attribute1(pub u32);

impl FromStr for Attribute1 {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Attribute1(u32::from_str_radix(s, 16)?))
    }
}

impl ColumnType for Attribute1 {
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Attribute 1: {:08X}", self.0);
        let text = format!("{:08X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[8], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].attr1 = self.0;
    }
}

pub struct Attribute2(pub u32);

impl FromStr for Attribute2 {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Attribute2(u32::from_str_radix(s, 16)?))
    }
}

impl ColumnType for Attribute2 {
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Attribute 2: {:08X}", self.0);
        let text = format!("{:08X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[9], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].attr2 = self.0;
    }
}

pub struct Attribute3(pub u32);

impl FromStr for Attribute3 {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Attribute3(u32::from_str_radix(s, 16)?))
    }
}

impl ColumnType for Attribute3 {
    fn update_column(&self, set_list: &ListStore, path: &TreePath) {
        println!("Attribute 3: {:08X}", self.0);
        let text = format!("{:08X}", self.0);
        let iter = set_list.get_iter(&path).unwrap();
        set_list.set(&iter, &[10], &[&text]);
    }

    fn update_obj(&self, set_objs: &Rc<RefCell<SetFile>>, path: &TreePath) {
        let idx = path.get_indices()[0];
        set_objs.borrow_mut().0[idx as usize].attr3 = self.0;
    }
}
